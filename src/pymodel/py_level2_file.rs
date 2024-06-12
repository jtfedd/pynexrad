use crate::model::volume::Volume;
use crate::pymodel::py_scan::PyScan;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Level2File")]
pub struct PyLevel2File {
    #[pyo3(get)]
    pub reflectivity: PyScan,
    #[pyo3(get)]
    pub velocity: PyScan,
}

impl PyLevel2File {
    pub(crate) fn new(volume: Volume) -> Self {
        Self {
            reflectivity: PyScan::new(&volume, "ref"),
            velocity: PyScan::new(&volume, "vel"),
        }
    }
}
