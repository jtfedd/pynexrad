use std::f32::consts::PI;

use crate::model::volume::Volume;
use crate::pymodel::py_sweep::PySweep;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Scan")]
pub struct PyScan {
    #[pyo3(get)]
    pub sweeps: Vec<PySweep>,
}

const MIN_SEPARATION: f32 = 0.1 * PI / 180.0;

impl PyScan {
    pub(crate) fn new(volume: &Volume, data_type: &str) -> Self {
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

        Self { sweeps }
    }
}
