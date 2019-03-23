use config::ConfigSave;
use failure::Error;
use image::RgbImage;
use std;

pub const SUPPORTED_IMAGE_EXT: [&str; 4] = [".ppm", ".jpeg", ".jpg", ".png"];

pub fn write_image(image: RgbImage, output_path: &str) -> std::io::Result<()> {
    image.save(output_path)
}

pub fn save_config(config_path: &str, config_save: ConfigSave) -> Result<(), Error> {
    let serialised = serde_yaml::to_string(&config_save)?;
    std::fs::write(config_path, serialised)?;
    Ok(())
}

pub fn load_config(config_path: &str) -> Result<ConfigSave, Error> {
    let read = std::fs::read_to_string(config_path)?;
    let deserialised: ConfigSave = serde_yaml::from_str(&read)?;
    Ok(deserialised)
}
