use pyo3::prelude::*;
use tokio;
use chrono::NaiveDate;

use nexrad::file::is_compressed;
use nexrad::decode::decode_file;
use nexrad::decompress::decompress_file;
use nexrad::download::download_file;
use nexrad::download::list_files;
use nexrad::file::FileMetadata;

use crate::model::PyLevel2File;
use crate::convert::convert_nexrad_file;

/// Creates a date from year, month, and day
fn create_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("date is valid")
}

/// Downloads and decodes a nexrad file
#[pyfunction]
fn download_nexrad_file(
    site: String, 
    year: i32, 
    month: u32, 
    day: u32, 
    identifier: String,
) -> PyLevel2File {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut bytes = rt.block_on(async {
        download_file(&FileMetadata::new(
            site,
            create_date(year, month, day),
            identifier,
        )).await
    }).expect("Should download without error");

    if is_compressed(&bytes) {
        bytes = decompress_file(&bytes).expect("decompresses file");
    }

    let decoded = decode_file(&bytes).expect("decodes file");

    convert_nexrad_file(&decoded)
}

/// Lists records from a particular site and date
#[pyfunction]
fn list_records(
    site: String,
    year: i32,
    month: u32,
    day: u32,
) -> Vec<String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let files = rt.block_on(async {
        list_files(&site, &create_date(year, month, day)).await
    }).expect("Should download without error");

    let keys = files
        .iter()
        .map(|file| {
            file.identifier().clone()
        })
        .collect();

    keys
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(download_nexrad_file, m)?)?;
    m.add_function(wrap_pyfunction!(list_records, m)?)?;
    m.add_class::<PyLevel2File>()?;
    Ok(())
}
