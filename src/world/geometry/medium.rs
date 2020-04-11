use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::float;
use crate::pdf::uniform;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::{Geometry, HitResult, Hittable};
use crate::world::materials::Material;
use crate::world::texture::Texture;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstantMedium {
    boundary: Box<Geometry>,
    density: f64,
    material: Material,
}

impl ConstantMedium {
    pub fn build(boundary: Geometry, density: f64, albedo: Texture) -> Geometry {
        Geometry::ConstantMedium(Box::from(ConstantMedium {
            boundary: Box::from(boundary),
            density,
            material: Material::Isotropic { albedo },
        }))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        self.boundary
            .hit(ray, std::f64::MIN, std::f64::MAX)
            .and_then(|first_hit| {
                self.boundary
                    .hit(ray, &first_hit.distance + 0.0001, std::f64::MAX)
                    .map(|second_hit| (first_hit, second_hit))
            })
            .and_then(|(first_hit, second_hit)| {
                let mut d1 = first_hit.distance;
                let mut d2 = second_hit.distance;

                d1 = float::max(d1, tmin);
                d2 = float::min(d2, tmax);

                if d1 >= d2 {
                    return None;
                }
                d1 = float::max(d1, 0.0);

                let distance_inside_boundary = (d2 - d1) * ray.direction().len();
                let hit_distance = -(1.0 / self.density) * uniform::<f64>().ln();

                if hit_distance >= distance_inside_boundary {
                    return None;
                }

                let distance = d1 + hit_distance / ray.direction().len();

                Some(HitResult {
                    distance,
                    ray: *ray,
                    point: ray.point(distance),
                    surface_normal: Vector::new(1.0, 0.0, 0.0), // Arbitrary,
                    material: self.material.clone(),
                    ..first_hit
                })
            })
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        self.boundary.bounding_box(time_start, time_end)
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        self.material.validate(assets)?;
        self.boundary.validate(assets)?;
        Ok(())
    }

    fn is_attractor(&self) -> bool {
        false
    }
}
