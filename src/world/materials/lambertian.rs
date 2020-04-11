use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::onb::Onb;
use crate::pdf::Pdf;
use crate::world::geometry::HitResult;
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

pub fn scatter(albedo: &Texture, hit: &HitResult, assets: &Assets) -> Option<ScatterResult> {
    let albedo = albedo.value(hit.texture_coords, &hit.point, &assets);
    let pdf = Pdf::Cosine(Onb::build_from_w(&hit.face_normal()));

    Some(ScatterResult::diffuse(albedo, pdf))
}
