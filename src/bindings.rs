use tokio;
use pyo3::prelude::*;
use nexrad::decode::decode_file;
use nexrad::download::download_file;
use nexrad::file::FileMetadata;
use chrono::NaiveDate;
use crate::decompress::decompress;
use crate::convert::convert_nexrad_file;
use crate::model::PyLevel2File;

/// Decodes a nexrad file given bytes
#[pyfunction]
fn parse_nexrad_file(bytes: Vec<u8>) -> PyLevel2File {
    let decompressed = decompress(bytes);
    let decoded = decode_file(&decompressed).expect("decodes file");
    convert_nexrad_file(&decoded)
}

/// Downloads and decodes a nexrad file
#[pyfunction]
fn download_nexrad_file(
    site: &str, 
    year: i32, 
    month: u32, 
    day: u32, 
    identifier: &str,
) -> PyLevel2File {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let bytes = rt.block_on(async {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("date is valid");
        download_file(&FileMetadata::new(site.to_string(), date, identifier.to_string())).await
    }).expect("Should download without error");

    parse_nexrad_file(bytes)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_nexrad_file, m)?)?;
    m.add_function(wrap_pyfunction!(download_nexrad_file, m)?)?;
    m.add_class::<PyLevel2File>()?;
    Ok(())
}
