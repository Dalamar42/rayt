use camera::Ray;
use data::assets::Assets;
use data::vector::Vector;
use failure::Error;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::geometry::{Geometry, HitResult};

#[derive(Serialize, Deserialize)]
pub struct Translate {
    geometry: Box<dyn Geometry>,
    offset: Vector,
}

impl Translate {
    pub fn new(geometry: Box<dyn Geometry>, offset: Vector) -> Translate {
        Translate { geometry, offset }
    }
}

#[typetag::serde]
impl Geometry for Translate {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult {
        let moved_ray = ray.offset(self.offset);
        match self.geometry.hit(&moved_ray, tmin, tmax) {
            HitResult::Miss => HitResult::Miss,
            HitResult::Hit {
                distance,
                ray,
                point,
                surface_normal,
                material,
                texture_coords,
            } => HitResult::Hit {
                distance,
                ray,
                point: point + self.offset,
                surface_normal,
                material,
                texture_coords,
            },
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        match self.geometry.bounding_box(time_start, time_end) {
            None => None,
            Some(bounding_box) => Some(AxisAlignedBoundingBox::new(
                bounding_box.min() + self.offset,
                bounding_box.max() + self.offset,
            )),
        }
    }

    fn validate(&self, assets: &Assets) -> Result<(), Error> {
        self.geometry.validate(assets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use world::geometry::cube::Cube;
    use world::materials::Material;

    #[test]
    fn test_translate_hit() {
        let cube = Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        )
        .translate(Vector::new(1.0, 0.2, 0.0));

        let ray = Ray::new(Vector::new(3.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX);
        match hit_result {
            HitResult::Hit { distance, .. } => {
                assert_approx_eq!(distance, 1.0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_translate_surface_normal() {
        let cube = Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        )
        .translate(Vector::new(1.0, 0.2, 0.0));

        let ray = Ray::new(Vector::new(3.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX);
        match hit_result {
            HitResult::Hit { surface_normal, .. } => {
                assert_eq!(surface_normal, Vector::new(1.0, 0.0, 0.0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_translate_bounding_box() {
        let cube = Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        )
        .translate(Vector::new(1.0, 0.2, 0.0));

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(1.0, 0.2, 0.0), Vector::new(2.0, 1.2, 1.0));

        assert_eq!(cube.bounding_box(0.0, 0.0), Some(expected_box));
    }

    #[test]
    fn test_translate_texture_coords() {
        let cube = Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        )
        .translate(Vector::new(1.0, 0.2, 0.0));

        let ray = Ray::new(Vector::new(3.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX);
        match hit_result {
            HitResult::Hit { texture_coords, .. } => {
                let (u, v) = texture_coords;
                assert_approx_eq!(u, 0.3);
                assert_approx_eq!(v, 0.5);
            }
            _ => panic!(),
        }
    }
}
