use nexrad_data::aws::archive::{download_file, Identifier};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let name = "KDMX20240521_224629_V06";
    let id = Identifier::new(name.to_string());

    println!("Downloading file \"{}\"...", name);
    let downloaded_file = download_file(id).await.expect("is downloaded");
    println!("Data file size (bytes): {}", downloaded_file.data().len());

    let mut file = std::fs::File::create(name).expect("create file");
    file.write_all(downloaded_file.data()).expect("write file");

    return Ok(());
}
