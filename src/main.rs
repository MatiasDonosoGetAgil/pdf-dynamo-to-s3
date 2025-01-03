use lambda_http::{service_fn, Body, Error, Request, RequestPayloadExt as _, Response};
use s3::S3Uploader;
use serde_json::json;
use tracing::Level;
use tracing_subscriber;
mod maker;
mod s3;
use maker::pdf;
use maker::IOrder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Inicializas el logger con tracing + tracing-subscriber
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        // Si quieres usar JSON para tus logs:
        .json()
        .init();

    let func = service_fn(|event: Request| async move {
        // Ejemplo: parsear el body como MyPayload
        let body: Option<IOrder> = event.payload::<IOrder>()?;
        if let Some(payload) = body {
            // let data = maker::test_data();
            let pdf_as_vec = pdf(&payload).unwrap();
            let mut s3 = S3Uploader::new().await;
            let normal = s3
                .upload_file(pdf_as_vec, payload.IdPedido, true)
                .await
                .unwrap();

            let respuesta = json!(
                { "mensaje": normal }
            );
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
