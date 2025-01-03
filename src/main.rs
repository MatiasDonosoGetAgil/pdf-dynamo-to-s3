use lambda_http::{service_fn, Body, Error, Request, RequestPayloadExt as _, Response};
use s3::S3Uploader;
use serde_json::json;
use tracing::Level;
use tracing_subscriber;
mod maker;
mod s3;
use maker::pdf;
use maker::Payload;

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

            // Ejecutar la generación del PDF y la inicialización de S3 en paralelo
            let (pdf_result, mut s3) = tokio::join!(
                tokio::task::spawn_blocking(move || pdf(&payload.order)),
                S3Uploader::new()
            );

            // Manejar los resultados
            let pdf_as_vec = pdf_result.unwrap().unwrap();
            let normal = s3
                .upload_file(pdf_as_vec, &id_pedido.as_str(), true)
                .await
                .unwrap();

            let respuesta = json!({ "mensaje": normal });
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
