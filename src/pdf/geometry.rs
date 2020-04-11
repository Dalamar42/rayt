//! PDF that samples towards a list of geometries

use crate::data::vector::Vector;
use crate::pdf::uniform_between;
use crate::world::geometry::{Geometry, Hittable};

pub fn value(geometries: &[Geometry], origin: &Vector, direction: &Vector) -> f64 {
    let weight = 1.0 / geometries.len() as f64;
    geometries
        .iter()
        .map(|geo| weight * geo.pdf_value(origin, direction))
        .sum()
}

pub fn generate(geometries: &[Geometry], origin: &Vector) -> Vector {
    let choice = uniform_between::<usize>(0, geometries.len());
    geometries.get(choice).unwrap().random(origin)
}
