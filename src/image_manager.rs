use image::GenericImageView;

pub(crate) fn open_image(filename: &str, width: u16, height: u16, char_size_ratio: f32) -> Result<image::DynamicImage, String> {
    let image = image::open(filename);

    if image.is_err() {
        return Err(format!("open image file {} failed", filename));
    }
    let image = image.unwrap();
    if width == 0 && height == 0 {
        return Ok(image);
    }
    let (image_width, image_height) = image.dimensions();
    let image_width = image_width as f32;
    let image_height = image_height as f32;
    if width == 0 {
        // width has to be computed depending on height
        let ratio = height as f32 / image_height;
        let width = (char_size_ratio * image_width * ratio) as u16;
        return Ok(image.resize(width as u32, height as u32, image::imageops::FilterType::Lanczos3));
    }

    if height == 0 {
        // height has to be computed depending on width
        let ratio = width as f32 / image_width;
        let _height = (image_height * ratio / char_size_ratio).floor() as u32;
        let _width = u32::from(width);
        return Ok(image.resize_exact(_width, _height, image::imageops::FilterType::Lanczos3));
    }

    let image = image.resize(width as u32, height as u32, image::imageops::FilterType::Lanczos3);
    Ok(image)
}
