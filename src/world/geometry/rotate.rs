use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::{Geometry, HitResult, Hittable};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeometryError {
    #[error("rotation is only supported for geometries have have bounding boxes")]
    RotationUnsupported(),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RotateY {
    geometry: Box<Geometry>,
    angle: f64,
}

impl RotateY {
    pub fn build(geometry: Geometry, angle: f64) -> Result<Geometry, GeometryError> {
        if geometry.bounding_box(0.0, 0.0).is_none() {
            return Err(GeometryError::RotationUnsupported());
        }
        Ok(Geometry::RotateY(Box::from(RotateY {
            geometry: Box::from(geometry),
            angle,
        })))
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let origin = ray.origin().rotate_y(-self.angle);
        let direction = ray.direction().rotate_y(-self.angle);
        let rotated_ray = Ray::new(origin, direction, ray.time());

        self.geometry.hit(&rotated_ray, tmin, tmax).map(|hit| {
            let point = hit.point.rotate_y(self.angle);
            let surface_normal = hit.surface_normal.rotate_y(self.angle);
            HitResult {
                point,
                surface_normal,
                ..hit
            }
        })
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        match self.geometry.bounding_box(time_start, time_end) {
            None => None,
            Some(bbox) => {
                let mut min = Vector::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
                let mut max = Vector::new(std::f64::MIN, std::f64::MIN, std::f64::MIN);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let tester = Vector::new(
                                (i as f64) * bbox.max().x() + ((1 - i) as f64) * bbox.min().x(),
                                (j as f64) * bbox.max().y() + ((1 - j) as f64) * bbox.min().y(),
                                (k as f64) * bbox.max().z() + ((1 - k) as f64) * bbox.min().z(),
                            )
                            .rotate_y(self.angle);

                            min = min.min(&tester);
                            max = max.max(&tester);
                        }
                    }
                }
                Some(AxisAlignedBoundingBox::new(min, max))
            }
        }
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        self.geometry.validate(assets)
    }

    fn is_attractor(&self) -> bool {
        self.geometry.is_attractor()
    }

    fn pdf_value(&self, origin: &Vector, direction: &Vector) -> f64 {
        self.geometry.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Vector) -> Vector {
        self.geometry.random(origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::geometry::cube::Cube;
    use crate::world::materials::Material;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_rotate_hit() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(2.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let ray = Ray::new(Vector::new(3.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);

        let rotated_cube = cube.rotate_y(-90.0).unwrap();

        let hit_result = rotated_cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 3.0);
    }

    #[test]
    fn test_rotate_surface_normal() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(2.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let ray = Ray::new(Vector::new(3.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.surface_normal.x(), 1.0);
        assert_approx_eq!(hit_result.surface_normal.y(), 0.0);
        assert_approx_eq!(hit_result.surface_normal.z(), 0.0);

        let rotated_cube = cube.rotate_y(-90.0).unwrap();

        let hit_result = rotated_cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.surface_normal.x(), 1.0);
        assert_approx_eq!(hit_result.surface_normal.y(), 0.0);
        assert_approx_eq!(hit_result.surface_normal.z(), 0.0);
    }

    #[test]
    fn test_rotate_bounding_box() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(2.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(0.0, 0.0, 0.0), Vector::new(2.0, 1.0, 1.0));

        assert_eq!(cube.bounding_box(0.0, 0.0), Some(expected_box));

        let rotated_cube = cube.rotate_y(-90.0).unwrap();

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(-1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 2.0));
        let bbox = rotated_cube.bounding_box(0.0, 0.0).unwrap();

        assert_approx_eq!(bbox.min().x(), expected_box.min().x());
        assert_approx_eq!(bbox.min().y(), expected_box.min().y());
        assert_approx_eq!(bbox.min().z(), expected_box.min().z());

        assert_approx_eq!(bbox.max().x(), expected_box.max().x());
        assert_approx_eq!(bbox.max().y(), expected_box.max().y());
        assert_approx_eq!(bbox.max().z(), expected_box.max().z());
    }

    #[test]
    fn test_rotate_texture_coords() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(2.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let ray = Ray::new(Vector::new(3.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let (u, v) = hit_result.texture_coords;
        assert_approx_eq!(u, 0.5);
        assert_approx_eq!(v, 0.5);

        let rotated_cube = cube.rotate_y(-90.0).unwrap();

        let hit_result = rotated_cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let (u, v) = hit_result.texture_coords;
        assert_approx_eq!(u, 0.25);
        assert_approx_eq!(v, 0.5);
    }
}
