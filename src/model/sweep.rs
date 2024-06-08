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

    pub nyquist_vel: f32,

    pub reflectivity: Option<SweepData>,
    pub velocity: Option<SweepData>,
}

fn extract_data(
    radials: &Vec<Message31>,
    data_type: &str,
    az_count: usize,
    range_count: usize,
) -> Option<SweepData> {
    if !validate_sweep(radials, data_type) {
        return None;
    }

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

        let scale = data_moment.data().scale();
        let offset = data_moment.data().offset();

        for (gate_index, raw_gate) in raw_gates.iter().enumerate() {
            if *raw_gate >= 2 {
                let scaled_gate = (*raw_gate as f32 - offset) / scale;
                data.set_value(scaled_gate, radial_index, gate_index);
            }
        }
    }

    return Some(data);
}

fn extract_nyquist_vel(radials: &Vec<Message31>) -> f32 {
    let nyquist_vel = radials[0].radial_data().unwrap().nyquist_velocity();

    for radial in radials {
        if nyquist_vel != radial.radial_data().unwrap().nyquist_velocity() {
            panic!("Nyquist values are not consistent");
        }
    }

    return nyquist_vel as f32 * 0.01;
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

fn extract_range_info(radial: &Message31, data_type: &str) -> (f32, f32, i32) {
    let mut sample_data_moment = radial.reflectivity_data().unwrap();
    if data_type == "vel" && radial.velocity_data().is_some() {
        sample_data_moment = radial.velocity_data().unwrap();
    }

    let range_step = sample_data_moment
        .data()
        .data_moment_range_sample_interval() as f32
        / 1000.0;
    let range_first = sample_data_moment.data().data_moment_range() as f32 / 1000.0;
    let range_count = sample_data_moment.data().number_data_moment_gates() as i32;

    return (range_first, range_step, range_count);
}

impl Sweep {
    pub(crate) fn new(radials: &Vec<Message31>) -> Self {
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

        let (r_first, r_step, r_count) = extract_range_info(&radials[0], "ref");
        let (v_first, v_step, v_count) = extract_range_info(&radials[0], "vel");

        if r_first != v_first {
            panic!("First gate does not match")
        }

        if r_step != v_step {
            panic!("Gate step does not match")
        }

        let range_first = r_first;
        let range_step = r_step;
        let range_count = i32::max(r_count, v_count);

        let nyquist_vel = extract_nyquist_vel(radials);

        let reflectivity = extract_data(radials, "ref", az_count as usize, range_count as usize);
        let velocity = extract_data(radials, "vel", az_count as usize, range_count as usize);

        return Self {
            elevation,
            az_first,
            az_step,
            az_count,
            range_first,
            range_step,
            range_count,
            nyquist_vel,
            reflectivity,
            velocity,
        };
    }

    pub(crate) fn has_product(&self, product: &str) -> bool {
        return match product {
            "ref" => self.reflectivity.is_some(),
            "vel" => self.velocity.is_some(),
            _ => false,
        };
    }
}
