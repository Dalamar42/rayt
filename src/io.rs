use std;
use data::image::Image;

const IMAGE_PATH: &str = "image.ppm";
const IMAGE_FORMAT: &str = "P3";
const MAX_COLOUR: u8 = 255;

pub fn write_image_as_ppm(image: Image) -> std::io::Result<()> {
    let mut rows: Vec<String> = vec!();

    rows.push(IMAGE_FORMAT.to_string());
    rows.push(format!("{} {}", image.num_cols, image.num_rows));
    rows.push(MAX_COLOUR.to_string());

    image.pixels.iter()
        .map(|p| format!("{:3} {:3} {:3}", p.r_norm(), p.g_norm(), p.b_norm()))
        .for_each(|fp| rows.push(fp));

    rows.push("".to_string());

    std::fs::write(IMAGE_PATH, rows.join("\n"))?;
    Ok(())
}
