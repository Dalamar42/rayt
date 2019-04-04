pub mod perlin;

use data::assets::Assets;
use data::colour::Colour;
use data::image::Image;
use data::vector::Vector;
use failure::Error;
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
    Image {
        asset_name: String,
    },
}

impl Texture {
    pub fn value(&self, texture_coords: (f64, f64), point: &Vector, assets: &Assets) -> Colour {
        match self {
            Texture::Constant { colour } => *colour,
            Texture::Checker { odd, even } => {
                checker_texture(&odd, &even, texture_coords, &point, &assets)
            }
            Texture::Noise {
                base_colour,
                scale,
                noisiness,
                noise_config,
            } => noise_texture(&base_colour, *scale, *noisiness, &noise_config, &point),
            Texture::Image { asset_name } => {
                let image = assets.get_asset(asset_name);
                image_texture(image, texture_coords)
            }
        }
    }

    pub fn validate(&self, assets: &Assets) -> Result<(), Error> {
        match self {
            Texture::Image { asset_name } => {
                assets.validate(&asset_name)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn checker_texture(
    odd: &Texture,
    even: &Texture,
    texture_coords: (f64, f64),
    point: &Vector,
    assets: &Assets,
) -> Colour {
    let sines =
        f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
    if sines < 0.0 {
        odd.value(texture_coords, &point, &assets)
    } else {
        even.value(texture_coords, &point, &assets)
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

fn image_texture(image: &Image, texture_coords: (f64, f64)) -> Colour {
    let height = image.height() as f64;
    let width = image.width() as f64;

    let row = texture_coords.0 * (height - 1.0);
    let col = texture_coords.1 * (width - 1.0);

    let row = f64::max(0.0, row);
    let col = f64::max(0.0, col);

    let row = f64::min(height - 1.0, row);
    let col = f64::min(width - 1.0, col);

    *image.get_pixel(row as u32, col as u32)
}
