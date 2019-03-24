use camera::Ray;
use data::vector::Vector;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::geometry::{Geometry, HitResult};
use world::materials::{Material, ScatterResult};

#[derive(Serialize, Deserialize)]
pub struct Entity {
    geometry: Box<dyn Geometry>,
    material: Box<dyn Material>,
}

impl Entity {
    pub fn new(geometry: Box<dyn Geometry>, material: Box<dyn Material>) -> Entity {
        Entity { geometry, material }
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> (HitResult, &Entity) {
        (self.geometry.hit(&ray, tmin, tmax), &self)
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        hit_point: &Vector,
        surface_normal: &Vector,
    ) -> Option<ScatterResult> {
        self.material.scatter(ray, hit_point, surface_normal)
    }

    pub fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        self.geometry.bounding_box(time_start, time_end)
    }
}
