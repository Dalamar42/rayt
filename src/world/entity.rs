use world::geometry::Geometry;
use world::materials::{Material, ScatterResult};
use view::Ray;

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub geometry: Box<dyn Geometry>,
    pub material: Box<dyn Material>,
}

impl Entity {

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64> {
        self.geometry.hit(&ray, tmin, tmax)
    }

    pub fn scatter(&self, ray: &Ray, distance: f64) -> Option<ScatterResult> {
        self.material.scatter(&self.geometry, &ray, distance)
    }
}