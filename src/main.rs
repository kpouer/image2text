use crate::params::Params;
use pixel::PixelMode;

mod ansi;
mod color;
mod converter;
mod image_manager;
mod params;
mod pixel;

fn main() {
    if let Ok(params) = parse_options() {
        if params.filename.is_empty() {
            println!("no filename given");
            return;
        }
        let str = converter::image_2_ascii(&params);
        println!("{}", str);
    } else {
        usage();
    }
}

fn parse_options() -> Result<Params, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut params = Params::new();

    let mut i = 1;
    while i < args.len() - 1 {
        let current_param = &args[i];
        if current_param == "-w" || current_param == "--width" {
            params.width = args[i + 1].parse::<u16>().unwrap();
            i += 2;
        } else if current_param == "-h" || current_param == "--height" {
            params.height = args[i + 1].parse::<u16>().unwrap();
            i += 2;
        } else if current_param == "-c" || current_param == "--color" {
            let next_arg = args[i + 1].clone();
            i += 2;
            if let Ok(color_mode) = next_arg.parse::<color::ColorMode>() {
                params.color_mode = color_mode;
            } else {
                return Err("color mode should be bw, ansi or truecolor".to_string());
            }
        } else if current_param == "-p" || current_param == "--pixel" {
            if let Ok(pixel_mode) = args[i + 1].parse::<PixelMode>() {
                params.pixels = pixel_mode.get_pixels().to_string();
            } else {
                return Err("pixel mode should be ascii, ascii2 or unicode".to_string());
            }
            i += 2;
        } else if current_param == "-cp" || current_param == "--custom-pixel" {
            params.pixels.clone_from(&args[i + 1]);
            i += 2;
        } else if current_param == "-cr" || current_param == "--char-size-ratio" {
            params.char_size_ratio = args[i + 1].parse::<f32>().unwrap();
            i += 2;
        } else if current_param == "-i" || current_param == "--inverted" {
            params.inverted = true;
            i += 1;
        } else if current_param == "-bg" || current_param == "--background-color" {
            params.background_color = args[i + 1].to_string();
            i += 2;
        } else if current_param == "-f" || current_param == "--file" {
            params.filename = args[i + 1].to_string();
            i += 2;
        } else {
            return Err(format!("unknown option {}", current_param));
        }
    }

    if params.filename.is_empty() {
        return Err("image file should not be empty".to_string());
    }
    Ok(params)
}

fn usage() {
    println!("image2text");
    println!("Usage: image2text [OPTIONS] [FILE]");
    println!("convert an image to ascii art");
    println!();
    println!("  -f,  --file <file>              the image filename you want to conver");
    println!("  -h,  --height <height>          the height of the output ascii art");
    println!("  -w,  --width <width>            the width of the output ascii art");
    println!("  -c,  --color <colormode>        the color mode of the output ascii art (bw, ansi or truecolor)");
    println!("  -i,  --inverted                 Invert the pixels");
    println!("  -p,  --pixel <pixel>            the pixel mode of the output ascii art (ascii, ascii2 or unicode)");
    println!("  -cp, --custom-pixel <pixel>     the custom pixel of the output ascii art");
    println!("  -cr, --char-size-ratio <ratio>  the char size ratio of the output ascii art");
}
