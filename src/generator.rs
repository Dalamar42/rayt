use camera::{CameraSave, Lens};
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use failure::Error;
use rand::prelude::*;
use world::background::Background;
use world::geometry::cube::Cube;
use world::geometry::medium::ConstantMedium;
use world::geometry::rectangle::{XyRect, XzRect, YzRect};
use world::geometry::sphere::{MovingSphere, Sphere};
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::perlin::build_noise_config;
use world::texture::Texture;
use world::WorldSave;

arg_enum! {
    #[derive(Debug)]
    pub enum Scene {
        Basic,
        Cover,
        CoverWithMotionBlur,
        CoverWithChecker,
        Perlin,
        Planets,
        SimpleLight,
        CornellBox,
        CornellSmoke,
    }
}

pub fn build_scene_config(scene: &Scene) -> Result<ConfigSave, Error> {
    match scene {
        Scene::Basic => build_basic_config(),
        Scene::Cover => build_book_cover_config(false, false),
        Scene::CoverWithMotionBlur => build_book_cover_config(true, false),
        Scene::CoverWithChecker => build_book_cover_config(true, true),
        Scene::Perlin => build_perlin_demo_config(),
        Scene::Planets => build_planets_config(),
        Scene::SimpleLight => build_simple_light_config(),
        Scene::CornellBox => build_cornell_box_config(),
        Scene::CornellSmoke => build_cornell_smoke_config(),
    }
}

fn build_basic_config() -> Result<ConfigSave, Error> {
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

fn build_book_cover_config(motion_blur: bool, checker_texture: bool) -> Result<ConfigSave, Error> {
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

    Ok(ConfigSave::new(aspect, camera, world))
}

fn build_perlin_demo_config() -> Result<ConfigSave, Error> {
    let aspect = 1.5;

    let camera = CameraSave::new(
        &Vector::new(13.0, 2.0, 3.0),
        &Vector::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(20.0, 0.0, 10.0),
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

    Ok(ConfigSave::new(aspect, camera, world))
}

fn build_planets_config() -> Result<ConfigSave, Error> {
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

fn build_simple_light_config() -> Result<ConfigSave, Error> {
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

fn build_cornell_box_config() -> Result<ConfigSave, Error> {
    let aspect = 1.0;

    let camera = CameraSave::new(
        &Vector::new(278.0, 278.0, -800.0),
        &Vector::new(278.0, 278.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(40.0, 0.0, 10.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(8);

    let red = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.65, 0.05, 0.05),
        },
    };
    let white = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.73, 0.73, 0.73),
        },
    };
    let green = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.12, 0.45, 0.15),
        },
    };
    let light = Material::DiffuseLight {
        emit: Texture::Constant {
            colour: Colour::new(15.0, 15.0, 15.0),
        },
    };

    geometries.push(Box::new(
        YzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green).flip(),
    ));
    geometries.push(Box::new(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red)));
    geometries.push(Box::new(
        XzRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light).flip(),
    ));
    geometries.push(Box::new(
        XzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()).flip(),
    ));
    geometries.push(Box::new(XzRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    )));
    geometries.push(Box::new(
        XyRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()).flip(),
    ));
    geometries.push(Box::new(
        Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(165.0, 165.0, 165.0),
            white.clone(),
        )
        .rotate_y(-18.0)?
        .translate(Vector::new(130.0, 0.0, 65.0)),
    ));
    geometries.push(Box::new(
        Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(165.0, 330.0, 165.0),
            white,
        )
        .rotate_y(15.0)?
        .translate(Vector::new(265.0, 0.0, 295.0)),
    ));

    let black = Colour::new(0.0, 0.0, 0.0);
    let background = Background::new(black, black);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}

fn build_cornell_smoke_config() -> Result<ConfigSave, Error> {
    let aspect = 1.0;

    let camera = CameraSave::new(
        &Vector::new(278.0, 278.0, -800.0),
        &Vector::new(278.0, 278.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(40.0, 0.0, 10.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(8);

    let red = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.65, 0.05, 0.05),
        },
    };
    let white = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.73, 0.73, 0.73),
        },
    };
    let green = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.12, 0.45, 0.15),
        },
    };
    let light = Material::DiffuseLight {
        emit: Texture::Constant {
            colour: Colour::new(7.0, 7.0, 7.0),
        },
    };

    geometries.push(Box::new(
        YzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green).flip(),
    ));
    geometries.push(Box::new(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red)));
    geometries.push(Box::new(
        XzRect::new((113.0, 443.0), (127.0, 332.0), 554.0, light).flip(),
    ));
    geometries.push(Box::new(
        XzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()).flip(),
    ));
    geometries.push(Box::new(XzRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    )));
    geometries.push(Box::new(
        XyRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()).flip(),
    ));

    let box_boundary_a = Cube::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(165.0, 165.0, 165.0),
        white.clone(),
    )
    .rotate_y(-18.0)?
    .translate(Vector::new(130.0, 0.0, 65.0));
    let box_boundary_b = Cube::new(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(165.0, 330.0, 165.0),
        white,
    )
    .rotate_y(15.0)?
    .translate(Vector::new(265.0, 0.0, 295.0));

    geometries.push(Box::new(ConstantMedium::new(
        Box::new(box_boundary_a),
        0.01,
        Texture::Constant {
            colour: Colour::new(1.0, 1.0, 1.0),
        },
    )));
    geometries.push(Box::new(ConstantMedium::new(
        Box::new(box_boundary_b),
        0.01,
        Texture::Constant {
            colour: Colour::new(0.0, 0.0, 0.0),
        },
    )));

    let black = Colour::new(0.0, 0.0, 0.0);
    let background = Background::new(black, black);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
