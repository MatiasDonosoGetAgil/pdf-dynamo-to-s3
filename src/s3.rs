// use aws_config::ConfigLoader;
use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::Client;

pub struct S3Uploader {
    pub s3_client: Client,
    pub aws_bucket_name: String,
}

impl S3Uploader {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region("us-east-2")
            .load()
            .await;
        let s3_client = aws_sdk_s3::Client::new(&config);
        let aws_bucket_name = String::from("agil");
        Self {
            s3_client,
            aws_bucket_name,
        }
    }

    pub async fn upload_file(
        &mut self,
        buffer: Vec<u8>,
        key: &str,
        pdf: bool,
    ) -> Result<String, String> {
        if buffer.is_empty() {
            return Err("Buffer está vacío".to_string());
        }

        let (content_type, extension) = if pdf {
            ("application/pdf", "pdf")
        } else {
            ("application/octet-stream", "txt")
        };

        // Usar timestamp para garantizar unicidad
        let timestamp = chrono::Utc::now().timestamp();
        let object_key = format!("{}-{}.{}", key, timestamp, extension);

        // Crear ByteStream directamente sin clonar el buffer
        let byte_stream = aws_sdk_s3::primitives::ByteStream::from(buffer);

        // Construir la URL antes de la subida
        let url = format!("https://barbecue.getagil.com/{}", object_key);

        match self
            .s3_client
            .put_object()
            .bucket(&self.aws_bucket_name)
            .key(&object_key)
            .body(byte_stream)
            .content_type(content_type)
            .acl(ObjectCannedAcl::PublicRead)
            .send()
            .await
        {
            Ok(_) => Ok(url),
            Err(err) => Err(format!("Error al subir el archivo a S3: {}", err)),
        }
    }
}
