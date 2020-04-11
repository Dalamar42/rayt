//! A module containing helpers for working with ortho-normal basis

use crate::data::vector::Vector;

#[derive(Debug)]
pub struct Onb {
    u: Vector,
    v: Vector,
    w: Vector,
}

impl Onb {
    pub fn build_from_w(vector_n: &Vector) -> Onb {
        let w = vector_n.unit_vector();

        // Check if it might be the x axis and if not pick it, otherwise pick the y axis
        let a = if w.x().abs() > 0.9 {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };

        let v = Vector::cross(&w, &a).unit_vector();
        let u = Vector::cross(&w, &v);

        Onb { u, v, w }
    }

    pub fn w(&self) -> &Vector {
        &self.w
    }

    pub fn local_from_vec(&self, a: &Vector) -> Vector {
        a.x() * self.u + a.y() * self.v + a.z() * self.w
    }
}
