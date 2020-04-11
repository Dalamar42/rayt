use crate::camera::{Camera, CameraSave};
use crate::data::assets::Assets;
use crate::world::background::Background;
use crate::world::geometry::bounding_volume_hierarchy::BoundingVolumeHierarchyNode;
use crate::world::geometry::{Geometry, Hittable};
use crate::world::WorldSave;

pub struct Config {
    width: u32,
    height: u32,
    camera: Camera,
    background: Background,
    bvh: Geometry,
    attractors: Vec<Geometry>,
    num_of_rays: u64,
    assets: Assets,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSave {
    aspect: f64,
    camera: CameraSave,
    world: WorldSave,
}

impl Config {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn num_of_rays(&self) -> u64 {
        self.num_of_rays
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn bvh(&self) -> &Geometry {
        &self.bvh
    }

    pub fn attractors(&self) -> Vec<Geometry> {
        // TODO Remove clone
        self.attractors.to_vec()
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }
}

impl ConfigSave {
    pub fn new(aspect: f64, camera: CameraSave, world: WorldSave) -> ConfigSave {
        ConfigSave {
            aspect,
            camera,
            world,
        }
    }

    pub fn into_config(mut self, width: u32, num_of_rays: u64, assets: Assets) -> Config {
        let camera = self.camera.into_camera();

        let time_start = camera.time_start();
        let time_end = camera.time_end();

        let geometries = self.world.drain_geometries();

        let attractors = geometries
            .iter()
            .filter(|g| g.is_attractor())
            .cloned()
            .collect();

        let bvh = BoundingVolumeHierarchyNode::build(geometries, time_start, time_end);

        Config {
            width,
            height: (f64::from(width) / self.aspect) as u32,
            camera,
            background: self.world.background().clone(),
            bvh,
            attractors,
            num_of_rays,
            assets,
        }
    }

    pub fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        self.world.validate(assets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Lens;
    use crate::data::colour::Colour;
    use crate::data::vector::Vector;
    use crate::world::background::Background;
    use crate::world::geometry::sphere::Sphere;
    use crate::world::materials::Material;
    use crate::world::texture::Texture;

    #[test]
    fn test_serialise_roundtrip_camera() {
        let camera = CameraSave::new(
            &Vector::new(13.0, 2.0, 3.0),
            &Vector::new(0.0, 0.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
            1.5,
            Lens::new(20.0, 0.1, 10.0),
            0.0,
            1.0,
        );

        let serialised = serde_yaml::to_string(&camera).unwrap();
        let deserialised = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(camera, deserialised);
    }

    #[test]
    fn test_serialise_roundtrip_world() {
        let mut world = WorldSave::new(
            Background::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.0, 0.0)),
            vec![
                Sphere::build(
                    Vector::new(0.0, 0.0, -1.0),
                    0.5,
                    Material::Lambertian {
                        albedo: Texture::Constant {
                            colour: Colour::new(0.1, 0.2, 0.5),
                        },
                    },
                ),
                Sphere::build(
                    Vector::new(0.0, -100.5, -1.0),
                    100.0,
                    Material::Lambertian {
                        albedo: Texture::Constant {
                            colour: Colour::new(0.8, 0.8, 0.0),
                        },
                    },
                ),
                Sphere::build(
                    Vector::new(1.0, 0.0, -1.0),
                    0.5,
                    Material::Metal {
                        albedo: Colour::new(0.8, 0.6, 0.2),
                        fuzz: 0.1,
                    },
                ),
                Sphere::build(
                    Vector::new(-1.0, 0.0, -1.0),
                    -0.45,
                    Material::Dielectric {
                        refractive_index: 1.5,
                    },
                ),
            ],
        );

        let serialised = serde_yaml::to_string(&world).unwrap();
        let mut deserialised: WorldSave = serde_yaml::from_str(&serialised).unwrap();

        assert_eq!(
            world.drain_geometries().len(),
            deserialised.drain_geometries().len()
        );
    }

    #[test]
    fn test_serialise_roundtrip_saved_config() {
        let camera = CameraSave::new(
            &Vector::new(13.0, 2.0, 3.0),
            &Vector::new(0.0, 0.0, 0.0),
            &Vector::new(0.0, 1.0, 0.0),
            1.5,
            Lens::new(20.0, 0.1, 10.0),
            0.0,
            1.0,
        );
        let world = WorldSave::new(
            Background::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.0, 0.0)),
            vec![Sphere::build(
                Vector::new(0.0, 0.0, -1.0),
                0.5,
                Material::Lambertian {
                    albedo: Texture::Constant {
                        colour: Colour::new(0.1, 0.2, 0.5),
                    },
                },
            )],
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
