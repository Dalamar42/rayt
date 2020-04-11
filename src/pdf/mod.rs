use crate::data::vector::Vector;
use crate::onb::Onb;
use crate::world::geometry::Geometry;

mod cosine;
mod geometry;
mod mixture;

pub enum Pdf {
    Cosine(Onb),
    Geometry {
        geometries: Vec<Geometry>,
        origin: Vector,
    },
    Mixture(Box<Pdf>, Box<Pdf>),
}

impl Pdf {
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
