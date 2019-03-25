pub mod axis_aligned_bounding_box;
pub mod bounding_volume_hierarchy;
pub mod sphere;

use camera::Ray;
use data::vector::Vector;
use std::cmp::Ordering;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::materials::Material;

#[derive(Debug, Clone)]
pub enum HitResult {
    Hit {
        distance: f64,
        ray: Ray,
        point: Vector,
        surface_normal: Vector,
        material: Material,
    },
    Miss,
}

#[typetag::serde(tag = "type")]
pub trait Geometry: Sync {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult;

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox>;
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
        };
        assert_ne!(hit_result.clone(), other_hit_result.clone());
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
        };
        assert!(other_hit_result > hit_result);
        assert!(hit_result < other_hit_result);
    }
}
