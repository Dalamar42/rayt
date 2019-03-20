use config::Config;
use data::colour::Colour;
use data::image::Image;
use indicatif::ProgressBar;
use rayon::prelude::*;
use view::Ray;

pub fn render(config: &Config, progress_bar: &ProgressBar) -> Image {
    let pixels: Vec<Colour> = config
        .camera
        .pixels(&config)
        .par_iter()
        .map(|(row, col)| config.camera.rays(*row, *col, &config))
        .map(|rays| colour_from_rays(&rays, &config, &progress_bar))
        .map(|colour| colour.gamma_2())
        .collect();

    progress_bar.finish();

    Image {
        pixels,
        num_rows: config.height,
        num_cols: config.width,
    }
}

fn colour_from_rays(rays: &[Ray], config: &Config, progress_bar: &ProgressBar) -> Colour {
    let colour_sum: Colour = rays.iter().map(|ray| colour(&ray, &config, 0)).sum();

    progress_bar.inc(1);

    colour_sum / (rays.len() as f64)
}

fn colour(ray: &Ray, config: &Config, depth: u64) -> Colour {
    if depth >= 50 {
        return Colour {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    let maybe_hit_result = config
        .world
        .volumes
        .iter()
        .filter_map(|volume| {
            let maybe_hit_distance = volume.hit(&ray, 0.001, core::f64::MAX);
            match maybe_hit_distance {
                Some(hit_distance) => Some((hit_distance, volume)),
                None => None,
            }
        })
        .min_by(|(distance_a, _), (distance_b, _)| {
            // Should never get a NaN here. Panic if we do
            distance_a.partial_cmp(distance_b).unwrap()
        })
        .into_iter()
        .filter_map(|(distance, volume)| volume.scatter(&ray, distance))
        .last();

    match maybe_hit_result {
        Some(scatter) => scatter.attenuation * colour(&scatter.ray, &config, depth + 1),
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            linear_interpolation(
                t,
                &config.world.background.bottom,
                &config.world.background.top,
            )
        }
    }
}

fn linear_interpolation(t: f64, colour_a: &Colour, colour_b: &Colour) -> Colour {
    (1.0 - t) * colour_a + t * colour_b
}
