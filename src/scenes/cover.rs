use crate::camera::{CameraSave, Lens};
use crate::config::ConfigSave;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::sampling::uniform;
use crate::world::background::Background;
use crate::world::geometry::sphere::{MovingSphere, Sphere};
use crate::world::geometry::Geometry;
use crate::world::materials::Material;
use crate::world::texture::Texture;
use crate::world::WorldSave;

pub fn build(motion_blur: bool, checker_texture: bool) -> Result<ConfigSave, anyhow::Error> {
    let aspect = 1.5;

    let camera = CameraSave::new(
        &Vector::new(13.0, 2.0, 3.0),
        &Vector::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(20.0, 0.1, 10.0),
        0.0,
        1.0,
    );

    let n = 500;
    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(n);

    // Floor
    let floor_material = if checker_texture {
        Texture::Checker {
            even: Box::from(Texture::Constant {
                colour: Colour::new(0.2, 0.3, 0.1),
            }),
            odd: Box::from(Texture::Constant {
                colour: Colour::new(0.9, 0.9, 0.9),
            }),
        }
    } else {
        Texture::Constant {
            colour: Colour::new(0.5, 0.5, 0.5),
        }
    };

    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: floor_material,
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

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = uniform();
            let centre = Vector::new(
                f64::from(a) + 0.9 * uniform::<f64>(),
                0.2,
                f64::from(b) + 0.9 * uniform::<f64>(),
            );

            if (centre - Vector::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    if motion_blur {
                        geometries.push(Box::from(MovingSphere::new(
                            centre,
                            0.0,
                            centre + Vector::new(0.0, 0.5 * uniform::<f64>(), 0.0),
                            1.0,
                            0.2,
                            Material::Lambertian {
                                albedo: Texture::Constant {
                                    colour: Colour::new(
                                        uniform::<f64>() * uniform::<f64>(),
                                        uniform::<f64>() * uniform::<f64>(),
                                        uniform::<f64>() * uniform::<f64>(),
                                    ),
                                },
                            },
                        )));
                    } else {
                        geometries.push(Box::from(Sphere::new(
                            centre,
                            0.2,
                            Material::Lambertian {
                                albedo: Texture::Constant {
                                    colour: Colour::new(
                                        uniform::<f64>() * uniform::<f64>(),
                                        uniform::<f64>() * uniform::<f64>(),
                                        uniform::<f64>() * uniform::<f64>(),
                                    ),
                                },
                            },
                        )));
                    }
                } else if choose_mat < 0.95 {
                    geometries.push(Box::from(Sphere::new(
                        centre,
                        0.2,
                        Material::Metal {
                            albedo: Colour::new(
                                0.5 * (1.0 + uniform::<f64>()),
                                0.5 * (1.0 + uniform::<f64>()),
                                0.5 * (1.0 + uniform::<f64>()),
                            ),
                            fuzz: 0.5 * uniform::<f64>(),
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

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
