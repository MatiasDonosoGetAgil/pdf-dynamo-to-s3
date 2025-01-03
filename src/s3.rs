use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::Client;

pub struct S3Uploader {
    pub s3_client: Client,
    pub aws_bucket_name: String,
    pub name_id: u64,
}

impl S3Uploader {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        let s3_client = aws_sdk_s3::Client::new(&config);
        let aws_bucket_name = String::from("agil");
        Self {
            s3_client,
            aws_bucket_name,
            name_id: 1, // Inicialización de `name_id`
        }
    }

    pub async fn upload_file(
        &mut self,
        buffer: Vec<u8>,
        key: &str,
        file_type: &str,
    ) -> Result<String, String> {
        if buffer.is_empty() {
            eprintln!("Buffer is undefined or empty");
            return Err("Buffer is undefined or empty".to_string());
        }

        let content_type = match file_type {
            "pdf" => "application/pdf",
            _ => "application/octet-stream",
        };

        let object_key = format!("{}.{}", key, file_type);

        // Crear la carga del archivo hacia S3
        let byte_stream: aws_sdk_s3::primitives::ByteStream =
            aws_sdk_s3::primitives::ByteStream::from(buffer.clone());

        match self
            .s3_client
            .put_object()
            .bucket(&self.aws_bucket_name)
            .key(&object_key)
            .body(byte_stream)
            .content_type(content_type)
            .acl("public-read".into())
            .send()
            .await
        {
            Ok(response) => {
                println!("Archivo subido exitosamente a S3: {:?}", response);
                Ok(format!(
                    "https://{}.s3.amazonaws.com/{}",
                    self.aws_bucket_name, object_key
                ))
            }
            Err(err) => {
                eprintln!("Error al subir el archivo a S3: {}", err);
                Err("Error al subir el archivo a S3".to_string())
            }
        }
    }
}
