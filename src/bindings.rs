use nexrad_data::aws::archive::{download_file, list_files, Identifier};
use pyo3::prelude::*;

use crate::convert::convert_nexrad_file;
use crate::pymodel::py_level2_file::PyLevel2File;
use crate::pymodel::py_sweep::PySweep;
use crate::util::create_date;

/// Downloads and decodes a nexrad file
fn download_nexrad_file_impl(identifier: String) -> PyLevel2File {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let f = rt
        .block_on(async { download_file(Identifier::new(identifier)).await })
        .expect("Should download without error");

    convert_nexrad_file(&f)
}

/// Lists records from a particular site and date
fn list_records_impl(site: String, year: i32, month: u32, day: u32) -> Vec<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let files = rt
        .block_on(async { list_files(&site, &create_date(year, month, day)).await })
        .expect("Should download without error");

    let keys = files.iter().map(|id| String::from(id.name())).collect();

    keys
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn download_nexrad_file(py: Python, identifier: String) -> PyResult<PyLevel2File> {
        let result = py.allow_threads(move || download_nexrad_file_impl(identifier));

        Ok(result)
    }

    #[pyfn(m)]
    fn list_records(
        py: Python,
        site: String,
        year: i32,
        month: u32,
        day: u32,
    ) -> PyResult<Vec<String>> {
        let result = py.allow_threads(move || list_records_impl(site, year, month, day));

        Ok(result)
    }

    // m.add_function(wrap_pyfunction!(download_nexrad_file, m)?)?;
    m.add_function(wrap_pyfunction!(list_records, m)?)?;
    m.add_class::<PyLevel2File>()?;
    m.add_class::<PySweep>()?;
    Ok(())
}
