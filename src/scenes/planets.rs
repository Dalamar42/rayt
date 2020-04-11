use crate::camera::{CameraSave, Lens};
use crate::config::ConfigSave;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::world::background::Background;
use crate::world::geometry::sphere::Sphere;
use crate::world::geometry::Geometry;
use crate::world::materials::Material;
use crate::world::texture::Texture;
use crate::world::WorldSave;

pub fn build() -> Result<ConfigSave, anyhow::Error> {
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

    let mut geometries: Vec<Geometry> = Vec::with_capacity(4);

    geometries.push(Sphere::build(
        Vector::new(-1.0, 0.0, -6.0),
        4.0,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("jupiter.jpg"),
            },
        },
    ));
    geometries.push(Sphere::build(
        Vector::new(2.0, 1.0, -1.0),
        1.0,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("earth.jpg"),
            },
        },
    ));
    geometries.push(Sphere::build(
        Vector::new(3.5, 0.7, -1.0),
        0.3,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("moon.jpg"),
            },
        },
    ));
    geometries.push(Sphere::build(
        Vector::new(-1.5, -1.0, -1.0),
        0.8,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("mars.jpg"),
            },
        },
    ));
    geometries.push(Sphere::build(
        Vector::new(1.7, -1.3, -1.0),
        0.7,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("earth_night.jpg"),
            },
        },
    ));

    let background_colour = Colour::new(1.0, 1.0, 1.0);
    let background = Background::new(background_colour, background_colour);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
