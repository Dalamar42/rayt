//! PDF p(direction = cosθ / π

use crate::data::vector::Vector;
use crate::onb::Onb;
use crate::pdf::uniform;
use std::f64::consts::PI;

pub fn value(onb: &Onb, direction: &Vector) -> f64 {
    let cosine = Vector::dot(&direction.unit_vector(), onb.w());
    if cosine <= 0.0 {
        0.0
    } else {
        cosine / PI
    }
}

pub fn generate(onb: &Onb) -> Vector {
    onb.local_from_vec(&random_cosine_direction())
}

fn random_cosine_direction() -> Vector {
    let r1 = uniform::<f64>();
    let r2 = uniform::<f64>();

    let z = f64::sqrt(1.0 - r2);

    let phi = 2.0 * PI * r1;

    let x = f64::cos(phi) * f64::sqrt(r2);
    let y = f64::sin(phi) * f64::sqrt(r2);

    Vector::new(x, y, z)
}
