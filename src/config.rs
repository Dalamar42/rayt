use view::{Camera, CameraSave};
use data::vector::Vector;
use data::colour::Colour;
use world::entity::Entity;
use world::geometry::Sphere;
use world::materials::{Lambertian, Metal, Dielectric};
use world::World;
use rand::prelude::*;

pub struct Config {
    pub width: u64,
    pub height: u64,
    pub camera: Camera,
    pub world: World,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSave {
    pub aspect: f64,
    pub camera: CameraSave,
    pub world: World,
}

impl Config {
    pub fn from_save(save: ConfigSave, width: u64) -> Config {
        Config {
            width,
            height: (width as f64 / save.aspect) as u64,
            camera: Camera::from_save(save.camera),
            world: save.world,
        }
    }
}

pub fn build_book_cover_config() -> ConfigSave {
    let aspect = 1.5;

    let camera = Camera::new(
        &Vector { x: 13.0, y: 2.0, z: 3.0 },
        &Vector { x: 0.0, y: 0.0, z: 0.0 },
        &Vector {x: 0.0, y: 1.0, z: 0.0},
        20.0,
        aspect,
        0.1,
        10.0,
    ).into_save();
    let world = build_book_cover_world();

    ConfigSave {aspect, camera, world}
}

fn build_book_cover_world() -> World {
    let n = 500;
    let mut volumes: Vec<Entity> = Vec::with_capacity(n);

    // Floor
    volumes.push(
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector {x: 0.0, y: -1000.0, z: 0.0},
                radius: 1000.0,
            }),
            material: Box::from(Lambertian {
                albedo: Colour {r: 0.5, g: 0.5, b: 0.5},
            }),
        }
    );

    // 3 big spheres
    volumes.push(
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector {x: 0.0, y: 1.0, z: 0.0},
                radius: 1.0,
            }),
            material: Box::from(Dielectric {
                refractive_index: 1.5,
            }),
        }
    );
    volumes.push(
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector {x: -4.0, y: 1.0, z: 0.0},
                radius: 1.0,
            }),
            material: Box::from(Lambertian {
                albedo: Colour {r: 0.4, g: 0.2, b: 0.1},
            }),
        }
    );
    volumes.push(
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector {x: 4.0, y: 1.0, z: 0.0},
                radius: 1.0,
            }),
            material: Box::from(Metal {
                albedo: Colour {r: 0.7, g: 0.6, b: 0.5},
                fuzz: 0.0,
            }),
        }
    );

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let centre = Vector {
                x: (a as f64) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * rng.gen::<f64>(),
            };

            if (&centre - Vector {x: 4.0, y: 0.2, z: 0.0}).len() > 0.9 {
                if choose_mat < 0.8 {
                    volumes.push(
                        Entity {
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
                        }
                    );
                } else if choose_mat < 0.95 {
                    volumes.push(
                        Entity {
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
                        }
                    );
                } else {
                    volumes.push(
                        Entity {
                            geometry: Box::from(Sphere {
                                centre,
                                radius: 0.2,
                            }),
                            material: Box::from(Dielectric {
                                refractive_index: 1.5,
                            }),
                        }
                    );
                }
            }
        }
    }

    World {volumes}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialise_roundtrip_camera() {
        let camera = Camera::new(
            &Vector { x: 13.0, y: 2.0, z: 3.0 },
            &Vector { x: 0.0, y: 0.0, z: 0.0 },
            &Vector {x: 0.0, y: 1.0, z: 0.0},
            20.0,
            1.5,
            0.1,
            10.0,
        ).into_save();

        let serialised = serde_yaml::to_string(&camera).unwrap();
        let deserialised = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(camera, deserialised);
    }

    #[test]
    fn test_serialise_roundtrip_world() {
        let world = World {
            volumes: vec![
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {x: 0.0, y: 0.0, z: -1.0},
                        radius: 0.5,
                    }),
                    material: Box::from(Lambertian {
                        albedo: Colour {r: 0.1, g: 0.2, b: 0.5},
                    }),
                },
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {x: 0.0, y: -100.5, z: -1.0},
                        radius: 100.0,
                    }),
                    material: Box::from(Lambertian {
                        albedo: Colour {r: 0.8, g: 0.8, b: 0.0},
                    }),
                },
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector { x: 1.0, y: 0.0, z: -1.0 },
                        radius: 0.5,
                    }),
                    material: Box::from(Metal {
                        albedo: Colour {r: 0.8, g: 0.6, b: 0.2},
                        fuzz: 0.1,
                    }),
                },
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector { x: -1.0, y: 0.0, z: -1.0 },
                        radius: -0.45,
                    }),
                    material: Box::from(Dielectric{
                        refractive_index: 1.5,
                    }),
                },
            ]
        };

        let serialised = serde_yaml::to_string(&world).unwrap();
        let deserialised: World = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(world.volumes.len(), deserialised.volumes.len());
    }

    #[test]
    fn test_serialise_roundtrip_saved_config() {
        let camera = Camera::new(
            &Vector { x: 13.0, y: 2.0, z: 3.0 },
            &Vector { x: 0.0, y: 0.0, z: 0.0 },
            &Vector {x: 0.0, y: 1.0, z: 0.0},
            20.0,
            1.5,
            0.1,
            10.0,
        ).into_save();
        let world = World {
            volumes: vec![
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {x: 0.0, y: 0.0, z: -1.0},
                        radius: 0.5,
                    }),
                    material: Box::from(Lambertian {
                        albedo: Colour {r: 0.1, g: 0.2, b: 0.5},
                    }),
                },
            ]
        };
        let saved_config = ConfigSave {
            aspect: 1.5,
            camera,
            world,
        };

        let serialised = serde_yaml::to_string(&saved_config).unwrap();
        serde_yaml::from_str::<ConfigSave>(&serialised).unwrap();
    }
}
