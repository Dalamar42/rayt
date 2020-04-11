use crate::camera::Ray;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::sampling::uniform;
use crate::world::geometry::HitResult;
use crate::world::materials::ScatterResult;

const REFRACTIVE_INDEX_OF_AIR: f64 = 1.0;
const DIELECTRIC_ATTENUATION: [f64; 3] = [1.0, 1.0, 1.0];

pub fn scatter(refractive_index: f64, hit: &HitResult) -> Option<ScatterResult> {
    let unit_vector = hit.ray.direction().unit_vector();
    let reflected = reflect(&unit_vector, &hit.surface_normal);

    let uvn = Vector::dot(&unit_vector, &hit.surface_normal);

    // Determine whether we are going from air to the geometry or vv
    // This current does not support refraction from inside one geometry to another
    let (sign, n_i, n_t) = if uvn > 0.0 {
        (-1.0, refractive_index, REFRACTIVE_INDEX_OF_AIR)
    } else {
        (1.0, REFRACTIVE_INDEX_OF_AIR, refractive_index)
    };

    let cosine = -sign * uvn;
    let reflect_prob = reflectivity_schlick_approx(cosine, n_i, n_t);
    let reflect_rand: f64 = uniform();
    let should_reflect = reflect_rand < reflect_prob;

    let maybe_refracted = if should_reflect {
        None
    } else {
        refract(&unit_vector, &(sign * hit.surface_normal), n_i / n_t)
    };

    let ray = match maybe_refracted {
        Some(refracted) => Ray::new(hit.point, refracted, hit.ray.time()),
        None => Ray::new(hit.point, reflected, hit.ray.time()),
    };

    Some(ScatterResult::specular(
        Colour::new(
            DIELECTRIC_ATTENUATION[0],
            DIELECTRIC_ATTENUATION[1],
            DIELECTRIC_ATTENUATION[2],
        ),
        ray,
    ))
}

fn reflect(unit_vector: &Vector, surface_normal: &Vector) -> Vector {
    let uv = unit_vector;
    let n = surface_normal;

    let b = Vector::dot(uv, n) * n;

    uv - 2.0 * b
}

fn refract(
    unit_vector: &Vector,
    surface_normal: &Vector,
    refractive_index_ratio: f64,
) -> Option<Vector> {
    let uv = unit_vector;
    let n = surface_normal;

    let dt = Vector::dot(uv, n);

    let ni_over_nt = refractive_index_ratio;
    let discriminant = 2.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        return Some(refracted);
    }

    None
}

fn reflectivity_schlick_approx(cosine: f64, n_i: f64, n_t: f64) -> f64 {
    let r0 = (n_i - n_t) / (n_i + n_t);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}
