use crate::camera::Ray;
use crate::config::Config;
use crate::data::colour::Colour;
use crate::data::image::{Image, Pixel};
use crate::pdf::Pdf;
use crate::world::geometry::{HitResult, Hittable};
use crate::world::materials::ScatterResult;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_SCATTER_DEPTH: u64 = 50;

pub struct RenderOutput {
    pub image: Image,
    pub failed_rays: usize,
}

pub fn render(config: &Config, progress_bar: &ProgressBar) -> RenderOutput {
    let failed_rays = AtomicUsize::new(0);
    let mut pixel_coords = config.camera().pixels(&config);

    // When pixels are processed in order patterns in the image can affect the remaining time
    // estimate produced by the progress bar, e.g. a empty part of the image can be processed first
    // thus producing a false low remaining time estimate while the rest of the image might have a
    // very high number of objects and be slower to process.
    // Shuffle pixels to break up these patterns and improve the quality of the estimate
    let mut rng = rand::thread_rng();
    pixel_coords.shuffle(&mut rng);

    let pixels: Vec<Pixel> = pixel_coords
        .par_iter()
        .map(|(row, col)| pixel(*row, *col, &config, &progress_bar, &failed_rays))
        .collect();

    progress_bar.finish();

    let image = Image::from(&pixels);
    RenderOutput {
        image,
        failed_rays: failed_rays.load(Ordering::SeqCst),
    }
}

fn pixel(
    row: u32,
    col: u32,
    config: &Config,
    progress_bar: &ProgressBar,
    failed_rays: &AtomicUsize,
) -> Pixel {
    let rays = config.camera().rays(row, col, &config);

    let colour_sum = panic::catch_unwind(|| {
        rays.iter()
            .map(|ray| colour(&ray, &config, 0, failed_rays))
            .sum()
    });
    let colour_sum: Colour = match colour_sum {
        Ok(colour_sum) => colour_sum,
        Err(err) => {
            // A rayon parallel iter will not terminate other threads when one panics
            eprintln!("A rendering thread panicked {:?}", err);
            std::process::exit(1);
        }
    };

    let colour = colour_sum / (rays.len() as f64);
    let colour = colour.gamma_2();

    progress_bar.inc(1);

    // Translate into the coordinate system expected by the image crate
    Pixel::new(row, col, colour)
}

fn colour(ray: &Ray, config: &Config, depth: u64, failed_rays: &AtomicUsize) -> Colour {
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
                .scatter(&hit, &config.assets())
                .map(|scatter| {
                    colour_from_scatter(config, depth, &hit, emitted, scatter, failed_rays)
                })
                .unwrap_or(emitted)
        })
        .unwrap_or_else(|| background(&ray, &config))
}

fn colour_from_scatter(
    config: &Config,
    depth: u64,
    hit: &HitResult,
    emitted: Colour,
    scatter: ScatterResult,
    failed_rays: &AtomicUsize,
) -> Colour {
    match scatter {
        ScatterResult::Specular { attenuation, ray } => {
            emitted + attenuation * colour(&ray, &config, depth + 1, failed_rays)
        }
        ScatterResult::Diffuse { attenuation, pdf } => {
            let attractors = config.attractors();
            let pdf = if attractors.is_empty() {
                pdf
            } else {
                Pdf::Mixture(
                    pdf.boxed(),
                    Pdf::Geometry {
                        geometries: &attractors,
                        origin: hit.point,
                    }
                    .boxed(),
                )
            };

            let direction = pdf.generate();
            let pdf_value = pdf.value(&direction);

            if pdf_value <= 0.0 {
                // This means there is no valid scattered ray we should sample.
                // Return just emitted to avoid a NaN from the division by 0
                failed_rays.fetch_add(1, Ordering::SeqCst);
                return emitted;
            }

            let scattered = Ray::new(hit.point, direction, hit.ray.time());
            let scattering_pdf = hit.material.scattering_pdf(&hit.face_normal(), &scattered);
            let scatter_colour =
                attenuation * scattering_pdf * colour(&scattered, &config, depth + 1, failed_rays)
                    / pdf_value;
            emitted + scatter_colour
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
