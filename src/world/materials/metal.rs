use crate::camera::Ray;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::pdf::random_point_in_unit_sphere;
use crate::world::geometry::HitResult;
use crate::world::materials::ScatterResult;

pub fn scatter(albedo: &Colour, fuzz: f64, hit: &HitResult) -> Option<ScatterResult> {
    let unit_vector = hit.ray.direction().unit_vector();
    let reflected = reflect(&unit_vector, &hit.face_normal());
    let ray = Ray::new(
        hit.point,
        reflected + fuzz * random_point_in_unit_sphere(),
        hit.ray.time(),
    );

    Some(ScatterResult::specular(*albedo, ray))
}

fn reflect(unit_vector: &Vector, surface_normal: &Vector) -> Vector {
    let uv = unit_vector;
    let n = surface_normal;

    let b = Vector::dot(uv, n) * n;

    uv - 2.0 * b
}
