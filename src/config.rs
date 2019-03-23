use camera::{Camera, CameraSave};
use world::World;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub world: World,
    pub num_of_rays: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSave {
    pub aspect: f64,
    pub camera: CameraSave,
    pub world: World,
}

impl Config {
    pub fn from_save(save: ConfigSave, width: u32, num_of_rays: u64) -> Config {
        Config {
            width,
            height: (f64::from(width) / save.aspect) as u32,
            camera: Camera::from_save(save.camera),
            world: save.world,
            num_of_rays,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::colour::Colour;
    use data::vector::Vector;
    use world::background::Background;
    use world::entity::Entity;
    use world::geometry::Sphere;
    use world::materials::{Dielectric, Lambertian, Metal};

    #[test]
    fn test_serialise_roundtrip_camera() {
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
            1.5,
            0.1,
            10.0,
        )
        .into_save();

        let serialised = serde_yaml::to_string(&camera).unwrap();
        let deserialised = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(camera, deserialised);
    }

    #[test]
    fn test_serialise_roundtrip_world() {
        let world = World {
            background: Background {
                bottom: Colour {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
                top: Colour {
                    r: 0.5,
                    g: 0.0,
                    b: 0.0,
                },
            },
            volumes: vec![
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {
                            x: 0.0,
                            y: 0.0,
                            z: -1.0,
                        },
                        radius: 0.5,
                    }),
                    material: Box::from(Lambertian {
                        albedo: Colour {
                            r: 0.1,
                            g: 0.2,
                            b: 0.5,
                        },
                    }),
                },
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {
                            x: 0.0,
                            y: -100.5,
                            z: -1.0,
                        },
                        radius: 100.0,
                    }),
                    material: Box::from(Lambertian {
                        albedo: Colour {
                            r: 0.8,
                            g: 0.8,
                            b: 0.0,
                        },
                    }),
                },
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {
                            x: 1.0,
                            y: 0.0,
                            z: -1.0,
                        },
                        radius: 0.5,
                    }),
                    material: Box::from(Metal {
                        albedo: Colour {
                            r: 0.8,
                            g: 0.6,
                            b: 0.2,
                        },
                        fuzz: 0.1,
                    }),
                },
                Entity {
                    geometry: Box::from(Sphere {
                        centre: Vector {
                            x: -1.0,
                            y: 0.0,
                            z: -1.0,
                        },
                        radius: -0.45,
                    }),
                    material: Box::from(Dielectric {
                        refractive_index: 1.5,
                    }),
                },
            ],
        };

        let serialised = serde_yaml::to_string(&world).unwrap();
        let deserialised: World = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(world.volumes.len(), deserialised.volumes.len());
    }

    #[test]
    fn test_serialise_roundtrip_saved_config() {
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
            1.5,
            0.1,
            10.0,
        )
        .into_save();
        let world = World {
            background: Background {
                bottom: Colour {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
                top: Colour {
                    r: 0.5,
                    g: 0.0,
                    b: 0.0,
                },
            },
            volumes: vec![Entity {
                geometry: Box::from(Sphere {
                    centre: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
                    radius: 0.5,
                }),
                material: Box::from(Lambertian {
                    albedo: Colour {
                        r: 0.1,
                        g: 0.2,
                        b: 0.5,
                    },
                }),
            }],
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
