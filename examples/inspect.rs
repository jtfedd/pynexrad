use nexrad_data::volume;
use nexrad_decode::messages::{digital_radar_data, volume_coverage_pattern, MessageContents};

fn main() {
    println!("Loading file");
    let file_name = "examples/KDMX20240521_224629_V06";
    let bytes = std::fs::read(file_name).expect("file exists");
    let file = volume::File::new(bytes);

    // Collect all of the radials from the file
    let mut radials: Vec<Box<digital_radar_data::Message>> = Vec::new();
    let mut vcp_option: Option<Box<volume_coverage_pattern::Message>> = None;

    let records = file.records();

    for mut record in records {
        if record.compressed() {
            record = record
                .decompress()
                .expect("Should decompress record successfully");
        }

        let messages = record.messages().expect("Has messages");
        for message in messages {
            match message.contents().clone() {
                MessageContents::DigitalRadarData(radar_data_message) => {
                    radials.push(radar_data_message);
                }
                MessageContents::VolumeCoveragePattern(m) => vcp_option = Some(m),
                _ => {}
            }
        }
    }

    let vcp = vcp_option.as_ref().unwrap();
    // println!("{:#?}", vcp);

    let mut sweeps: Vec<Vec<&Box<digital_radar_data::Message>>> = Vec::new();
    for _ in 0..vcp.header.number_of_elevation_cuts {
        sweeps.push(Vec::new());
    }

    for radial in &radials {
        sweeps[radial.header.elevation_number as usize - 1].push(radial);
    }

    for (i, sweep) in sweeps.iter().enumerate() {
        println!(
            "{}, {}, {}",
            i,
            sweep.len(),
            vcp.elevations[i].elevation_angle_degrees()
        );
        if sweep.len() == 0 {
            continue;
        }

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

        let max_time = sweep
            .iter()
            .map(|r| r.header.date_time().unwrap())
            .max()
            .unwrap();

        avg_el /= sweep.len() as f32;

        println!(
            "{:<2} {:<4} {:>7.2} {:>7.2} {:>7.2} {} {:>15} {:<4?}",
            i,
            sweep.len(),
            min_el,
            max_el,
            avg_el,
            max_time.format("%d/%m/%Y %H:%M:%S"),
            format_args!(
                "{:>15}",
                format!("{:?}", vcp.elevations[i].channel_configuration())
            ),
            vcp.elevations[i].waveform_type(),
        );
    }
}
