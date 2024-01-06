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
        Self { reflectivity, velocity }
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
    pub(crate) fn new(meta: Vec<PySweep>, data: Vec<f32>) -> Self {
        Self { meta, data }
    }
}

#[derive(Clone)]
#[pyclass(name = "Sweep")]
pub struct PySweep {
    #[pyo3(get)]
    pub elevation: f32,

    #[pyo3(get)]
    pub az_first: f32,
    #[pyo3(get)]
    pub az_step: f32,
    #[pyo3(get)]
    pub az_count: i32,

    #[pyo3(get)]
    pub range_first: f32,
    #[pyo3(get)]
    pub range_step: f32,
    #[pyo3(get)]
    pub range_count: i32,

    #[pyo3(get)]
    pub offset: i32,
}

impl PySweep {
    pub(crate) fn empty(elevation: f32) -> Self {
        Self {
            elevation,
            az_first: 0.0,
            az_step: 0.0,
            az_count: 0,
            range_first: 0.0,
            range_step: 0.0,
            range_count: 0,
            offset: 0,
        }
    }

    pub(crate) fn new(
        elevation: f32,
        az_first:f32,
        az_step: f32,
        az_count: i32,
        range_first: f32,
        range_step: f32,
        range_count: i32,
        offset: i32,
    ) -> Self {
        Self {
            elevation,
            az_first,
            az_step,
            az_count,
            range_first,
            range_step,
            range_count,
            offset
        }
    }
}
