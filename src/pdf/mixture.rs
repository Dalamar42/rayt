//! Mixture of PDFs

use crate::data::vector::Vector;
use crate::pdf::{uniform, Pdf};

const MIXTURE_RATIO: f64 = 0.5;

pub fn value(pdf_a: &Pdf, pdf_b: &Pdf, direction: &Vector) -> f64 {
    MIXTURE_RATIO * pdf_a.value(direction) + (1.0 - MIXTURE_RATIO) * pdf_b.value(direction)
}

pub fn generate(pdf_a: &Pdf, pdf_b: &Pdf) -> Vector {
    if uniform::<f64>() < MIXTURE_RATIO {
        pdf_a.generate()
    } else {
        pdf_b.generate()
    }
}
