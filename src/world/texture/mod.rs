pub mod perlin;

use data::colour::Colour;
use data::vector::Vector;
use world::texture::perlin::{perlin_turbulence, NoiseConfig};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Texture {
    Constant {
        colour: Colour,
    },
    Checker {
        even: Box<Texture>,
        odd: Box<Texture>,
    },
    Noise {
        base_colour: Colour,
        scale: f64,
        noisiness: f64,
        noise_config: NoiseConfig,
    },
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, point: &Vector) -> Colour {
        match self {
            Texture::Constant { colour } => *colour,
            Texture::Checker { odd, even } => checker_texture(&odd, &even, u, v, &point),
            Texture::Noise {
                base_colour,
                scale,
                noisiness,
                noise_config,
            } => noise_texture(&base_colour, *scale, *noisiness, &noise_config, &point),
        }
    }
}

fn checker_texture(odd: &Texture, even: &Texture, u: f64, v: f64, point: &Vector) -> Colour {
    let sines =
        f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
    if sines < 0.0 {
        odd.value(u, v, &point)
    } else {
        even.value(u, v, &point)
    }
}

fn noise_texture(
    base_colour: &Colour,
    scale: f64,
    noisiness: f64,
    noise_config: &NoiseConfig,
    point: &Vector,
) -> Colour {
    let noise = perlin_turbulence(&noise_config, &point, 7);
    let mult = 0.5 * (1.0 + f64::sin(scale * point.z() + noisiness * noise));

    mult * base_colour
}
