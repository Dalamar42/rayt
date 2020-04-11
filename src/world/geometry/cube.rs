use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::rectangle::{XyRect, XzRect, YzRect};
use crate::world::geometry::{Geometry, HitResult, Hittable};
use crate::world::materials::Material;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cube {
    rectangles: Vec<Geometry>,
    pmin: Vector,
    pmax: Vector,
}

impl Cube {
    pub fn build(pmin: Vector, pmax: Vector, material: Material) -> Geometry {
        let mut rectangles: Vec<Geometry> = Vec::with_capacity(6);

        rectangles.push(XyRect::build(
            (pmin.x(), pmax.x()),
            (pmin.y(), pmax.y()),
            pmax.z(),
            material.clone(),
        ));
        rectangles.push(
            XyRect::build(
                (pmin.x(), pmax.x()),
                (pmin.y(), pmax.y()),
                pmin.z(),
                material.clone(),
            )
            .flip(),
        );
        rectangles.push(XzRect::build(
            (pmin.x(), pmax.x()),
            (pmin.z(), pmax.z()),
            pmax.y(),
            material.clone(),
        ));
        rectangles.push(
            XzRect::build(
                (pmin.x(), pmax.x()),
                (pmin.z(), pmax.z()),
                pmin.y(),
                material.clone(),
            )
            .flip(),
        );
        rectangles.push(YzRect::build(
            (pmin.y(), pmax.y()),
            (pmin.z(), pmax.z()),
            pmax.x(),
            material.clone(),
        ));
        rectangles.push(
            YzRect::build(
                (pmin.y(), pmax.y()),
                (pmin.z(), pmax.z()),
                pmin.x(),
                material,
            )
            .flip(),
        );

        Geometry::Cube(Box::from(Cube {
            rectangles,
            pmin,
            pmax,
        }))
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        self.rectangles
            .iter()
            .flat_map(|rect| rect.hit(ray, tmin, tmax))
            .min()
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(self.pmin, self.pmax))
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        for rectangle in &self.rectangles {
            rectangle.validate(assets)?
        }
        Ok(())
    }

    fn is_attractor(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_cube_hit() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let ray = Ray::new(Vector::new(2.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);

        let ray = Ray::new(Vector::new(-1.0, 0.5, 0.5), Vector::new(1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);

        let ray = Ray::new(Vector::new(0.5, 2.0, 0.5), Vector::new(0.0, -1.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);
    }

    #[test]
    fn test_cube_surface_normal() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let ray = Ray::new(Vector::new(2.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_eq!(hit_result.surface_normal, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_cube_bounding_box() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0));

        assert_eq!(cube.bounding_box(0.0, 0.0), Some(expected_box));
    }

    #[test]
    fn test_cube_texture_coords() {
        let cube = Cube::build(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0),
            Material::Dielectric {
                refractive_index: 1.5,
            },
        );

        let ray = Ray::new(Vector::new(2.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);
        let hit_result = cube.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let (u, v) = hit_result.texture_coords;
        assert_approx_eq!(u, 0.5);
        assert_approx_eq!(v, 0.5);
    }
}
