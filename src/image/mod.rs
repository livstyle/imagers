use image::*;

pub fn resize_by_path(source_path: &str, target_path: &str, nwidth: u32, nheight: u32) -> String {
    let img = image::open(source_path).unwrap();
    let resied_img = img.resize(nwidth, nheight, imageops::FilterType::Nearest);
    match resied_img.save(target_path) {
        Ok(res) => "succeed".to_owned(),
        Err(err) => match err {
            ImageError::Decoding(_) => "decoding error".to_owned(),
            ImageError::Parameter(_) => "parameter error".to_owned(),
            ImageError::Limits(_) => "limits error".to_owned(),
            ImageError::Unsupported(_) => "unsupported error".to_owned(),
            ImageError::IoError(_) => "io error".to_owned(),
            ImageError::Encoding(_) => "encoding error".to_owned(),
        }
    }
}