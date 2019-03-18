use config::Config;
use rand::prelude::*;
use data::vector::Vector;
use std::f64::consts::PI;

const ANTI_ALIASING_FACTOR: u64 = 100;

#[derive(Debug)]
pub struct Ray {
    a: Vector,
    b: Vector,
}

impl Ray {
    pub fn new(a: Vector, b: Vector) -> Ray {
        Ray {a, b}
    }

    pub fn origin(&self) -> &Vector {
        &self.a
    }

    pub fn direction(&self) -> &Vector {
        &self.b
    }

    pub fn point(&self, t: f64) -> Vector {
        &self.a + t * &self.b
    }
}

#[derive(Debug)]
pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(
        look_from: &Vector,
        look_at: &Vector,
        view_up: &Vector,
        vertical_fov: f64,
        aspect: f64,
    ) -> Camera {
        let theta = vertical_fov * PI / 180.0;
        let vector_to_plane = look_from - look_at;

        let half_height = f64::tan(theta / 2.0) * vector_to_plane.len();
        let half_width = aspect * half_height;

        let w = vector_to_plane.unit_vector();
        let u = Vector::cross(view_up, &w).unit_vector();
        let v = Vector::cross(&w, &u);

        let origin = look_from.clone();
        let lower_left_corner = &origin - half_width * &u - half_height * &v - &w;
        let horizontal = 2.0 * half_width * &u;
        let vertical = 2.0 * half_height * &v;

        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn pixels(&self, config: &Config) -> Vec<(u64, u64)> {
        let height = (&config).height;
        let width = (&config).width;

        iproduct!((0..height).rev(), 0..width).collect()
    }

    pub fn rays(&self, row: u64, col: u64, config: &Config) -> Vec<Ray> {
        let height = (&config).height;
        let width = (&config).width;

        (0..ANTI_ALIASING_FACTOR)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let row_fuzz: f64 = rng.gen();
                let col_fuzz: f64 = rng.gen();

                let v = (row as f64) + row_fuzz;
                let h = (col as f64) + col_fuzz;

                self.ray(h / (width as f64), v / (height as f64))
            })
            .collect()
    }

    fn ray(&self, h: f64, v: f64) -> Ray {
        Ray {
            a: (&self).origin.clone(),
            b: &self.lower_left_corner +
                h * &self.horizontal +
                v * &self.vertical -
                &self.origin,
        }
    }
}
