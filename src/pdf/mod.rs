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
use crate::onb::Onb;
use crate::sampling::uniform;
use crate::world::geometry::Geometry;
use std::f64::consts::PI;

mod cosine;
mod geometry;
mod mixture;

pub enum Pdf<'a> {
    Cosine(Onb),
    Geometry {
        geometries: &'a Vec<Geometry>,
        origin: Vector,
    },
    Mixture(Box<Pdf<'a>>, Box<Pdf<'a>>),
}

impl Pdf<'_> {
    pub fn boxed(self) -> Box<Self> {
        Box::from(self)
    }

    pub fn value(&self, direction: &Vector) -> f64 {
        match self {
            Pdf::Cosine(onb) => cosine::value(&onb, direction),
            Pdf::Geometry { geometries, origin } => {
                geometry::value(&geometries, &origin, direction)
            }
            Pdf::Mixture(pdf_a, pdf_b) => mixture::value(&pdf_a, &pdf_b, direction),
        }
    }

    pub fn generate(&self) -> Vector {
        match self {
            Pdf::Cosine(onb) => cosine::generate(&onb),
            Pdf::Geometry { geometries, origin } => geometry::generate(&geometries, &origin),
            Pdf::Mixture(pdf_a, pdf_b) => mixture::generate(&pdf_a, &pdf_b),
        }
    }
}

pub fn random_to_sphere(radius: f64, cp: &Vector) -> Vector {
    let r1 = uniform::<f64>();
    let r2 = uniform::<f64>();

    let cos_theta_max = f64::sqrt(1.0 - radius.powi(2) / cp.len_squared());
    let z = 1.0 + r2 * (cos_theta_max - 1.0);

    let phi = 2.0 * PI * r1;
    let x = f64::cos(phi) * f64::sqrt(1.0 - z.powi(2));
    let y = f64::sin(phi) * f64::sqrt(1.0 - z.powi(2));

    Vector::new(x, y, z)
}
