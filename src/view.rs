use config::Config;
use rand::prelude::*;
use data::vector::Vector;

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
        origin: Vector,
        lower_left_corner: Vector,
        horizontal: Vector,
        vertical: Vector
    ) -> Camera {
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
