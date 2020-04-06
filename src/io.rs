use cli::{ConfigPath, ImagePath, OutputPath};
use config::ConfigSave;
use data::image::Image;
use std;

pub const SUPPORTED_IMAGE_EXT: [&str; 4] = [".ppm", ".jpeg", ".jpg", ".png"];

pub fn write_image(image: Image, output_path: &OutputPath) -> std::io::Result<()> {
    image.into_rgb_image().save(output_path.path())
}

pub fn load_image(image_path: &ImagePath) -> Result<Image, anyhow::Error> {
    let image = image::open(image_path.path())?;
    Ok(Image::from(&image))
}

pub fn save_config(config_path: &ConfigPath, config_save: ConfigSave) -> Result<(), anyhow::Error> {
    let serialised = serde_yaml::to_string(&config_save)?;
    std::fs::write(config_path.path(), serialised)?;
    Ok(())
}

pub fn load_config(config_path: &ConfigPath) -> Result<ConfigSave, anyhow::Error> {
    let read = std::fs::read_to_string(config_path.path())?;
    let deserialised: ConfigSave = serde_yaml::from_str(&read)?;
    Ok(deserialised)
}
