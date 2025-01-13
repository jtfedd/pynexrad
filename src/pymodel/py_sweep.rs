use pyo3::prelude::*;

use crate::model::{sweep::Sweep, sweep_type::*};

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
    pub start_time: i64,
    #[pyo3(get)]
    pub end_time: i64,

    #[pyo3(get)]
    pub data: Vec<u8>,
}

impl PySweep {
    pub(crate) fn empty(start_time: i64, end_time: i64, elevation: f32) -> Self {
        Self {
            elevation,
            az_first: 0.0,
            az_step: 0.0,
            az_count: 0,
            range_first: 0.0,
            range_step: 0.0,
            range_count: 0,
            start_time: start_time,
            end_time: end_time,
            data: Vec::new(),
        }
    }

    pub(crate) fn new(sweep: &Sweep, data_type: SweepType) -> Self {
        let mut data: Vec<u8> = Vec::new();

        let (min, max) = match data_type {
            REFLECTIVITY => (-20.0, 80.0),
            VELOCITY => (-100.0, 100.0),
            _ => panic!("Unexpected product: {}", data_type),
        };

        let product = match data_type {
            REFLECTIVITY => sweep.reflectivity.as_ref().unwrap(),
            VELOCITY => sweep.velocity.as_ref().unwrap(),
            _ => panic!("Unexpected product {}", data_type),
        };

        // Find the first gate with data somewhere in one of the radials
        let mut first_gate = 0;
        let mut found_data = false;
        for gate in 0..product.gates {
            for radial in 0..product.radials {
                if !product.get_mask(radial, gate) {
                    // We have data!
                    found_data = true;
                    break;
                }
            }
            if found_data {
                break;
            }
            // We didn't find data in any radial for this gate
            // The first gate with data must be at least the next one
            first_gate = gate as i32 + 1;
        }

        // Find the last gate with data somewhere in one of the radials
        let mut last_gate = product.gates as i32 - 1;
        found_data = false;
        for gate in (0..product.gates).rev() {
            for radial in 0..product.radials {
                if !product.get_mask(radial, gate) {
                    // We have data!
                    found_data = true;
                    break;
                }
            }
            if found_data {
                break;
            }
            // We didn't find data in any radial for this gate
            // The last gate with data must be at least the one before
            last_gate = gate as i32 - 1;
        }

        if last_gate < first_gate {
            return PySweep::empty(
                sweep.start_time.timestamp(),
                sweep.end_time.timestamp(),
                sweep.elevation,
            );
        }

        for radial in 0..product.radials {
            data.push(0);
            for gate in (first_gate as usize)..((last_gate + 1) as usize) {
                if product.get_mask(radial, gate) {
                    data.push(0);
                } else {
                    let mut value = product.get_value(radial, gate);

                    value -= min;
                    value /= max - min;
                    value = f32::max(f32::min(value, 1.0), 0.0);
                    value *= 254.0;
                    let u_value = value.round() as u8;

                    data.push(u_value + 1);
                }
            }
            data.push(0);
        }

        let range_first = sweep.range_first + (first_gate as f32 * sweep.range_step);
        let range_count = last_gate - first_gate + 1;

        Self {
            elevation: sweep.elevation,
            az_first: sweep.az_first,
            az_step: sweep.az_step,
            az_count: sweep.az_count,
            range_first: range_first - sweep.range_step,
            range_step: sweep.range_step,
            range_count: range_count + 2,
            start_time: sweep.start_time.timestamp(),
            end_time: sweep.end_time.timestamp(),
            data,
        }
    }
}
