use std::str::FromStr;

pub(crate) const ASCII: &str = ".,:;i1tfLCG08@";
const ASCII2: &str = ".:-=+*#%@";
const UNICODE: &str = "▢▣▤▥▦▧▨▩";

pub(crate) enum PixelMode {
    ASCII,
    ASCII2,
    UNICODE,
}

impl FromStr for PixelMode {
    type Err = ();

    fn from_str(input: &str) -> Result<PixelMode, Self::Err> {
        match input {
            "ascii" => Ok(PixelMode::ASCII),
            "ascii2" => Ok(PixelMode::ASCII2),
            "unicode" => Ok(PixelMode::UNICODE),
            _ => Err(()),
        }
    }
}

impl PixelMode {
    pub(crate) fn get_pixels(&self) -> &'static str {
        match self {
            PixelMode::ASCII => ASCII,
            PixelMode::ASCII2 => ASCII2,
            PixelMode::UNICODE => UNICODE,
        }
    }
}
