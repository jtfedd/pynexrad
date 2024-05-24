use nannou::prelude::*;
use nexrad::{decode::decode_file, decompress::decompress_file, file::is_compressed, model::DataFile};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    radar: DataFile,
}

fn model(_app: &App) -> Model {
    println!("Loading file");
    let file_name = "KDMX20240521_215236_V06";
    let mut file = std::fs::read(file_name).expect("file exists");

    println!("Decompressing file");
    if is_compressed(file.as_slice()) {
        file = decompress_file(file.as_slice()).expect("decompresses");
    }

    println!("Decoding file");
    let radar = decode_file(&file).expect("is valid");

    Model {radar}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    println!("View");

    let draw = app.draw();
    draw.background().color(BLACK);

    let requested_product = "vel";
    let min = -20.0;
    let max = 20.0;

    let mut scans: Vec<_> = model.radar.elevation_scans().iter().collect();
    scans.sort_by(|a: &(&u8, &Vec<nexrad::model::Message31>), b| a.0.partial_cmp(&b.0).unwrap());

    let (_, radials) = scans[1];

    let radial = radials.iter().next().unwrap();
    let radial_reflectivity = radial.reflectivity_data().unwrap().data();

    let moment_range = radial_reflectivity.data_moment_range();
    let gate_width = radial_reflectivity.data_moment_range_sample_interval() as f32;

    for radial in radials {
        
        let mut azimuth_angle = radial.header().azm() - 90.0;
        if azimuth_angle < 0.0 {
            azimuth_angle = 360.0 + azimuth_angle;
        }
        azimuth_angle = -azimuth_angle;

        let azimuth_spacing = radial.header().azm_res() as f32;
        let azimuth_first = (azimuth_angle - (azimuth_spacing / 4.0)) * (PI / 180.0);
        let azimuth_last = (azimuth_angle + (azimuth_spacing / 4.0)) * (PI / 180.0);

        let data_moment = match requested_product {
            "ref" => radial.reflectivity_data().unwrap(),
            "vel" => radial.velocity_data().unwrap(),
            _ => panic!("Unexpected product: {}", requested_product),
        };

        let mut raw_gates: Vec<u16> =
            vec![0; data_moment.data().number_data_moment_gates() as usize];

        for (i, v) in data_moment.moment_data().iter().enumerate() {
            raw_gates[i] = *v as u16;
        }

        let mut scaled_gates: Vec<f32> = Vec::new();
        for raw_gate in raw_gates {
            if raw_gate < 2 {
                scaled_gates.push(-1.0);
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

                scaled_gates.push(scaled_gate);
            }
        }

        let mut distance = moment_range as f32;

        for scaled_gate in scaled_gates {
            if scaled_gate < 0.0 {
                distance += gate_width;
                continue;
            }

            let mut scale = 1.0 / 1000.0;
            scale *= 4.0;

            let dist_near = distance * scale;
            let dist_far = (distance + gate_width) * scale;

            let angle_cos = azimuth_first.cos();
            let angle_sin = azimuth_first.sin();

            let point1 = pt2(angle_cos * dist_near, angle_sin * dist_near);
            let point2 = pt2(angle_cos * dist_far, angle_sin * dist_far);

            let angle_cos = azimuth_last.cos();
            let angle_sin = azimuth_last.sin();

            let point3 = pt2(angle_cos * dist_far, angle_sin * dist_far);
            let point4 = pt2(angle_cos * dist_near, angle_sin * dist_near);

            draw.quad()
                .color(rgb(scaled_gate, 1.0-scaled_gate, 0.0))
                .points(point1, point2, point3, point4);

            distance += gate_width;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}