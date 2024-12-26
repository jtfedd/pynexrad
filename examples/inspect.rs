use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use nexrad_data::volume;
use nexrad_decode::messages::{digital_radar_data, Message};

fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    println!("Loading file");
    let file_name = "examples/KDMX20240521_215236_V06";
    let bytes = std::fs::read(file_name).expect("file exists");
    let file = volume::File::new(bytes);

    // Collect all of the radials from the file
    let mut radials: Vec<Box<digital_radar_data::Message>> = Vec::new();
    let mut max_el_number: u8 = 0;

    let records = file.records();

    for mut record in records {
        if record.compressed() {
            record = record
                .decompress()
                .expect("Should decompress record successfully");
        }

        let messages = record.messages().expect("Has messages");
        for message in messages {
            if let Message::DigitalRadarData(radar_data_message) = message.message.clone() {
                if radar_data_message.header.elevation_number > max_el_number {
                    max_el_number = radar_data_message.header.elevation_number;
                }
                radials.push(radar_data_message);
            }

            if let Message::VolumeCoveragePattern(vcp) = message.message.clone() {
                println!("{:#?}", vcp)
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

    for (i, sweep) in sweeps.iter().enumerate() {
        let mut max_el = 0 as f32;
        let mut min_el = 1000 as f32;
        let mut avg_el = 0 as f32;
        let mut max_time: Option<DateTime<Utc>> = None;

        for radial in sweep {
            avg_el += radial.header.elevation_angle;

            if radial.header.elevation_angle < min_el {
                min_el = radial.header.elevation_angle
            }

            if radial.header.elevation_angle > max_el {
                max_el = radial.header.elevation_angle
            }

            if max_time.is_none() || max_time.unwrap().partial_cmp(&radial.header.date_time().unwrap()).unwrap() == Ordering::Less {
                max_time = Some(radial.header.date_time().unwrap())
            }
        }

        avg_el /= sweep.len() as f32;

        println!(
            "{:<2} {:>7.2} {:>7.2} {:>7.2} {} {} {}",
            i,
            min_el,
            max_el,
            avg_el,
            sweep[0]
            .header
            .date_time()
            .expect("has")
            .format("%d/%m/%Y %H:%M:%S"),
            max_time.unwrap().format("%d/%m/%Y %H:%M:%S"),
            max_time.unwrap().timestamp(),
        );
    }
}
