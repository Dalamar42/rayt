use camera::Ray;
use data::assets::Assets;
use data::vector::Vector;
use sampling::random_point_in_unit_sphere;
use std::f64::consts::PI;
use world::materials::ScatterResult;
use world::texture::Texture;

pub fn scattering_pdf(surface_normal: &Vector, scattered: &Ray) -> f64 {
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
    let diffuse = random_point_in_unit_sphere();
    let target = hit_point + surface_normal + diffuse;

    let ray = Ray::new(*hit_point, target - hit_point, ray.time());
    let pdf = Vector::dot(surface_normal, &ray.direction().unit_vector()) / PI;

    Some(ScatterResult::new(
        ray,
        albedo.value(texture_coords, &hit_point, &assets),
        pdf,
    ))
}
