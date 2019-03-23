pub mod sphere;
use camera::Ray;
use data::vector::Vector;

#[typetag::serde(tag = "type")]
pub trait Geometry: Sync {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64>;

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector;
}
