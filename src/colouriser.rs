use view::Ray;
use config::Config;
use data::colour::Colour;

pub fn build_colouriser() -> impl Fn(&Ray, &Config) -> Colour {
    |ray, config| colour(&ray, &config, 0)
}

fn colour(ray: &Ray, config: &Config, depth: u64) -> Colour {
    if depth >= 50 {
        return Colour {r: 0.0, g: 0.0, b: 0.0}
    }

    let maybe_hit_result = config.volumes
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
        .filter_map(|(distance, volume)| {
            volume.scatter(&ray, distance)
        })
        .last();

    match maybe_hit_result {
        Some(scatter) => {
            scatter.attenuation * colour(&scatter.ray, &config, depth + 1)
        },
        None => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            let white = Colour {r: 1.0, g: 1.0, b: 1.0};
            let blue = Colour {r: 0.5, g: 0.7, b: 1.0};

            linear_interpolation(t, white, blue)
        },
    }
}

fn linear_interpolation(t: f64, colour_a: Colour, colour_b: Colour) -> Colour {
    (1.0 - t) * colour_a + t * colour_b
}
