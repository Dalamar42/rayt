use camera::Camera;
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use rand::prelude::*;
use world::background::Background;
use world::entity::Entity;
use world::geometry::Sphere;
use world::materials::{Dielectric, Lambertian, Metal};
use world::World;

pub fn build_book_cover_config() -> ConfigSave {
    let aspect = 1.5;

    let camera = Camera::new(
        &Vector {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        &Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        &Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        20.0,
        aspect,
        0.1,
        10.0,
    )
    .into_save();
    let world = build_book_cover_world();

    ConfigSave {
        aspect,
        camera,
        world,
    }
}

fn build_book_cover_world() -> World {
    let n = 500;
    let mut volumes: Vec<Entity> = Vec::with_capacity(n);

    // Floor
    volumes.push(Entity {
        geometry: Box::from(Sphere {
            centre: Vector {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            radius: 1000.0,
        }),
        material: Box::from(Lambertian {
            albedo: Colour {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
        }),
    });

    // 3 big spheres
    volumes.push(Entity {
        geometry: Box::from(Sphere {
            centre: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
        }),
        material: Box::from(Dielectric {
            refractive_index: 1.5,
        }),
    });
    volumes.push(Entity {
        geometry: Box::from(Sphere {
            centre: Vector {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
        }),
        material: Box::from(Lambertian {
            albedo: Colour {
                r: 0.4,
                g: 0.2,
                b: 0.1,
            },
        }),
    });
    volumes.push(Entity {
        geometry: Box::from(Sphere {
            centre: Vector {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
        }),
        material: Box::from(Metal {
            albedo: Colour {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
            fuzz: 0.0,
        }),
    });

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let centre = Vector {
                x: f64::from(a) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: f64::from(b) + 0.9 * rng.gen::<f64>(),
            };

            if (&centre
                - Vector {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .len()
                > 0.9
            {
                if choose_mat < 0.8 {
                    volumes.push(Entity {
                        geometry: Box::from(Sphere {
                            centre,
                            radius: 0.2,
                        }),
                        material: Box::from(Lambertian {
                            albedo: Colour {
                                r: rng.gen::<f64>() * rng.gen::<f64>(),
                                g: rng.gen::<f64>() * rng.gen::<f64>(),
                                b: rng.gen::<f64>() * rng.gen::<f64>(),
                            },
                        }),
                    });
                } else if choose_mat < 0.95 {
                    volumes.push(Entity {
                        geometry: Box::from(Sphere {
                            centre,
                            radius: 0.2,
                        }),
                        material: Box::from(Metal {
                            albedo: Colour {
                                r: 0.5 * (1.0 + rng.gen::<f64>()),
                                g: 0.5 * (1.0 + rng.gen::<f64>()),
                                b: 0.5 * (1.0 + rng.gen::<f64>()),
                            },
                            fuzz: 0.5 * rng.gen::<f64>(),
                        }),
                    });
                } else {
                    volumes.push(Entity {
                        geometry: Box::from(Sphere {
                            centre,
                            radius: 0.2,
                        }),
                        material: Box::from(Dielectric {
                            refractive_index: 1.5,
                        }),
                    });
                }
            }
        }
    }

    let white = Colour {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    let blue = Colour {
        r: 0.5,
        g: 0.7,
        b: 1.0,
    };
    let background = Background {
        bottom: white,
        top: blue,
    };

    World {
        background,
        volumes,
    }
}
