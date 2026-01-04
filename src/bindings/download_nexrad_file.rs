use nexrad_data::aws::archive::{download_file, Identifier};
use pyo3::{pyfunction, PyResult, Python};

use super::convert::convert_nexrad_file;
use crate::pymodel::py_level2_file::PyLevel2File;

#[pyfunction]
pub fn download_nexrad_file(py: Python, identifier: String) -> PyResult<PyLevel2File> {
    let result = py.allow_threads(move || download_nexrad_file_impl(identifier));

    Ok(result)
}

/// Downloads and decodes a nexrad file
fn download_nexrad_file_impl(identifier: String) -> PyLevel2File {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let f = rt
        .block_on(async { download_file(Identifier::new(identifier)).await })
        .expect("Should download without error");

    convert_nexrad_file(f.records())
}
