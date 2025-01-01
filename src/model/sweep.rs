use std::f32::consts::PI;

use chrono::{DateTime, Utc};
use nexrad_decode::messages::digital_radar_data::{Message, ScaledMomentValue};

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

    pub time: DateTime<Utc>,

    pub reflectivity: Option<SweepData>,
    pub velocity: Option<SweepData>,
}

fn extract_data(
    radials: &Vec<Box<Message>>,
    data_type: &str,
    az_count: usize,
    range_count: usize,
) -> Option<SweepData> {
    if !validate_sweep(radials, data_type) {
        return None;
    }

    let mut data = SweepData::new(az_count as usize, range_count as usize);

    let mut sorted_radials: Vec<_> = radials.iter().collect();
    sorted_radials.sort_by(|a, b| {
        a.header
            .azimuth_angle
            .partial_cmp(&b.header.azimuth_angle)
            .unwrap()
    });
    for (radial_index, radial) in sorted_radials.iter().enumerate() {
        let data_moment = match data_type {
            "ref" => radial.reflectivity_data_block.as_ref().unwrap(),
            "vel" => radial.velocity_data_block.as_ref().unwrap(),
            _ => panic!("Unexpected product: {}", data_type),
        };

        for (gate_index, gate_value) in data_moment.decoded_values().iter().enumerate() {
            match gate_value {
                ScaledMomentValue::Value(value) => {
                    data.set_value(*value, radial_index, gate_index);
                }
                _ => {}
            }
        }
    }

    return Some(data);
}

fn extract_nyquist_vel(radials: &Vec<Box<Message>>) -> f32 {
    let nyquist_vel = radials[0]
        .radial_data_block
        .as_ref()
        .unwrap()
        .nyquist_velocity;

    for radial in radials {
        if nyquist_vel != radial.radial_data_block.as_ref().unwrap().nyquist_velocity {
            panic!("Nyquist values are not consistent");
        }
    }

    return nyquist_vel as f32 * 0.01;
}

fn validate_sweep(radials: &Vec<Box<Message>>, data_type: &str) -> bool {
    for radial in radials {
        let data_moment = match data_type {
            "ref" => &radial.reflectivity_data_block,
            "vel" => &radial.velocity_data_block,
            _ => panic!("Unexpected product: {}", data_type),
        };

        if data_moment.is_none() {
            return false;
        }
    }

    true
}

fn extract_range_info(radial: &Message, data_type: &str) -> (f32, f32, i32) {
    let mut sample_data_moment = radial.reflectivity_data_block.as_ref().unwrap();
    if data_type == "vel" && radial.velocity_data_block.is_some() {
        sample_data_moment = radial.velocity_data_block.as_ref().unwrap();
    }

    let range_step = sample_data_moment.header.data_moment_range_sample_interval as f32 / 1000.0;
    let range_first = sample_data_moment.header.data_moment_range as f32 / 1000.0;
    let range_count = sample_data_moment.header.number_of_data_moment_gates as i32;

    return (range_first, range_step, range_count);
}

impl Sweep {
    pub(crate) fn new(radials: &Vec<Box<Message>>) -> Self {
        let mut elevation_avg = 0.0 as f32;
        for radial in radials.iter() {
            elevation_avg += radial.header.elevation_angle;
        }
        elevation_avg /= radials.len() as f32;
        elevation_avg *= PI / 180.0;

        let elevation = elevation_avg;

        let rad_hdr = &radials[0].header;
        let az_first = (rad_hdr.azimuth_indexing_mode as f32 / 100.0) * PI / 180.0;
        let az_count = radials.len() as i32;
        let az_step = if rad_hdr.azimuth_resolution_spacing == 1 {
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

        let time = radials
            .iter()
            .map(|r| r.header.date_time().unwrap())
            .max()
            .unwrap();

        return Self {
            elevation,
            az_first,
            az_step,
            az_count,
            range_first,
            range_step,
            range_count,
            nyquist_vel,
            time,
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
