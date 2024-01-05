
use pyo3::prelude::*;
use nexrad::decode::decode_file;
use crate::decompress::decompress;
use crate::convert::convert_nexrad_file;
use crate::model::PyLevel2File;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn parse_nexrad_file(bytes: Vec<u8>) -> PyLevel2File {
    let decompressed = decompress(bytes);
    let decoded = decode_file(&decompressed).expect("decodes file");
    *convert_nexrad_file(&decoded)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_nexrad_file, m)?)?;
    m.add_class::<PyLevel2File>()?;
    Ok(())
}
