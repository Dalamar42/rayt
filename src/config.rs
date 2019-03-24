use camera::{Camera, CameraSave};
use world::World;

pub struct Config {
    width: u32,
    height: u32,
    camera: Camera,
    world: World,
    num_of_rays: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSave {
    aspect: f64,
    camera: CameraSave,
    world: World,
}

impl Config {
    pub fn from_save(save: ConfigSave, width: u32, num_of_rays: u64) -> Config {
        Config {
            width,
            height: (f64::from(width) / save.aspect) as u32,
            camera: save.camera.into_camera(),
            world: save.world,
            num_of_rays,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn num_of_rays(&self) -> u64 {
        self.num_of_rays
    }
}

impl ConfigSave {
    pub fn new(aspect: f64, camera: CameraSave, world: World) -> ConfigSave {
        ConfigSave {
            aspect,
            camera,
            world,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data::colour::Colour;
    use data::vector::Vector;
    use world::background::Background;
    use world::geometry::sphere::Sphere;
    use world::materials::Material;

    #[test]
    fn test_serialise_roundtrip_camera() {
        let camera = CameraSave::new(
            &Vector::new(13.0, 2.0, 3.0),
            &Vector::new(0.0, 0.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
            20.0,
            1.5,
            0.1,
            10.0,
            0.0,
            1.0,
        );

        let serialised = serde_yaml::to_string(&camera).unwrap();
        let deserialised = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(camera, deserialised);
    }

    #[test]
    fn test_serialise_roundtrip_world() {
        let world = World::new(
            Background::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.0, 0.0)),
            vec![
                Box::from(Sphere::new(
                    Vector::new(0.0, 0.0, -1.0),
                    0.5,
                    Material::Lambertian {
                        albedo: Colour::new(0.1, 0.2, 0.5),
                    },
                )),
                Box::from(Sphere::new(
                    Vector::new(0.0, -100.5, -1.0),
                    100.0,
                    Material::Lambertian {
                        albedo: Colour::new(0.8, 0.8, 0.0),
                    },
                )),
                Box::from(Sphere::new(
                    Vector::new(1.0, 0.0, -1.0),
                    0.5,
                    Material::Metal {
                        albedo: Colour::new(0.8, 0.6, 0.2),
                        fuzz: 0.1,
                    },
                )),
                Box::from(Sphere::new(
                    Vector::new(-1.0, 0.0, -1.0),
                    -0.45,
                    Material::Dielectric {
                        refractive_index: 1.5,
                    },
                )),
            ],
        );

        let serialised = serde_yaml::to_string(&world).unwrap();
        let deserialised: World = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(world.geometries().len(), deserialised.geometries().len());
    }

    #[test]
    fn test_serialise_roundtrip_saved_config() {
        let camera = CameraSave::new(
            &Vector::new(13.0, 2.0, 3.0),
            &Vector::new(0.0, 0.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
            20.0,
            1.5,
            0.1,
            10.0,
            0.0,
            1.0,
        );
        let world = World::new(
            Background::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.0, 0.0)),
            vec![Box::from(Sphere::new(
                Vector::new(0.0, 0.0, -1.0),
                0.5,
                Material::Lambertian {
                    albedo: Colour::new(0.1, 0.2, 0.5),
                },
            ))],
        );
        let saved_config = ConfigSave {
            aspect: 1.5,
            camera,
            world,
        };

        let serialised = serde_yaml::to_string(&saved_config).unwrap();
        serde_yaml::from_str::<ConfigSave>(&serialised).unwrap();
    }
}
