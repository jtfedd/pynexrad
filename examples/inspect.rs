use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use nexrad_data::volume;
use nexrad_decode::messages::{Message, digital_radar_data, volume_coverage_pattern};

fn main() {
    println!("Loading file");
    let file_name = "examples/KDMX20240521_215236_V06";
    let bytes = std::fs::read(file_name).expect("file exists");
    let file = volume::File::new(bytes);

    // Collect all of the radials from the file
    let mut radials: Vec<Box<digital_radar_data::Message>> = Vec::new();
    let mut vcp: Option<Box<volume_coverage_pattern::Message>> = None;
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
            match message.message {
                Message::DigitalRadarData(radar_data_message) => {
                    if radar_data_message.header.elevation_number > max_el_number {
                        max_el_number = radar_data_message.header.elevation_number;
                    }
                    radials.push(radar_data_message);
                },
                Message::VolumeCoveragePattern(m) => {
                    vcp = Some(m)
                }
                _ => {}
            }
        }
    }

    let mut sweeps: Vec<Vec<&Box<digital_radar_data::Message>>> = Vec::new();
    for _ in 0..max_el_number {
        sweeps.push(Vec::new());
    }

    for radial in &radials {
        sweeps[radial.header.elevation_number as usize - 1].push(radial);
    }

    for (i, sweep) in sweeps.iter().enumerate() {
        let mut max_el = 0 as f32;
        let mut min_el = 1000 as f32;
        let mut avg_el = 0 as f32;

        for radial in sweep {
            avg_el += radial.header.elevation_angle;

            if radial.header.elevation_angle < min_el {
                min_el = radial.header.elevation_angle
            }

            if radial.header.elevation_angle > max_el {
                max_el = radial.header.elevation_angle
            }
        }

        let max_time = sweep.iter().map(|r| r.header.date_time().unwrap()).max().unwrap();

        avg_el /= sweep.len() as f32;

        println!(
            "{:<2} {:>7.2} {:>7.2} {:>7.2} {} {:>15} {:<4?}",
            i,
            min_el,
            max_el,
            avg_el,
            max_time.format("%d/%m/%Y %H:%M:%S"),
            format_args!("{:>15}", format!("{:?}", vcp.as_ref().unwrap().elevations[i].channel_configuration())),
            vcp.as_ref().unwrap().elevations[i].waveform_type(),
        );
    }
}
