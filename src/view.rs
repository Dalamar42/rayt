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
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f64,
    save: CameraSave,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraSave {
    look_from: Vector,
    look_at: Vector,
    view_up: Vector,
    vertical_fov: f64,
    aspect: f64,
    aperture: f64,
    focus_distance: f64,
}

impl Camera {
    pub fn new(
        look_from: &Vector,
        look_at: &Vector,
        view_up: &Vector,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        let theta = vertical_fov * PI / 180.0;
        let half_height = f64::tan(theta / 2.0) * focus_distance;
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vector::cross(view_up, &w).unit_vector();
        let v = Vector::cross(&w, &u);

        let origin = look_from.clone();
        let lower_left_corner = &origin - half_width * &u - half_height * &v - focus_distance * &w;
        let horizontal = 2.0 * half_width * &u;
        let vertical = 2.0 * half_height * &v;

        let save = CameraSave {
            look_from: look_from.clone(),
            look_at: look_at.clone(),
            view_up: view_up.clone(),
            vertical_fov,
            aspect,
            aperture,
            focus_distance,
        };

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            save,
        }
    }

    pub fn into_save(self) -> CameraSave {
        self.save
    }

    pub fn from_save(save: CameraSave) -> Camera {
        Camera::new(
            &save.look_from,
            &save.look_at,
            &save.view_up,
            save.vertical_fov,
            save.aspect,
            save.aperture,
            save.focus_distance,
        )
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
        let rd = self.lens_radius * random_point_in_unit_disk();
        let lens_offset = &self.u * rd.x + &self.v * rd.y;
        Ray {
            a: &self.origin + &lens_offset,
            b: &self.lower_left_corner +
                h * &self.horizontal +
                v * &self.vertical -
                &self.origin -
                &lens_offset,
        }
    }
}

fn random_point_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    let centre = Vector {x: 1.0, y: 1.0, z: 0.0};

    loop {
        let point = 2.0 * Vector {x: rng.gen(), y: rng.gen(), z: 0.0} - &centre;
        if Vector::dot(&point, &point) < 1.0 {
            return point
        }
    }
}
