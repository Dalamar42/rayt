use camera::CameraSave;
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use rand::prelude::*;
use world::background::Background;
use world::geometry::sphere::{MovingSphere, Sphere};
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::Texture;
use world::World;

pub fn build_book_cover_config() -> ConfigSave {
    let aspect = 1.5;

    let camera = CameraSave::new(
        &Vector::new(13.0, 2.0, 3.0),
        &Vector::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        20.0,
        aspect,
        0.1,
        10.0,
        0.0,
        1.0,
    );
    let world = build_book_cover_world();

    ConfigSave::new(aspect, camera, world)
}

fn build_book_cover_world() -> World {
    let n = 500;
    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(n);

    // Floor
    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Texture::Checker {
                even: Box::from(Texture::Constant {
                    colour: Colour::new(0.2, 0.3, 0.1),
                }),
                odd: Box::from(Texture::Constant {
                    colour: Colour::new(0.9, 0.9, 0.9),
                }),
            },
        },
    )));

    // 3 big spheres
    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Texture::Constant {
                colour: Colour::new(0.4, 0.2, 0.1),
            },
        },
    )));
    geometries.push(Box::from(Sphere::new(
        Vector::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Colour::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let centre = Vector::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );

            if (&centre - Vector::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    geometries.push(Box::from(MovingSphere::new(
                        centre.clone(),
                        0.0,
                        &centre + Vector::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                        1.0,
                        0.2,
                        Material::Lambertian {
                            albedo: Texture::Constant {
                                colour: Colour::new(
                                    rng.gen::<f64>() * rng.gen::<f64>(),
                                    rng.gen::<f64>() * rng.gen::<f64>(),
                                    rng.gen::<f64>() * rng.gen::<f64>(),
                                ),
                            },
                        },
                    )));
                } else if choose_mat < 0.95 {
                    geometries.push(Box::from(Sphere::new(
                        centre,
                        0.2,
                        Material::Metal {
                            albedo: Colour::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            fuzz: 0.5 * rng.gen::<f64>(),
                        },
                    )));
                } else {
                    geometries.push(Box::from(Sphere::new(
                        centre,
                        0.2,
                        Material::Dielectric {
                            refractive_index: 1.5,
                        },
                    )));
                }
            }
        }
    }

    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);
    let background = Background::new(white, blue);

    World::new(background, geometries)
}
