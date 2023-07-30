use crate::color::ColorMode;
use crate::pixel;

pub(crate) struct Params {
    pub(crate) filename: String,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) color_mode: ColorMode,
    pub(crate) pixels: String,
    pub(crate) background_color: String,
    pub(crate) char_size_ratio: f32,
    pub(crate) inverted: bool,
}

/// the char size ratio (because chars are usually not squares)
const DEFAULT_CHAR_SIZE_RATIO: f32 = 2.05;

impl Params {
    pub(crate) fn new() -> Params {
        Params {
            filename: "".to_string(),
            width: 150,
            height: 0,
            color_mode: ColorMode::Ansi,
            background_color: "".to_string(),
            pixels: pixel::ASCII.to_string(),
            char_size_ratio: DEFAULT_CHAR_SIZE_RATIO,
            inverted: false,
        }
    }
}