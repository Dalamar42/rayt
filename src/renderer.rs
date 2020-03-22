use camera::Ray;
use config::Config;
use data::colour::Colour;
use data::image::{Image, Pixel};
use indicatif::ProgressBar;
use rayon::prelude::*;
use world::geometry::{Geometry, HitResult};

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
    let hit_result = config.bvh().hit(&ray, 0.001, core::f64::MAX);

    match hit_result {
        HitResult::Miss => background(&ray, &config),
        HitResult::Hit {
            ray,
            point,
            surface_normal,
            material,
            texture_coords,
            ..
        } => {
            let emitted = material.emitted(texture_coords, &point, &config.assets());

            if depth >= MAX_SCATTER_DEPTH {
                return emitted;
            }

            let maybe_scatter = material.scatter(
                &ray,
                &point,
                &surface_normal,
                texture_coords,
                &config.assets(),
            );
            match maybe_scatter {
                Some(scatter) => {
                    emitted + scatter.attenuation() * colour(&scatter.ray(), &config, depth + 1)
                }
                None => emitted,
            }
        }
    }
}

fn background(ray: &Ray, config: &Config) -> Colour {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    linear_interpolation(t, &config.background().bottom(), &config.background().top())
}

fn linear_interpolation(t: f64, colour_a: &Colour, colour_b: &Colour) -> Colour {
    (1.0 - t) * colour_a + t * colour_b
}
