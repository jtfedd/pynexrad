use nannou::prelude::*;
use nexrad::{decode::decode_file, decompress::decompress_file, file::is_compressed, model::DataFile};
use std::io::BufReader;
use std::fs::File;
use image::{DynamicImage,GenericImageView, ImageFormat};

fn main() {
    nannou::app(model)
        .view(view)
        .run();
}

struct Model {
    product: String,
    radar: DataFile,
    ref_scale: DynamicImage,
    vel_scale: DynamicImage,
    center: Point2,
    zoom: f32,
    last_mouse: Point2,
    dragging: bool,
}

fn model(app: &App) -> Model {
    println!("Loading file");
    let file_name = "examples/KDMX20240521_215236_V06";
    let mut file = std::fs::read(file_name).expect("file exists");

    println!("Decompressing file");
    if is_compressed(file.as_slice()) {
        file = decompress_file(file.as_slice()).expect("decompresses");
    }

    println!("Decoding file");
    let radar = decode_file(&file).expect("is valid");

    let ref_scale = image::load(BufReader::new(File::open("examples/reflectivity_scale.png").expect("file exists")), ImageFormat::Png).expect("image exists");
    let vel_scale = image::load(BufReader::new(File::open("examples/velocity_scale.png").expect("file exists")), ImageFormat::Png).expect("image exists");

    app.new_window()
        .size(1280, 800)
        .key_pressed(key_pressed)
        .mouse_wheel(mouse_wheel)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();

    Model {
        radar,
        ref_scale,
        vel_scale,
        product: String::from("ref"),
        center: Point2::new(0.0, 0.0),
        zoom: 0.004,
        last_mouse: Point2::new(0.0, 0.0),
        dragging: false,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::R {
        model.product = String::from("ref");
    }

    if key == Key::V {
        model.product = String::from("vel");
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.dragging = true;
    }
}

fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.dragging = false;
    }
}

fn mouse_wheel(_app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    let scroll: i32;

    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            scroll = y as i32;
        }
        MouseScrollDelta::PixelDelta(delta) => {
            scroll = delta.y as i32;
        }
    }

    if scroll == 0 {
        return;
    }

    for _ in 0..abs(scroll) {
        if scroll > 0 {
            model.zoom *= 1.2;
        } else {
            model.zoom /= 1.2;
        }
    }
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Point2) {
    if model.dragging {
        let delta = pos - model.last_mouse;
        model.center += delta;
    }

    model.last_mouse = pos;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let requested_product = model.product.as_str();

    let min=match requested_product {
        "ref" => -20.0,
        "vel" => -60.0,
        _ => panic!("Unexpected product: {}", requested_product)
    };

    let max = match requested_product {
        "ref" => 80.0,
        "vel" => 60.0,
        _ => panic!("Unexpected product: {}", requested_product)
    };

    let mut scans: Vec<_> = model.radar.elevation_scans().iter().collect();
    scans.sort_by(|a: &(&u8, &Vec<nexrad::model::Message31>), b| a.0.partial_cmp(&b.0).unwrap());

    let (_, radials) = scans[1];

    let radial = radials.iter().next().unwrap();
    let radial_reflectivity = radial.reflectivity_data().unwrap().data();

    let moment_range = radial_reflectivity.data_moment_range() as f32;
    let gate_width = radial_reflectivity.data_moment_range_sample_interval() as f32;

    let boundary = app.window_rect();
    let center_x = model.center.x;
    let center_y = model.center.y;

    let colors = match requested_product {
        "ref" => &model.ref_scale,
        "vel" => &model.vel_scale,
        _ => panic!("Unexpected product: {}", requested_product)
    };

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

        let mut distance = moment_range;

        for scaled_gate in scaled_gates {
            if scaled_gate < 0.0 {
                distance += gate_width;
                continue;
            }

            let dist_near = distance * model.zoom;
            let dist_far = (distance + gate_width) * model.zoom;

            let angle_cos = azimuth_first.cos();
            let angle_sin = azimuth_first.sin();

            let point1 = pt2(center_x + angle_cos * dist_near, center_y + angle_sin * dist_near);
            let point2 = pt2(center_x + angle_cos * dist_far, center_y + angle_sin * dist_far);

            let angle_cos = azimuth_last.cos();
            let angle_sin = azimuth_last.sin();

            let point3 = pt2(center_x + angle_cos * dist_far, center_y + angle_sin * dist_far);
            let point4 = pt2(center_x + angle_cos * dist_near, center_y + angle_sin * dist_near);

            if boundary.contains(point1) ||
                boundary.contains(point2) ||
                boundary.contains(point3) ||
                boundary.contains(point4) {

                let mut color_index = ((1.0 - scaled_gate) * (colors.height() as f32)).floor() as u32;
                if color_index == colors.height() {
                    color_index = colors.height() - 1;
                }

                let color = colors.get_pixel(0, color_index);

                draw.quad()
                    .color(rgb(color[0], color[1], color[2]))
                    .points(point1, point2, point3, point4);
            }

            distance += gate_width;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}