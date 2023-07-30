use image::{GenericImageView, Rgba};
use crate::ansi::{ANSI_BOLD_ON, ANSI_RESET};
use crate::image_manager::open_image;
use crate::params::Params;

pub(crate) fn image_2_ascii(params: &Params) -> String {
    let image = open_image(&params.filename, params.width, params.height, params.char_size_ratio);
    if image.is_err() {
        return image.unwrap_err();
    }
    let image = image.unwrap();
    let result = convert_image_2_ascii(&image, params);
    result
}

fn convert_image_2_ascii(image: &image::DynamicImage, params: &Params) -> String {
    let (width, height) = image.dimensions();
    let mut result = String::new();
    if params.background_color.len() != 0 {
        result.push_str(format!("\x1b[{}m", params.background_color).as_str());
    }
    result.push_str(ANSI_BOLD_ON); // set bold
    let mut previous_color: Rgba<u8> = image.get_pixel(0, 0);
    for y in 0..height {
        for x in 0..width {
            let pixel_color = image.get_pixel(x, y);

            if pixel_color != previous_color {
                if let Some(ansi_color) = params.color_mode.colorize(&pixel_color) {
                    result.push_str(ansi_color.as_str());
                }
                previous_color = pixel_color;
            }
            let pixel_index = get_pixel(&pixel_color, params);
            let pixel = &params.pixels[pixel_index..pixel_index + 1];
            result.push_str(pixel);
        }
        result.push_str("\n");
    }
    result.push_str(ANSI_RESET);
    result
}

fn get_pixel(pixel_color: &Rgba<u8>, params: &Params) -> usize {
    let gray = rgb_2_grayscale(&pixel_color) as usize;
    let len = params.pixels.len();
    if params.inverted {
        let pixel_index = len - 1 - gray * len / 256;
        pixel_index
    } else {
        let pixel_index = gray * len / 256;
        pixel_index
    }
}

fn rgb_2_grayscale(pixel: &Rgba<u8>) -> u8 {
    let grayscale = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
    grayscale
}