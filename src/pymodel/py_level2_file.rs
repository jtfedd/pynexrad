use crate::model::volume::Volume;
use crate::pymodel::py_sweep::PySweep;
use pyo3::prelude::*;

use std::f32::consts::PI;

const MIN_SEPARATION: f32 = 0.1 * PI / 180.0;

#[derive(Clone)]
#[pyclass(name = "Level2File")]
pub struct PyLevel2File {
    #[pyo3(get)]
    pub reflectivity: Vec<PySweep>,
    #[pyo3(get)]
    pub velocity: Vec<PySweep>,
}

fn collect_sweeps(volume: &Volume, data_type: &str) -> Vec<PySweep> {
    let mut sweeps: Vec<PySweep> = Vec::new();

    sweeps.push(PySweep::empty(0.0));

    for sweep in volume.sweeps.iter() {
        if !sweep.has_product(data_type) {
            continue;
        }

        // Sometimes there are overlapping sweeps on the same elevation.
        // For now discard duplicates.
        let mut elevation_exists = false;
        for m in sweeps.iter() {
            if (sweep.elevation - m.elevation).abs() < MIN_SEPARATION {
                elevation_exists = true;
                break;
            }
        }
        if elevation_exists {
            continue;
        }

        sweeps.push(PySweep::new(sweep, data_type))
    }

    sweeps.sort_by(|a, b| a.elevation.partial_cmp(&b.elevation).unwrap());

    if sweeps.len() > 1 {
        let prev_diff = sweeps[sweeps.len() - 1].elevation - sweeps[sweeps.len() - 2].elevation;
        sweeps.push(PySweep::empty(
            sweeps[sweeps.len() - 1].elevation + prev_diff,
        ))
    }

    sweeps
}

impl PyLevel2File {
    pub(crate) fn new(volume: Volume) -> Self {
        Self {
            reflectivity: collect_sweeps(&volume, "ref"),
            velocity: collect_sweeps(&volume, "vel"),
        }
    }
}
