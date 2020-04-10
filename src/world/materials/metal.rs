use crate::camera::Ray;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::sampling::random_point_in_unit_sphere;
use crate::world::materials::ScatterResult;

pub fn scatter(
    albedo: &Colour,
    fuzz: f64,
    ray: &Ray,
    hit_point: &Vector,
    surface_normal: &Vector,
) -> Option<ScatterResult> {
    let unit_vector = ray.direction().unit_vector();
    let reflected = reflect(&unit_vector, &surface_normal);

    let ray = Ray::new(
        *hit_point,
        reflected + fuzz * random_point_in_unit_sphere(),
        ray.time(),
    );

    if Vector::dot(&ray.direction(), &surface_normal) <= 0.0 {
        return None;
    }

    Some(ScatterResult::new(ray, *albedo, 1.0))
}

fn reflect(unit_vector: &Vector, surface_normal: &Vector) -> Vector {
    let uv = unit_vector;
    let n = surface_normal;

    let b = Vector::dot(uv, n) * n;

    uv - 2.0 * b
}
