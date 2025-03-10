use nexrad_data::aws::archive::list_files;
use pyo3::{pyfunction, PyResult, Python};

use super::util::create_date;

#[pyfunction]
pub fn list_records(
    py: Python,
    site: String,
    year: i32,
    month: u32,
    day: u32,
) -> PyResult<Vec<String>> {
    let result = py.allow_threads(move || list_records_impl(site, year, month, day));

    Ok(result)
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
