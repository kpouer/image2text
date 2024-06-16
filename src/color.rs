use std::str::FromStr;

use image::Rgba;
use rgb2ansi256::rgb_to_ansi256;

pub(crate) enum ColorMode {
    BlackAndWhite,
    Ansi,
    TrueColor,
}

impl FromStr for ColorMode {
    type Err = ();

    fn from_str(input: &str) -> Result<ColorMode, Self::Err> {
        match input {
            "bw" => Ok(ColorMode::BlackAndWhite),
            "ansi" => Ok(ColorMode::Ansi),
            "truecolor" => Ok(ColorMode::TrueColor),
            _ => Err(()),
        }
    }
}

impl ColorMode {
    pub(crate) fn colorize(&self, color: &Rgba<u8>) -> Option<String> {
        match self {
            ColorMode::BlackAndWhite => None,
            ColorMode::Ansi => Some(ansi(color)),
            ColorMode::TrueColor => Some(truecolor(color)),
        }
    }
}

fn ansi(color: &Rgba<u8>) -> String {
    let ansicolor = rgb_to_ansi256(color[0], color[1], color[2]);
    let string = format!("\x1b[38;5;{}m", ansicolor);
    string
}

fn truecolor(color: &Rgba<u8>) -> String {
    let r = color[0];
    let g = color[1];
    let b = color[2];
    let string = format!("\x1b[38;2;{};{};{}m", r, g, b);
    string
}
