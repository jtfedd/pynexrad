
use pyo3::prelude::*;
use nexrad::decompress::decompress_file;
use nexrad::decode::decode_file;
use nexrad::file::is_compressed;
use nexrad::model::DataFile;
use crate::level2file::PyLevel2File;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn parse_nexrad_file(file: Vec<u8>) -> PyLevel2File {
    // if is_compressed(file.as_slice()) {
    //     file = decompress_file(&file).expect("decompresses file");
    // }

    // let decoded = decode_file(&file).expect("decodes file");

    PyLevel2File::new(file[0] as f64, file[1] as f64)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_nexrad_file, m)?)?;
    m.add_class::<PyLevel2File>()?;
    Ok(())
}
