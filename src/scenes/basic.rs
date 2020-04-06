use camera::{CameraSave, Lens};
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use world::background::Background;
use world::geometry::sphere::Sphere;
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::Texture;
use world::WorldSave;

pub fn build() -> Result<ConfigSave, anyhow::Error> {
    let aspect = 2.0;

    let camera = CameraSave::new(
        &Vector::new(0.0, 0.2, 3.0),
        &Vector::new(0.0, 0.0, -1.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(35.0, 0.1, 4.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(4);

    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian {
            albedo: Texture::Constant {
                colour: Colour::new(0.1, 0.2, 0.5),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Texture::Constant {
                colour: Colour::new(0.8, 0.8, 0.0),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Colour::new(0.8, 0.6, 0.2),
            fuzz: 0.1,
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    )));

    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);
    let background = Background::new(blue, white);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
