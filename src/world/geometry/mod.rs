pub mod axis_aligned_bounding_box;
pub mod bounding_volume_hierarchy;
pub mod cube;
pub mod flip_normals;
pub mod medium;
pub mod rectangle;
pub mod rotate;
pub mod sphere;
pub mod translate;

use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::flip_normals::FlipNormals;
use crate::world::geometry::rotate::RotateY;
use crate::world::geometry::translate::Translate;
use crate::world::materials::Material;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct HitResult {
    pub distance: f64,
    pub ray: Ray,
    pub point: Vector,
    pub surface_normal: Vector,
    pub material: Material,
    pub texture_coords: (f64, f64),
}

impl HitResult {
    pub fn front_face(&self) -> bool {
        Vector::dot(self.ray.direction(), &self.surface_normal) < 0.0
    }
}

#[typetag::serde(tag = "type")]
pub trait Geometry: Sync {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult>;

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox>;

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error>;

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

    fn rotate_y(self, angle: f64) -> Result<RotateY, anyhow::Error>
    where
        Self: 'static + Sized,
    {
        let rotate = RotateY::new(Box::new(self), angle)?;
        Ok(rotate)
    }
}

impl Ord for HitResult {
    fn cmp(&self, other: &HitResult) -> Ordering {
        // We should never get a NaN here. Panic if we do
        self.distance.partial_cmp(&other.distance).unwrap()
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
        let hit_result = HitResult {
            distance: 0.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        assert_eq!(hit_result.clone(), hit_result.clone());

        let other_hit_result = HitResult {
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
        let hit_result = HitResult {
            distance: 0.0,
            ray: Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0),
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
            texture_coords: (1.0, 0.5),
        };
        let other_hit_result = HitResult {
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
