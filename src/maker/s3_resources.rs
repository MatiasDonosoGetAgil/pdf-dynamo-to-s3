use aws_config::load_from_env;
use aws_sdk_s3::Client;

pub struct S3Uploader {
    s3_client: Client,
    bucket_name: String,
}

impl S3Uploader {
    pub async fn new(region: &str, bucket_name: &str) -> Result<Self, Box<dyn Error>> {
        // Crear un proveedor de región
        let region_provider = RegionProviderChain::first_try(Some(Region::new(region.to_string())))
            .or_default_provider();

        // Cargar la configuración de AWS con la región especificada
        let config = aws_config::from_env().region(region_provider).load().await;

        // Crear el cliente de S3
        let client = Client::new(&config);

        // Construir y devolver el S3Uploader
        Ok(S3Uploader {
            s3_client: client,
            bucket_name: bucket_name.to_string(),
        })
    }
}
