use camera::{CameraSave, Lens};
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use failure::Error;
use world::background::Background;
use world::geometry::sphere::Sphere;
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::Texture;
use world::WorldSave;

pub fn build() -> Result<ConfigSave, Error> {
    let aspect = 2.0;

    let camera = CameraSave::new(
        &Vector::new(1.0, 0.2, 7.5),
        &Vector::new(0.0, 0.0, -1.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(45.0, 0.0, 4.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(4);

    geometries.push(Box::from(Sphere::new(
        Vector::new(-1.0, 0.0, -6.0),
        4.0,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("jupiter.jpg"),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(2.0, 1.0, -1.0),
        1.0,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("earth.jpg"),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(3.5, 0.7, -1.0),
        0.3,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("moon.jpg"),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(-1.5, -1.0, -1.0),
        0.8,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("mars.jpg"),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(1.7, -1.3, -1.0),
        0.7,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("earth_night.jpg"),
            },
        },
    )));

    let background_colour = Colour::new(1.0, 1.0, 1.0);
    let background = Background::new(background_colour, background_colour);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
