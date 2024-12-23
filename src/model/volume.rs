use nexrad_data::volume::File;
use nexrad_decode::messages::{digital_radar_data, Message};

use crate::model::sweep::Sweep;

pub struct Volume {
    pub sweeps: Vec<Sweep>,
}

impl Volume {
    pub(crate) fn new(file: &File) -> Self {
        let mut radials: Vec<Box<digital_radar_data::Message>> = Vec::new();
        let mut max_el_number: u8 = 0;

        for mut record in file.records() {
            if record.compressed() {
                record = record
                    .decompress()
                    .expect("Should decompress record successfully");
            }

            let messages = record.messages().expect("Has messages");
            for message in messages {
                if let Message::DigitalRadarData(radar_data_message) = message.message {
                    if radar_data_message.header.elevation_number > max_el_number {
                        max_el_number = radar_data_message.header.elevation_number;
                    }
                    radials.push(radar_data_message);
                }
            }
        }

        let mut sweeps: Vec<Vec<Box<digital_radar_data::Message>>> = Vec::new();
        for _ in 0..max_el_number {
            sweeps.push(Vec::new());
        }

        for radial in radials {
            sweeps[radial.header.elevation_number as usize - 1].push(radial);
        }

        let mut result_sweeps: Vec<Sweep> = Vec::new();
        for sweep in sweeps {
            result_sweeps.push(Sweep::new(&sweep));
        }

        Self {
            sweeps: result_sweeps,
        }
    }
}
