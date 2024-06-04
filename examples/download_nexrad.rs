use chrono::NaiveDate;
use nexrad::download::download_file;
use nexrad::file::is_compressed;
use nexrad::file::FileMetadata;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let date = NaiveDate::from_ymd_opt(2024, 5, 21).expect("is a valid date");

    let meta = FileMetadata::new(
        "KDMX".to_string(),
        date,
        "KDMX20240521_215236_V06".to_string(),
    );

    println!("Downloading file \"{}\"...", meta.identifier());
    let downloaded_file = download_file(&meta).await.expect("is downloaded");

    println!("Data file size (bytes): {}", downloaded_file.len());

    let compressed = is_compressed(downloaded_file.as_slice());
    println!("File data is compressed: {}", compressed);

    let mut file = std::fs::File::create(meta.identifier()).expect("create file");
    file.write_all(downloaded_file.as_slice())
        .expect("write file");

    return Ok(());
}
