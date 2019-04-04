use camera::CameraSave;
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use rand::prelude::*;
use std::str::FromStr;
use world::background::Background;
use world::geometry::sphere::{MovingSphere, Sphere};
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::perlin::build_noise_config;
use world::texture::Texture;
use world::WorldSave;

pub enum Scene {
    Basic,
    Cover,
    CoverWithMotionBlur,
    CoverWithChecker,
    Perlin,
    Earth,
}

impl FromStr for Scene {
    type Err = ();

    fn from_str(scene: &str) -> Result<Scene, ()> {
        match scene {
            "basic" => Ok(Scene::Basic),
            "cover" => Ok(Scene::Cover),
            "cover_with_motion_blur" => Ok(Scene::CoverWithMotionBlur),
            "cover_with_checker" => Ok(Scene::CoverWithChecker),
            "perlin" => Ok(Scene::Perlin),
            "earth" => Ok(Scene::Earth),
            _ => Err(()),
        }
    }
}

impl ToString for Scene {
    fn to_string(&self) -> String {
        match self {
            Scene::Basic => String::from("basic"),
            Scene::Cover => String::from("cover"),
            Scene::CoverWithMotionBlur => String::from("cover_with_motion_blur"),
            Scene::CoverWithChecker => String::from("cover_with_checker"),
            Scene::Perlin => String::from("perlin"),
            Scene::Earth => String::from("earth"),
        }
    }
}

pub fn build_scene_config(scene: &Scene) -> ConfigSave {
    match scene {
        Scene::Basic => build_basic_config(),
        Scene::Cover => build_book_cover_config(false, false),
        Scene::CoverWithMotionBlur => build_book_cover_config(true, false),
        Scene::CoverWithChecker => build_book_cover_config(true, true),
        Scene::Perlin => build_perlin_demo_config(),
        Scene::Earth => build_earth_config(),
    }
}

fn build_basic_config() -> ConfigSave {
    let aspect = 2.0;

    let camera = CameraSave::new(
        &Vector::new(0.0, 0.2, 3.0),
        &Vector::new(0.0, 0.0, -1.0),
        &Vector::new(0.0, 1.0, 0.0),
        35.0,
        aspect,
        0.1,
        4.0,
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

    ConfigSave::new(aspect, camera, world)
}

fn build_book_cover_config(motion_blur: bool, checker_texture: bool) -> ConfigSave {
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

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let centre = Vector::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );

            if (centre - Vector::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    if motion_blur {
                        geometries.push(Box::from(MovingSphere::new(
                            centre,
                            0.0,
                            centre + Vector::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
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
                    } else {
                        geometries.push(Box::from(Sphere::new(
                            centre,
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
                    }
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

    let world = WorldSave::new(background, geometries);

    ConfigSave::new(aspect, camera, world)
}

fn build_perlin_demo_config() -> ConfigSave {
    let aspect = 1.5;

    let camera = CameraSave::new(
        &Vector::new(13.0, 2.0, 3.0),
        &Vector::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        20.0,
        aspect,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(2);

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

    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);
    let background = Background::new(white, blue);

    let world = WorldSave::new(background, geometries);

    ConfigSave::new(aspect, camera, world)
}

fn build_earth_config() -> ConfigSave {
    let aspect = 2.0;

    let camera = CameraSave::new(
        &Vector::new(0.0, 0.2, 3.0),
        &Vector::new(0.0, 0.0, -1.0),
        &Vector::new(0.0, 1.0, 0.0),
        35.0,
        aspect,
        0.1,
        4.0,
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(4);

    geometries.push(Box::from(Sphere::new(
        Vector::new(0.0, 0.0, -1.0),
        1.0,
        Material::Lambertian {
            albedo: Texture::Image {
                asset_name: String::from("earth.jpg"),
            },
        },
    )));

    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);
    let background = Background::new(blue, white);

    let world = WorldSave::new(background, geometries);

    ConfigSave::new(aspect, camera, world)
}
