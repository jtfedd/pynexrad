use crate::{model::volume::Volume, pymodel::py_scan::PyScan};
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
    pub(crate) fn new(reflectivity: Volume, velocity: Volume) -> Self {
        Self {
            reflectivity: PyScan::new(reflectivity),
            velocity: PyScan::new(velocity),
        }
    }
}
