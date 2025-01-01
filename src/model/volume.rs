use nexrad_data::volume::File;
use nexrad_decode::messages::{digital_radar_data, volume_coverage_pattern, Message};

use crate::model::sweep::Sweep;

pub struct Volume {
    pub sweeps: Vec<Sweep>,
}

impl Volume {
    pub(crate) fn new(file: &File) -> Self {
        let mut radials: Vec<Box<digital_radar_data::Message>> = Vec::new();
        let mut vcp: Option<Box<volume_coverage_pattern::Message>> = None;

        for mut record in file.records() {
            if record.compressed() {
                record = record
                    .decompress()
                    .expect("Should decompress record successfully");
            }

            let messages = record.messages().expect("Has messages");
            for message in messages {
                match message.message {
                    Message::DigitalRadarData(radar_data_message) => {
                        radials.push(radar_data_message);
                    }
                    Message::VolumeCoveragePattern(volume_coverage_pattern) => {
                        vcp = Some(volume_coverage_pattern);
                    }
                    _ => {}
                }
            }
        }

        let mut sweeps: Vec<Vec<Box<digital_radar_data::Message>>> = Vec::new();
        for _ in 0..vcp.as_ref().unwrap().header.number_of_elevation_cuts {
            sweeps.push(Vec::new());
        }

        for radial in radials {
            sweeps[radial.header.elevation_number as usize - 1].push(radial);
        }

        let mut result_sweeps: Vec<Sweep> = Vec::new();
        for (i, sweep) in sweeps.iter().enumerate() {
            result_sweeps.push(Sweep::new(&vcp.as_ref().unwrap().elevations[i], &sweep));
        }

        Self {
            sweeps: result_sweeps,
        }
    }
}
