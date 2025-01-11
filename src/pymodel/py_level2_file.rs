use crate::model::sweep_type::*;
use crate::model::volume::Volume;
use crate::pymodel::py_sweep::PySweep;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Level2File")]
pub struct PyLevel2File {
    #[pyo3(get)]
    pub reflectivity: Vec<PySweep>,
    #[pyo3(get)]
    pub velocity: Vec<PySweep>,
}

fn collect_sweeps(volume: &Volume, data_type: SweepType) -> Vec<PySweep> {
    let mut sweeps: Vec<PySweep> = Vec::new();

    for sweep in volume.sweeps.iter() {
        if !sweep.has_product(data_type) {
            continue;
        }

        if (sweep.sweep_type & data_type) == 0 {
            continue;
        }

        sweeps.push(PySweep::new(sweep, data_type))
    }

    sweeps.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    return sweeps;
}

impl PyLevel2File {
    pub(crate) fn new(volume: Volume) -> Self {
        Self {
            reflectivity: collect_sweeps(&volume, REFLECTIVITY),
            velocity: collect_sweeps(&volume, VELOCITY),
        }
    }
}
