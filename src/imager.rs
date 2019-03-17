use view::Ray;
use config::Config;
use data::image::Image;
use data::colour::Colour;
use rayon::prelude::*;
use indicatif::ProgressBar;

pub fn build_image<T: Sync>(colouriser: T, config: &Config, progress_bar: &ProgressBar) -> Image
    where T: Fn(&Ray, &Config) -> Colour
{
    let pixels: Vec<Colour> = config.camera
        .pixels(&config)
        .par_iter()
        .map(|(row, col)| config.camera.rays(*row, *col, &config))
        .map(|rays| colour(&colouriser, &rays, &config, &progress_bar))
        .collect();

    progress_bar.finish();

    Image{pixels, num_rows: config.height, num_cols: config.width}
}

fn colour<T>(colouriser: &T, rays: &Vec<Ray>, config: &Config, progress_bar: &ProgressBar) -> Colour
    where T: Fn(&Ray, &Config) -> Colour
{
    let colour_sum: Colour = rays.iter()
        .map(|ray| colouriser(&ray, &config))
        .sum();

    progress_bar.inc(1);

    colour_sum / (rays.len() as f64)
}
