use crate::{model::volume::Volume, pymodel::py_sweep::PySweep};
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Scan")]
pub struct PyScan {
    #[pyo3(get)]
    pub sweeps: Vec<PySweep>,
}

impl PyScan {
    pub(crate) fn new(volume: Volume) -> Self {
        let mut sweeps: Vec<PySweep> = Vec::new();

        sweeps.push(PySweep::empty(0.0));

        for sweep in volume.sweeps {
            sweeps.push(PySweep::new(sweep, volume.data_type.as_str()))
        }

        sweeps.sort_by(|a, b| a.elevation.partial_cmp(&b.elevation).unwrap());

        if sweeps.len() > 1 {
            let prev_diff = sweeps[sweeps.len() - 1].elevation - sweeps[sweeps.len() - 2].elevation;
            sweeps.push(PySweep::empty(
                sweeps[sweeps.len() - 1].elevation + prev_diff,
            ))
        }

        Self { sweeps }
    }
}
