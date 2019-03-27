pub mod perlin;

use data::colour::Colour;
use data::vector::Vector;
use world::texture::perlin::{NoiseConfig, perlin_turbulence};

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
    pub fn value(&self, u: f64, v: f64, p: &Vector) -> Colour {
        match self {
            Texture::Constant { colour } => colour.clone(),
            Texture::Checker { odd, even } => checker_texture(&odd, &even, u, v, &p),
            Texture::Noise {
                base_colour,
                scale,
                noisiness,
                noise_config,
            } => noise_texture(&base_colour, *scale, *noisiness, &noise_config, &p),
        }
    }
}

fn checker_texture(odd: &Texture, even: &Texture, u: f64, v: f64, p: &Vector) -> Colour {
    let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
    if sines < 0.0 {
        odd.value(u, v, &p)
    } else {
        even.value(u, v, &p)
    }
}

fn noise_texture(
    base_colour: &Colour,
    scale: f64,
    noisiness: f64,
    noise_config: &NoiseConfig,
    p: &Vector,
) -> Colour {
    let noise = perlin_turbulence(&noise_config, &p, 7);
    let mult = 0.5 * (1.0 + f64::sin(scale * p.z() + noisiness * noise));

    mult * base_colour
}
