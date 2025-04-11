use base64::prelude::*;
use std::io::BufWriter;

use leptos::prelude::*;

use colors_transform::{Color, Hsl};

use crate::settings::Background;
use crate::settings::Opacity;
use crate::settings::Spec;

const SCALE: u32 = 32;

mod bytes;

fn get_image_dimensions(str: &String) -> (u32, u32) {
    let raw_bytes = str.as_bytes();
    let len = raw_bytes.len() as u32;
    return (len * SCALE, 8 * SCALE);
}

fn pixel_bytes(char: u8, bit: u8, color: &Hsl, bg: u8, opacity: u8) -> (u8, u8, u8, u8) {
    let r = color.get_red() as u8;
    let g = color.get_green() as u8;
    let b = color.get_blue() as u8;
    let mask = 1 << bit;
    if char & mask > 0 {
        return (r, g, b, 255);
    }
    return (bg, bg, bg, opacity);
}

fn get_png_bytes(str: &String, spec: &Spec) -> Vec<u8> {
    let raw_bytes = str.as_bytes();
    let mut data: Vec<u8> = Vec::new();
    let hue = spec.hue;
    let color = Hsl::from(hue as f32, 100.0, 50.0);
    let background: u8 = match spec.bg {
        Background::Black => 0,
        Background::White => 255,
    };
    let opacity = match spec.opacity {
        Opacity::Solid => 255,
        Opacity::Transparent => 0,
    };

    let columns = 0..8;
    let bytes_len = raw_bytes.len();

    columns.for_each(|bit| {
        for _bit_i in 0..SCALE {
            let rows = 0..bytes_len;
            rows.for_each(|char_i| {
                let char = raw_bytes[char_i];
                for _char_i in 0..SCALE {
                    let (r, g, b, a) = pixel_bytes(char, bit, &color, background, opacity);
                    data.push(r);
                    data.push(g);
                    data.push(b);
                    data.push(a);
                }
            });
        }
    });

    return data;
}

fn generate_image(bytes: &mut Vec<u8>, str: &String, spec: &Spec) -> () {
    let stream = BufWriter::new(bytes);

    let (width, height) = get_image_dimensions(str);

    let mut encoder = png::Encoder::new(stream, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);

    let mut writer = encoder.write_header().unwrap();

    let data = get_png_bytes(str, spec);

    writer.write_image_data(&data).unwrap();
}

fn get_data_url(string: String, spec: Spec) -> String {
    if string.is_empty() {
        return String::from("");
    }
    let mut bytes: Vec<u8> = Vec::new();
    let gen_bytes = bytes::Bytes::new(string.clone(), &spec.orient, &spec.ordering);

    generate_image(&mut bytes, &string, &spec);

    let result = BASE64_STANDARD.encode(bytes);
    let data_url = String::from("data:image/png;base64,") + result.as_str();

    return data_url;
}

#[component]
pub fn Generator(string: ReadSignal<String>, spec: ReadSignal<Spec>) -> impl IntoView {
    view! {
        <h2>Your Image:</h2>
        <p>Right click and save to download...</p>
        <div style:padding="1em" style:background="#777">
            <img height=512 width="auto" object-fit="contain" src={move || get_data_url(string.get(), spec.get())} />
        </div>
    }
}
