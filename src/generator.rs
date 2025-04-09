use base64::prelude::*;
use std::io::BufWriter;

use leptos::prelude::*;

const SCALE: u32 = 1;

fn get_image_dimensions(str: &String) -> (u32, u32) {
    let raw_bytes = str.as_bytes();
    let len = raw_bytes.len() as u32;
    return (len * SCALE, 8 * SCALE);
}

fn color_value(char: u8, bit: u8) -> u8 {
    let mask = 1 << bit;
    if char & mask > 0 {
        return 255;
    }
    return 0;
}

fn get_png_bytes(str: &String) -> Vec<u8> {
    let raw_bytes = str.as_bytes();
    let mut data: Vec<u8> = Vec::new();

    for bit in 0..8 {
        for char_i in 0..raw_bytes.len() {
            let char = raw_bytes[char_i];
            let bit_pos_x = char_i;
            let bit_pos_y = 1;
            data.push(color_value(char, bit));
            data.push(0);
            data.push(0);
            data.push(255);
        }
    }

    return Vec::from(data);
}

fn generate_image(bytes: &mut Vec<u8>, str: &String) -> () {
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

    let data = get_png_bytes(str);

    writer.write_image_data(&data).unwrap();
}

#[component]
pub fn Generator() -> impl IntoView {
    let mut bytes: Vec<u8> = Vec::new();
    let str = String::from("djmetzle");

    generate_image(&mut bytes, &str);

    let result = BASE64_STANDARD.encode(bytes);
    let data_url = String::from("data:image/png;base64,") + result.as_str();

    view! {
        <p>IMAGE</p>
        <img height="auto" width=512 object-fit="contain" src={data_url} />
    }
}
