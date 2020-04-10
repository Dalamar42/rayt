//! A module containing functions for generating random vectors using various distributions
//!
//! All distributions here are rotationally symmetric about z.
//!
//! Given PDF p(direction) = f(Θ) the 1-dimensional PDFs for θ and φ are:
//! - a(φ) = 1/(2π)
//! - b(θ) = 2π * f(θ) * sin(θ)
//!
//! Given r1, r1 uniform random numbers we have:
//! - r1 = integral_0_φ ( (1/2π) * dt )
//! - r1 = integral_0_θ ( 2π * f(t) * sin(t) * dt )

use crate::data::vector::Vector;
use rand::distributions::Standard;
use rand::prelude::*;
use std::f64::consts::PI;

pub fn uniform<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen::<T>()
}

pub fn random_point_in_unit_sphere() -> Vector {
    let centre = Vector::new(1.0, 1.0, 1.0);

    loop {
        let point = 2.0 * Vector::new(uniform(), uniform(), uniform()) - centre;
        if point.len_squared() < 1.0 {
            return point;
        }
    }
}

#[allow(dead_code)]
pub fn random_point_in_unit_hemisphere(surface_normal: &Vector) -> Vector {
    let point = random_point_in_unit_sphere();

    if Vector::dot(&point, surface_normal) > 0.0 {
        point
    } else {
        -point
    }
}

#[allow(dead_code)]
pub fn random_point_on_unit_sphere() -> Vector {
    let centre = Vector::new(1.0, 1.0, 1.0);

    loop {
        let point = 2.0 * Vector::new(uniform(), uniform(), uniform()) - centre;
        if point.len_squared() < 1.0 {
            return point.unit_vector();
        }
    }
}

pub fn random_cosine_direction() -> Vector {
    // PDF p(direction = cosθ / π
    let r1 = uniform::<f64>();
    let r2 = uniform::<f64>();

    let z = f64::sqrt(1.0 - r2);

    let phi = 2.0 * PI * r1;

    let x = f64::cos(phi) * f64::sqrt(r2);
    let y = f64::sin(phi) * f64::sqrt(r2);

    Vector::new(x, y, z)
}

#[allow(dead_code)]
pub fn random_uniform_in_sphere_direction() -> Vector {
    // PDF p(direction = 1 / (4π)
    let r1 = uniform::<f64>();
    let r2 = uniform::<f64>();

    let z = 1.0 - 2.0 * r2;

    let phi = 2.0 * PI * r1;

    let x = f64::cos(phi) * 2.0 * f64::sqrt(r2 * (1.0 - r2));
    let y = f64::sin(phi) * 2.0 * f64::sqrt(r2 * (1.0 - r2));

    Vector::new(x, y, z)
}

#[allow(dead_code)]
pub fn random_uniform_in_hemisphere_direction() -> Vector {
    // PDF p(direction = 1 / (2π)
    let r1 = uniform::<f64>();
    let r2 = uniform::<f64>();

    let z = 1.0 - r2;

    let phi = 2.0 * PI * r1;

    let x = f64::cos(phi) * 2.0 * f64::sqrt(r2 * (1.0 - r2));
    let y = f64::sin(phi) * 2.0 * f64::sqrt(r2 * (1.0 - r2));

    Vector::new(x, y, z)
}
