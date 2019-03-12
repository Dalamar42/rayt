use view::Ray;
use config::Config;
use data::image::Image;
use data::colour::Colour;

pub fn build_image<T>(colouriser: T, config: &Config) -> Image
    where T: Fn(&Ray, &Config) -> Colour
{
    let pixels: Vec<Colour> = config.camera
        .iter(&config)
        .map(|rays| colour_with_anti_aliasing(&colouriser, rays, &config))
        .collect();

    Image{pixels, num_rows: config.height, num_cols: config.width}
}

fn colour_with_anti_aliasing<T>(colouriser: &T, rays: Vec<Ray>, config: &Config) -> Colour
    where T: Fn(&Ray, &Config) -> Colour
{
    let colour_sum: Colour = rays.iter()
        .map(|ray| colouriser(&ray, &config))
        .sum();
    colour_sum / (rays.len() as f64)
}
