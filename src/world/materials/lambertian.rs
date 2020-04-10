use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::onb::Onb;
use crate::sampling::random_cosine_direction;
use crate::world::materials::ScatterResult;
use crate::world::texture::Texture;
use std::f64::consts::PI;

pub fn scattering_pdf(surface_normal: &Vector, scattered: &Ray) -> f64 {
    // Using s(direction) = cos(θ) / π, where θ is the angle relative to the surface normal
    let mut cosine = Vector::dot(surface_normal, &scattered.direction().unit_vector());
    if cosine < 0.0 {
        cosine = 0.0;
    }
    cosine / PI
}

pub fn scatter(
    albedo: &Texture,
    ray: &Ray,
    hit_point: &Vector,
    surface_normal: &Vector,
    texture_coords: (f64, f64),
    assets: &Assets,
) -> Option<ScatterResult> {
    // Sample using PDF p(direction = cosθ / π
    let onb = Onb::build_from_w(surface_normal);
    let direction = onb.local_from_vec(&random_cosine_direction());
    let ray = Ray::new(*hit_point, direction.unit_vector(), ray.time());
    let pdf = Vector::dot(onb.w(), &direction) / PI;

    Some(ScatterResult::new(
        ray,
        albedo.value(texture_coords, &hit_point, &assets),
        pdf,
    ))
}
