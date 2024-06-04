use std::f32::consts::PI;

use crate::model::sweep::Sweep;
use nexrad::model::{DataFile, Message31};

const MIN_SEPARATION: f32 = 0.1 * PI / 180.0;

pub struct Volume {
    pub data_type: String,
    pub sweeps: Vec<Sweep>,
}

impl Volume {
    pub(crate) fn new(file: &DataFile, data_type: &str) -> Self {
        let mut result_sweeps: Vec<Sweep> = Vec::new();

        let mut sweeps: Vec<_> = file.elevation_scans().iter().collect();
        sweeps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for sweep in sweeps {
            if !validate_sweep(sweep.1, data_type) {
                continue;
            }

            let result_sweep = Sweep::new(sweep.1, data_type);

            // Sometimes there are overlapping sweeps on the same elevation.
            // For now discard duplicates.
            let mut elevation_exists = false;
            for m in result_sweeps.iter() {
                if (result_sweep.elevation - m.elevation).abs() < MIN_SEPARATION {
                    elevation_exists = true;
                    break;
                }
            }
            if elevation_exists {
                continue;
            }

            result_sweeps.push(result_sweep);
        }

        Self {
            sweeps: result_sweeps,
            data_type: String::from(data_type),
        }
    }
}

fn validate_sweep(radials: &Vec<Message31>, data_type: &str) -> bool {
    for radial in radials {
        let data_moment = match data_type {
            "ref" => radial.reflectivity_data(),
            "vel" => radial.velocity_data(),
            _ => panic!("Unexpected product: {}", data_type),
        };

        if data_moment.is_none() {
            return false;
        }
    }

    true
}
