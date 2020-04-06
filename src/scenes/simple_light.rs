use camera::{CameraSave, Lens};
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use world::background::Background;
use world::geometry::rectangle::XyRect;
use world::geometry::sphere::Sphere;
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::perlin::build_noise_config;
use world::texture::Texture;
use world::WorldSave;

pub fn build() -> Result<ConfigSave, anyhow::Error> {
    let aspect = 1.5;

    let camera = CameraSave::new(
        &Vector::new(13.0, 3.5, 3.0),
        &Vector::new(0.0, 1.5, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(40.0, 0.0, 10.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(4);

    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Texture::Noise {
                base_colour: Colour::new(1.0, 1.0, 1.0),
                scale: 5.0,
                noisiness: 10.0,
                noise_config: build_noise_config(),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, 2.0, 0.0),
        2.0,
        Material::Lambertian {
            albedo: Texture::Noise {
                base_colour: Colour::new(1.0, 1.0, 1.0),
                scale: 5.0,
                noisiness: 10.0,
                noise_config: build_noise_config(),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, 7.0, 0.0),
        2.0,
        Material::DiffuseLight {
            emit: Texture::Constant {
                colour: Colour::new(4.0, 4.0, 4.0),
            },
        },
    )));
    geometries.push(Box::from(XyRect::new(
        (3.0, 5.0),
        (1.0, 3.0),
        -2.0,
        Material::DiffuseLight {
            emit: Texture::Constant {
                colour: Colour::new(4.0, 4.0, 4.0),
            },
        },
    )));

    let black = Colour::new(0.0, 0.0, 0.0);
    let background = Background::new(black, black);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
