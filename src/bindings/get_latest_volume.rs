use nexrad_data::aws::realtime::get_latest_volume as nexrad_latest_volume;
use pyo3::{pyfunction, PyResult, Python};

#[pyfunction]
pub fn get_latest_volume(py: Python, site: String) -> PyResult<i32> {
    let result = py.allow_threads(move || get_latest_volume_impl(site));

    Ok(result)
}

fn get_latest_volume_impl(site: String) -> i32 {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let latest_volume_result = rt
        .block_on(async { nexrad_latest_volume(&site).await })
        .expect("Should find latest volume");

    latest_volume_result.volume.unwrap().as_number() as i32
}
