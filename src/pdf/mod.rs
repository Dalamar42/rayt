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
use crate::world::geometry::Geometry;

mod cosine;
mod geometry;
mod mixture;

#[derive(Debug)]
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
