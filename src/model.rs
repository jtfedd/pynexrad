use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Level2File")]
pub struct PyLevel2File {
    #[pyo3(get)]
    reflectivity: PyScan,
    #[pyo3(get)]
    velocity: PyScan,
}

impl PyLevel2File {
    pub(crate) fn new(reflectivity: PyScan, velocity: PyScan) -> Self {
        Self {
            reflectivity, 
            velocity,
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "Scan")]
pub struct PyScan {
    #[pyo3(get)]
    meta: Vec<PySweep>,
    #[pyo3(get)]
    data: Vec<f32>,
}

impl PyScan {
    pub(crate) fn empty() -> Self {
        let meta = vec![PySweep::empty()];
        let data: Vec<f32> = vec![];

        return Self {
            meta,
            data,
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "Sweep")]
pub struct PySweep {
    #[pyo3(get)]
    elevation: f32,

    #[pyo3(get)]
    az_first: f32,
    #[pyo3(get)]
    az_step: f32,
    #[pyo3(get)]
    az_count: i32,

    #[pyo3(get)]
    range_first: f32,
    #[pyo3(get)]
    range_step: f32,
    #[pyo3(get)]
    range_count: i32,

    #[pyo3(get)]
    offset: i32,
}

impl PySweep {
    pub(crate) fn empty() -> Self {
        Self {
            elevation: 0.0,
            az_first: 0.0,
            az_step: 0.0,
            az_count: 0,
            range_first: 0.0,
            range_step: 0.0,
            range_count: 0,
            offset: 0,
        }
    }
}
