#[macro_use]
extern crate lazy_static;

use lambda_http::{service_fn, Body, Error, Request, RequestPayloadExt as _, Response};
use s3::S3Uploader;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::Level;
use tracing_subscriber;
mod maker;
mod s3;
use maker::{get_ticket_kitchen, get_ticket_pdf, IOrder, Payload};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Inicializas el logger con tracing + tracing-subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        // Si quieres usar JSON para tus logs:
        .json()
        .init();

    let func = service_fn(|event: Request| async move {
        let body: Option<Payload> = event.payload::<Payload>()?;
        if let Some(payload) = body {
            // Obtener el id_pedido antes de mover payload.order
            let id_pedido = payload.order.id_pedido.to_string();

            // Preparar los nombres de archivo
            let name = format!("tickets/pos/{}", id_pedido);
            let copy_name = format!("tickets/pos/{}-copy", id_pedido);
            let kitchen_name = format!("tickets/kitchen/{}", id_pedido);
            let kitchen_copy_name = format!("tickets/kitchen/{}-copy", id_pedido);

            // Ejecutar la generación del PDF y la inicialización de S3 en paralelo
            let orders: (IOrder, IOrder, IOrder, IOrder) = (
                payload.order.clone(),
                payload.order.clone(),
                payload.order.clone(),
                payload.order.clone(),
            );
            let (pdf_result, pdf_copy_result, txt_result, txt_copy_result, s3) = tokio::join!(
                tokio::task::spawn_blocking(move || get_ticket_pdf(&orders.0, false)),
                tokio::task::spawn_blocking(move || get_ticket_pdf(&orders.1, true)),
                tokio::task::spawn_blocking(move || get_ticket_kitchen(&orders.2, true)),
                tokio::task::spawn_blocking(move || get_ticket_kitchen(&orders.3, true)),
                S3Uploader::new()
            );

            // Extraer los resultados de los tickets
            let pdf_normal_vec = pdf_result.unwrap().unwrap();
            let pdf_copy_vec = pdf_copy_result.unwrap().unwrap();
            let txt_normal_vec = txt_result.unwrap().unwrap();
            let txt_copy_vec = txt_copy_result.unwrap().unwrap();

            // Crear el uploader compartido
            let s3_ref = Arc::new(Mutex::new(s3));

            // Crear clones de Arc para cada tarea
            let s3_copy = Arc::clone(&s3_ref);
            let s3_kitchen = Arc::clone(&s3_ref);
            let s3_kitchen_copy = Arc::clone(&s3_ref);

            // Subir archivos en paralelo
            let uploads = tokio::join!(
                async {
                    let mut s3 = s3_ref.lock().await;
                    s3.upload_file(pdf_normal_vec, &name, true).await
                },
                async {
                    let mut s3 = s3_copy.lock().await;
                    s3.upload_file(pdf_copy_vec, &copy_name, true).await
                },
                async {
                    let mut s3 = s3_kitchen.lock().await;
                    s3.upload_file(txt_normal_vec, &kitchen_name, false).await
                },
                async {
                    let mut s3 = s3_kitchen_copy.lock().await;
                    s3.upload_file(txt_copy_vec, &kitchen_copy_name, false)
                        .await
                }
            );

            let (pdf_normal, pdf_copy, txt_normal, txt_copy) = uploads;

            let respuesta = json!({
                "ticketPOS": {
                    "ticket": pdf_normal.unwrap(),
                    "copy": pdf_copy.unwrap()
                },
                "ticketKitchen": {
                    "ticket": txt_normal.unwrap(),
                    "copy": txt_copy.unwrap()
                }
            });
            Ok::<_, Error>(Response::new(Body::Text(respuesta.to_string())))
        } else {
            // Manejo de error...
            let respuesta = json!({ "error": "No se pudo parsear el payload" });
            Ok::<_, Error>(Response::new(Body::Text(respuesta.to_string())))
        }
    });

    lambda_http::run(func).await?;
    Ok(())
}
