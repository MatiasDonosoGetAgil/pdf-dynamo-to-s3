use lambda_http::{service_fn, Body, Error, Request, RequestPayloadExt as _, Response};
use s3::S3Uploader;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::Level;
use tracing_subscriber;
mod maker;
mod s3;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct MyPayload {
    // tus campos...
    texto: String,
}

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
        let body = event.payload::<MyPayload>()?;

        if let Some(payload) = body {
            let texto = payload.texto;
            println!("texto: {}", texto);

            let data = maker::test_data();
            let (vec_1, vec_2) = data.await;
            let mut s3 = S3Uploader::new().await;

            println!(
                "url 1: {}",
                s3.upload_file(vec_1, "test", "pdf").await.unwrap()
            );
            println!(
                "url 2: {}",
                s3.upload_file(vec_2, "test", "pdf").await.unwrap()
            );

            let respuesta = json!({ "mensaje": texto });
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
