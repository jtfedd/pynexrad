use pyo3::prelude::*;

use crate::model::sweep::Sweep;

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
    pub data: Vec<f32>,
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
            data: Vec::new(),
        }
    }

    pub(crate) fn new(sweep: Sweep, data_type: &str) -> Self {
        let mut data: Vec<f32> = Vec::new();

        let (min, max) = match data_type {
            "ref" => (-20.0, 80.0),
            "vel" => (-100.0, 100.0),
            _ => panic!("Unexpected product: {}", data_type),
        };

        for radial in 0..sweep.data.radials {
            data.push(-1.0);
            for gate in 0..sweep.data.gates {
                if sweep.data.get_mask(radial, gate) {
                    data.push(-1.0);
                } else {
                    let mut value = sweep.data.get_value(radial, gate);

                    value -= min;
                    value /= max - min;

                    value = f32::max(f32::min(value, 1.0), 0.0);

                    data.push(value);
                }
            }
            data.push(-1.0);
        }

        Self {
            elevation: sweep.elevation,
            az_first: sweep.az_first,
            az_step: sweep.az_step,
            az_count: sweep.az_count,
            range_first: sweep.range_first - sweep.range_step,
            range_step: sweep.range_step,
            range_count: sweep.range_count + 2,
            data,
        }
    }
}
