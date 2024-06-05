use std::f32::consts::PI;

use nexrad::model::Message31;

use crate::model::sweep_data::SweepData;

pub struct Sweep {
    pub elevation: f32,

    pub az_first: f32,
    pub az_step: f32,
    pub az_count: i32,

    pub range_first: f32,
    pub range_step: f32,
    pub range_count: i32,

    pub data: SweepData,
}

impl Sweep {
    pub(crate) fn new(radials: &Vec<Message31>, data_type: &str) -> Self {
        let sample_data_moment = match data_type {
            "ref" => radials[0].reflectivity_data().unwrap(),
            "vel" => radials[0].velocity_data().unwrap(),
            _ => panic!("Unexpected product: {}", data_type),
        };

        let mut elevation_avg = 0.0 as f32;
        for radial in radials.iter() {
            elevation_avg += radial.header().elev() as f32;
        }
        elevation_avg /= radials.len() as f32;
        elevation_avg *= PI / 180.0;

        let elevation = elevation_avg;

        let rad_hdr = radials[0].header();
        let az_first = (rad_hdr.azm_indexing_mode() as f32 / 100.0) * PI / 180.0;
        let az_count = radials.len() as i32;
        let az_step = if rad_hdr.azm_res() == 1 {
            0.5 * PI / 180.0
        } else {
            PI / 180.0
        };

        let range_step = sample_data_moment
            .data()
            .data_moment_range_sample_interval() as f32
            / 1000.0;
        let range_first = sample_data_moment.data().data_moment_range() as f32 / 1000.0;
        let range_count = sample_data_moment.data().number_data_moment_gates() as i32;

        let mut data = SweepData::new(az_count as usize, range_count as usize);

        let mut sorted_radials: Vec<_> = radials.iter().collect();
        sorted_radials.sort_by(|a, b| a.header().azm().partial_cmp(&b.header().azm()).unwrap());
        for (radial_index, radial) in sorted_radials.iter().enumerate() {
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

            for (gate_index, raw_gate) in raw_gates.iter().enumerate() {
                if *raw_gate < 2 {
                    data.set_mask(radial_index, gate_index);
                } else {
                    let scale = data_moment.data().scale();
                    let offset = data_moment.data().offset();
                    let scaled_gate = (*raw_gate as f32 - offset) / scale;
                    data.set_value(scaled_gate, radial_index, gate_index);
                }
            }
        }

        return Self {
            elevation,
            az_first,
            az_step,
            az_count,
            range_first,
            range_step,
            range_count,
            data,
        };
    }
}
