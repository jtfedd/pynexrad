use pyo3::prelude::*;

use crate::bindings::download_nexrad_file::download_nexrad_file;
use crate::bindings::list_records::list_records;
use crate::pymodel::py_level2_file::PyLevel2File;
use crate::pymodel::py_sweep::PySweep;

#[pymodule]
fn pynexrad(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_records, m)?)?;
    m.add_function(wrap_pyfunction!(download_nexrad_file, m)?)?;

    m.add_class::<PyLevel2File>()?;
    m.add_class::<PySweep>()?;
    Ok(())
}
