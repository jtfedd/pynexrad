use image::{DynamicImage, GenericImageView, ImageFormat};
use nannou::prelude::*;
use nexrad::{decode::decode_file, decompress::decompress_file, file::is_compressed};
use pynexrad::{convert::convert_nexrad_file, pymodel::py_level2_file::PyLevel2File};
use std::fs::File;
use std::io::BufReader;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {
    product: String,
    radar: PyLevel2File,
    sweep: i32,
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

    println!("Converting file");
    let pyradar = convert_nexrad_file(&radar);

    let ref_scale = image::load(
        BufReader::new(File::open("examples/reflectivity_scale.png").expect("file exists")),
        ImageFormat::Png,
    )
    .expect("image exists");
    let vel_scale = image::load(
        BufReader::new(File::open("examples/velocity_scale.png").expect("file exists")),
        ImageFormat::Png,
    )
    .expect("image exists");

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
        radar: pyradar,
        sweep: 1,
        ref_scale,
        vel_scale,
        product: String::from("ref"),
        center: Point2::new(0.0, 0.0),
        zoom: 2.0,
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
    let scroll: f32;

    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            scroll = y as f32;
        }
        MouseScrollDelta::PixelDelta(delta) => {
            scroll = (delta.y as f32) / 15.0;
        }
    }

    model.zoom *= f32::powf(1.2, scroll);
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Point2) {
    if model.dragging {
        let delta = pos - model.last_mouse;
        model.center += delta / model.zoom;
    }

    model.last_mouse = pos;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let requested_product = model.product.as_str();

    let data = match requested_product {
        "ref" => &model.radar.reflectivity.sweeps[model.sweep as usize],
        "vel" => &model.radar.velocity.sweeps[model.sweep as usize],
        _ => panic!("Unexpected product: {}", requested_product),
    };

    let colors = match requested_product {
        "ref" => &model.ref_scale,
        "vel" => &model.vel_scale,
        _ => panic!("Unexpected product: {}", requested_product),
    };

    let boundary = app.window_rect();
    let center_x = model.center.x * model.zoom;
    let center_y = model.center.y * model.zoom;

    for i in 0..data.az_count {
        let az = (PI / 2.0) - (data.az_first + (i as f32) * data.az_step);
        let az_first = az - (data.az_step / 2.0);
        let az_last = az + (data.az_step / 2.0);

        let first_cos = az_first.cos();
        let first_sin = az_first.sin();
        let last_cos = az_last.cos();
        let last_sin = az_last.sin();

        for j in 0..data.range_count {
            let value = data.data[((i * data.range_count) + j) as usize];

            if value < 0.0 {
                continue;
            }

            let dist_near = data.range_first + (j as f32 * data.range_step) * model.zoom;
            let dist_far = data.range_first + ((j + 1) as f32 * data.range_step) * model.zoom;

            let point1 = pt2(
                center_x + first_cos * dist_near,
                center_y + first_sin * dist_near,
            );
            let point2 = pt2(
                center_x + first_cos * dist_far,
                center_y + first_sin * dist_far,
            );

            let point3 = pt2(
                center_x + last_cos * dist_far,
                center_y + last_sin * dist_far,
            );
            let point4 = pt2(
                center_x + last_cos * dist_near,
                center_y + last_sin * dist_near,
            );

            if boundary.contains(point1)
                || boundary.contains(point2)
                || boundary.contains(point3)
                || boundary.contains(point4)
            {
                let mut color_index = ((1.0 - value) * (colors.height() as f32)).floor() as u32;
                if color_index == colors.height() {
                    color_index = colors.height() - 1;
                }

                let color = colors.get_pixel(0, color_index);

                draw.quad()
                    .color(rgb(color[0], color[1], color[2]))
                    .points(point1, point2, point3, point4);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
