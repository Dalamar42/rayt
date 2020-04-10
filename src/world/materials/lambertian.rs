use camera::Ray;
use data::assets::Assets;
use data::vector::Vector;
use sampling::random_point_in_unit_hemisphere;
use std::f64::consts::PI;
use world::materials::ScatterResult;
use world::texture::Texture;

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
    // Sample by choosing randomly from the hemisphere above the surface
    let direction = random_point_in_unit_hemisphere(surface_normal);
    let ray = Ray::new(*hit_point, direction.unit_vector(), ray.time());
    let pdf = 0.5 / PI;

    Some(ScatterResult::new(
        ray,
        albedo.value(texture_coords, &hit_point, &assets),
        pdf,
    ))
}
