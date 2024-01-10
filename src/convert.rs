use std::f32::consts::PI;

use nexrad::model::DataFile;
use nexrad::model::Message31;
use crate::model::PyLevel2File;
use crate::model::PyScan;
use crate::model::PySweep;

const MIN_SEPARATION: f32 = 0.1 * PI / 180.0;

pub fn convert_nexrad_file<'a>(file: &'a DataFile) -> PyLevel2File {
    let reflectivity = extract_volume(file, "ref", -20.0, 80.0);
    let velocity = extract_volume(file, "vel", -100.0, 100.0);

    return PyLevel2File::new(reflectivity, velocity)
}

fn extract_volume(file: &DataFile, data_type: &str, min: f32, max: f32) -> PyScan {
    let mut data: Vec<f32> = Vec::new();
    let mut meta: Vec<PySweep> = Vec::new();

    meta.push(PySweep::empty(0.0));

    let mut sweeps: Vec<_> = file.elevation_scans().iter().collect();
    sweeps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut offset = 0;

    for sweep in sweeps {
        // Not every sweep has every data type.
        if !validate_sweep(sweep.1, data_type) {
            continue;
        }

        let mut radials: Vec<_> = sweep.1.iter().collect();

        let sample_data_moment = match data_type {
            "ref" => radials[0].reflectivity_data().unwrap(),
            "vel" => radials[0].velocity_data().unwrap(),
            _ => panic!("Unexpected product: {}", data_type),
        };

        // Find the approximate elevation of this sweep
        let mut elevation_avg = 0.0 as f32;
        for radial in radials.iter() {
            elevation_avg += radial.header().elev() as f32;
        }
        elevation_avg /= radials.len() as f32;
        elevation_avg *= PI / 180.0;

        // Sometimes there are overlapping sweeps on the same elevation.
        // For now discard duplicates.
        let mut elevation_exists = false;
        for m in meta.iter() {
            if (elevation_avg - m.elevation).abs() < MIN_SEPARATION {
                elevation_exists = true;
                break;
            }
        }
        if elevation_exists {
            continue;
        }

        // Now we know that we have a sweep for the correct data type at an elevation
        // we have not yet encountered. Add the data to the array and add a meta entry.
        radials.sort_by(|a, b| a.header().azm().partial_cmp(&b.header().azm()).unwrap());
        let rad_hdr = radials[0].header();
        let az_first = (rad_hdr.azm_indexing_mode() as f32 / 100.0) * PI / 180.0;
        let az_count = radials.len() as i32;
        let az_step = if rad_hdr.azm_res() == 1 { 0.5 * PI / 180.0 } else { PI / 180.0 };

        let range_step = sample_data_moment.data().data_moment_range_sample_interval() as f32 / 1000.0;
        let range_first = (sample_data_moment.data().data_moment_range() as f32 / 1000.0) - range_step;
        let range_count = sample_data_moment.data().number_data_moment_gates() as i32 + 2;

        let sweep_meta = PySweep::new(
            elevation_avg,
            az_first,
            az_step,
            az_count,
            range_first,
            range_step,
            range_count,
            offset,
        );

        meta.push(sweep_meta);

        for radial in radials {
            let data_moment = match data_type {
                "ref" => radial.reflectivity_data().unwrap(),
                "vel" => radial.velocity_data().unwrap(),
                _ => panic!("Unexpected product: {}", data_type),
            };

            let mut raw_gates: Vec<u16> =
            vec![0; data_moment.data().number_data_moment_gates() as usize];

            assert_eq!(data_moment.data().data_word_size(), 8);
            for (i, v) in data_moment.moment_data().iter().enumerate() {
                raw_gates[i] = *v as u16;
            }

            data.push(-1.0);

            for raw_gate in raw_gates {
                if raw_gate < 2 {
                    data.push(-1.0);
                } else {
                    let scale = data_moment.data().scale();
                    let offset = data_moment.data().offset();

                    let mut scaled_gate = (raw_gate as f32 - offset) / scale;

                    scaled_gate -= min;
                    scaled_gate /= max - min;

                    if scaled_gate < 0.0 {
                        scaled_gate = 0.0;
                    }

                    if scaled_gate > 1.0 {
                        scaled_gate = 1.0;
                    }

                    data.push(scaled_gate);
                }
            }
    
            data.push(-1.0);
        }
        
        offset += az_count * range_count;
    }

    meta.sort_by(|a, b| a.elevation.partial_cmp(&b.elevation).unwrap());

    if meta.len() > 1 {
        let prev_diff = meta[meta.len() - 1].elevation - meta[meta.len()-2].elevation;
        meta.push(PySweep::empty(meta[meta.len() - 1].elevation + prev_diff))
    }

    PyScan::new(meta, data)
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
