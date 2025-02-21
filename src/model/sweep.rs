use std::f32::consts::PI;

use chrono::{DateTime, Utc};
use nexrad_decode::messages::{
    digital_radar_data::{Message, ScaledMomentValue},
    volume_coverage_pattern::{ElevationDataBlock, WaveformType},
};

use crate::model::sweep_data::SweepData;
use crate::model::sweep_type::*;

use uom::si::{
    angle::{degree, radian},
    f64::Angle,
    length::kilometer,
    velocity::meter_per_second,
};

pub struct Sweep {
    pub elevation: f32,

    pub az_first: f32,
    pub az_step: f32,
    pub az_count: i32,

    pub range_first: f32,
    pub range_step: f32,
    pub range_count: i32,

    pub nyquist_vel: f32,

    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    pub sweep_type: u8,

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
        .nyquist_velocity();

    for radial in radials {
        if nyquist_vel
            != radial
                .radial_data_block
                .as_ref()
                .unwrap()
                .nyquist_velocity()
        {
            panic!("Nyquist values are not consistent");
        }
    }

    return nyquist_vel.get::<meter_per_second>() as f32;
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

    let range_step = sample_data_moment
        .header
        .data_moment_range_sample_interval()
        .get::<kilometer>() as f32;
    let range_first = sample_data_moment
        .header
        .data_moment_range()
        .get::<kilometer>() as f32;
    let range_count = sample_data_moment.header.number_of_data_moment_gates as i32;

    return (range_first, range_step, range_count);
}

impl Sweep {
    pub(crate) fn new(
        elevation_meta: &ElevationDataBlock,
        radials: &Vec<Box<Message>>,
    ) -> Option<Self> {
        // If there are no radials we cannot create a sweep
        if radials.len() == 0 {
            return None;
        }

        let elevation = elevation_meta.elevation_angle().get::<radian>() as f32;

        let rad_hdr = &radials[0].header;
        let az_first = rad_hdr
            .azimuth_indexing_mode()
            .unwrap_or(Angle::new::<degree>(0.0))
            .get::<radian>() as f32;
        let az_count = radials.len() as i32;
        let az_step = rad_hdr.azimuth_resolution_spacing().get::<radian>() as f32;

        // Verify that there are the expected number of radials to make the sweep
        if ((2.0 * PI) / az_step).round() != az_count as f32 {
            return None;
        }

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

        let start_time = radials
            .iter()
            .map(|r| r.header.date_time().unwrap())
            .min()
            .unwrap();

        let end_time = radials
            .iter()
            .map(|r| r.header.date_time().unwrap())
            .max()
            .unwrap();

        let is_reflectivity =
            reflectivity.is_some() && elevation_meta.waveform_type() != WaveformType::CDW;
        let is_velocity = velocity.is_some() && elevation_meta.waveform_type() != WaveformType::CS;

        let mut sweep_type: u8 = 0;
        if is_reflectivity {
            sweep_type |= REFLECTIVITY;
        }
        if is_velocity {
            sweep_type |= VELOCITY;
        }

        return Some(Self {
            elevation,
            az_first,
            az_step,
            az_count,
            range_first,
            range_step,
            range_count,
            nyquist_vel,
            start_time,
            end_time,
            sweep_type,
            reflectivity,
            velocity,
        });
    }

    pub(crate) fn has_product(&self, product: SweepType) -> bool {
        return match product {
            REFLECTIVITY => self.reflectivity.is_some(),
            VELOCITY => self.velocity.is_some(),
            _ => false,
        };
    }
}
