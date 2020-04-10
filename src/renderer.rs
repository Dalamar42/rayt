use crate::camera::Ray;
use crate::config::Config;
use crate::data::colour::Colour;
use crate::data::image::{Image, Pixel};
use crate::world::geometry::Geometry;
use indicatif::ProgressBar;
use rayon::prelude::*;

const MAX_SCATTER_DEPTH: u64 = 50;

pub fn render(config: &Config, progress_bar: &ProgressBar) -> Image {
    let pixels: Vec<Pixel> = config
        .camera()
        .pixels(&config)
        .par_iter()
        .map(|(row, col)| pixel(*row, *col, &config, &progress_bar))
        .collect();

    progress_bar.finish();

    Image::from(&pixels)
}

fn pixel(row: u32, col: u32, config: &Config, progress_bar: &ProgressBar) -> Pixel {
    let rays = config.camera().rays(row, col, &config);

    let colour_sum: Colour = rays.iter().map(|ray| colour(&ray, &config, 0)).sum();
    let colour = colour_sum / (rays.len() as f64);
    let colour = colour.gamma_2();

    progress_bar.inc(1);

    // Translate into the coordinate system expected by the image crate
    Pixel::new(row, col, colour)
}

fn colour(ray: &Ray, config: &Config, depth: u64) -> Colour {
    config
        .bvh()
        .hit(&ray, 0.001, core::f64::MAX)
        .map(|hit| {
            let emitted = hit.material.emitted(
                hit.front_face(),
                hit.texture_coords,
                &hit.point,
                &config.assets(),
            );

            if depth >= MAX_SCATTER_DEPTH {
                return emitted;
            }

            hit.material
                .scatter(
                    &hit.ray,
                    &hit.point,
                    &hit.surface_normal,
                    hit.texture_coords,
                    &config.assets(),
                )
                .map(|scatter| {
                    let scattering_pdf = hit
                        .material
                        .scattering_pdf(&hit.surface_normal, &scatter.ray());
                    let scatter_colour = scatter.albedo()
                        * scattering_pdf
                        * colour(&scatter.ray(), &config, depth + 1)
                        / scatter.pdf();
                    emitted + scatter_colour
                })
                .unwrap_or(emitted)
        })
        .unwrap_or_else(|| background(&ray, &config))
}

fn background(ray: &Ray, config: &Config) -> Colour {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    linear_interpolation(t, &config.background().bottom(), &config.background().top())
}

fn linear_interpolation(t: f64, colour_a: &Colour, colour_b: &Colour) -> Colour {
    (1.0 - t) * colour_a + t * colour_b
}
