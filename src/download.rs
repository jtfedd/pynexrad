use std::error::Error;

use aws_sdk_s3::{config::Region, types::Object, Client, Config};

const REGION: &str = "us-east-1";
const BUCKET: &str = "noaa-nexrad-level2";

/// Download a data file specified by its metadata. Returns the downloaded file's encoded contents
/// which may then need to be decompressed and decoded.
pub async fn download_file(key: &str) -> Vec<u8> {
    // Download the object from S3
    download_object(&get_client().await, BUCKET, &key).await
}

/// Downloads an object from S3 and returns only its contents. This will only work for
/// unauthenticated requests (requests are unsigned).
async fn download_object(client: &Client, bucket: &str, key: &str) -> Result<Vec<u8>, dynError> {
    let operation = client.get_object().bucket(bucket).key(key);

    let response = operation.send().await?;
    let bytes = response.body.collect().await?;

    bytes.to_vec()
}

/// Lists objects from a S3 bucket with the specified prefix. This will only work for
/// unauthenticated requests (requests are unsigned).
async fn list_objects(client: &Client, bucket: &str, prefix: &str) -> Option<Vec<Object>> {
    let operation = client.list_objects_v2().bucket(bucket).prefix(prefix);

    let response = operation.send().await?;
    response.contents().map(|objects| objects.to_vec())
}

/// Creates a new S3 client for a predetermined region.
async fn get_client() -> Client {
    Client::from_conf(
        Config::builder()
            .region(Region::from_static(REGION))
            .build(),
    )
}