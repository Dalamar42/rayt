use config::ConfigSave;
use data::colour::MAX_COLOUR;
use failure::Error;
use renderer::Image;
use std;

const IMAGE_FORMAT: &str = "P3";

pub fn write_image_as_ppm(image: Image, output_path: &str) -> std::io::Result<()> {
    let mut rows: Vec<String> = vec![];

    rows.push(IMAGE_FORMAT.to_string());
    rows.push(format!("{} {}", image.num_cols, image.num_rows));
    rows.push(MAX_COLOUR.to_string());

    image
        .pixels
        .iter()
        .map(|p| format!("{:3} {:3} {:3}", p.r_norm(), p.g_norm(), p.b_norm()))
        .for_each(|fp| rows.push(fp));

    rows.push("".to_string());

    std::fs::write(output_path, rows.join("\n"))?;
    Ok(())
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
