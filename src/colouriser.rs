use view::Ray;
use config::Config;
use data::colour::Colour;

pub fn build_colouriser() -> impl Fn(&Ray, &Config) -> Colour {
    colour
}

fn colour(ray: &Ray, config: &Config) -> Colour {
    let maybe_hit_t = config.volumes
        .iter()
        .filter_map(|volume| {
            let maybe_hit_t = volume.hit(&ray, 0.0, core::f64::MAX);
            match maybe_hit_t {
                Some(hit_t) => Some((hit_t, volume)),
                None => None,
            }
        })
        .min_by(|(hit_t_a, _), (hit_t_b, _)| {
            // Should never get a NaN here. Panic if we do
            hit_t_a.partial_cmp(hit_t_b).unwrap()
        });

    match maybe_hit_t {
        Some((hit_t, volume)) => {
            let surface_normal = volume.surface_normal(&ray, hit_t);
            0.5 * Colour {
                r: surface_normal.x + 1.0,
                g: surface_normal.y + 1.0,
                b: surface_normal.z + 1.0,
            }
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
