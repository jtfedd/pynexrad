use tokio;
use pyo3::prelude::*;
use nexrad::decode::decode_file;
use nexrad::download::download_file;
use crate::decompress::decompress;
use crate::convert::convert_nexrad_file;
use crate::file::PyFileMetadata;
use crate::model::PyLevel2File;

/// Decodes a nexrad file given bytes
#[pyfunction]
fn parse_nexrad_file(bytes: Vec<u8>) -> PyLevel2File {
    println!("Decompressing");
    let decompressed = decompress(bytes);

    println!("Decoding");
    let decoded = decode_file(&decompressed).expect("decodes file");

    println!("Converting");
    convert_nexrad_file(&decoded)
}

/// Downloads and decodes a nexrad file
#[pyfunction]
fn download_nexrad_file(file_meta: PyFileMetadata) -> PyLevel2File {
    println!("Downloading");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let bytes = rt.block_on(async {
        download_file(&file_meta.to_nexrad_file_metadata()).await
    }).expect("Should download without error");

    parse_nexrad_file(bytes)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_nexrad_file, m)?)?;
    m.add_function(wrap_pyfunction!(download_nexrad_file, m)?)?;
    m.add_class::<PyLevel2File>()?;
    m.add_class::<PyFileMetadata>()?;
    Ok(())
}
