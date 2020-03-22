pub mod axis_aligned_bounding_box;
pub mod bounding_volume_hierarchy;
pub mod cube;
pub mod flip_normals;
pub mod rectangle;
pub mod rotate;
pub mod sphere;
pub mod translate;

use camera::Ray;
use data::assets::Assets;
use data::vector::Vector;
use failure::Error;
use std::cmp::Ordering;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::geometry::flip_normals::FlipNormals;
use world::geometry::rotate::RotateY;
use world::geometry::translate::Translate;
use world::materials::Material;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum HitResult {
    Hit {
        distance: f64,
        ray: Ray,
        point: Vector,
        surface_normal: Vector,
        material: Material,
        texture_coords: (f64, f64),
    },
    Miss,
}

#[typetag::serde(tag = "type")]
pub trait Geometry: Sync {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult;

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox>;

    fn validate(&self, assets: &Assets) -> Result<(), Error>;

    fn flip(self) -> FlipNormals
    where
        Self: 'static + Sized,
    {
        FlipNormals::new(Box::new(self))
    }

    fn translate(self, offset: Vector) -> Translate
    where
        Self: 'static + Sized,
    {
        Translate::new(Box::new(self), offset)
    }

    fn rotate_y(self, angle: f64) -> Result<RotateY, Error>
    where
        Self: 'static + Sized,
    {
        let rotate = RotateY::new(Box::new(self), angle)?;
        Ok(rotate)
    }
}

impl Ord for HitResult {
    fn cmp(&self, other: &HitResult) -> Ordering {
        match (self, other) {
            (HitResult::Miss, HitResult::Miss) => Ordering::Equal,
            (HitResult::Hit { .. }, HitResult::Miss) => Ordering::Less,
            (HitResult::Miss, HitResult::Hit { .. }) => Ordering::Greater,
            (
                HitResult::Hit { distance, .. },
                HitResult::Hit {
                    distance: other_distance,
                    ..
                },
            ) => {
                // We should never get a NaN here. Panic if we do
                distance.partial_cmp(other_distance).unwrap()
            }
        }
    }
}

impl PartialOrd for HitResult {
    fn partial_cmp(&self, other: &HitResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HitResult {
    fn eq(&self, other: &HitResult) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HitResult {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_result_eq() {
        let hit_result = HitResult::Hit {
            distance: 0.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };

        assert_eq!(HitResult::Miss, HitResult::Miss);
        assert_ne!(hit_result.clone(), HitResult::Miss);
        assert_eq!(hit_result.clone(), hit_result.clone());

        let other_hit_result = HitResult::Hit {
            distance: 1.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        assert_ne!(hit_result, other_hit_result);
    }

    #[test]
    fn test_hit_result_ord() {
        let hit_result = HitResult::Hit {
            distance: 0.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };

        assert!(hit_result < HitResult::Miss);
        assert!(HitResult::Miss > hit_result);

        let other_hit_result = HitResult::Hit {
            distance: 1.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        assert!(other_hit_result > hit_result);
        assert!(hit_result < other_hit_result);
    }
}
