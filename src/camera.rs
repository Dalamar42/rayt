use config::Config;
use data::vector::Vector;
use rand::prelude::*;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Ray {
    a: Vector,
    b: Vector,
    time: f64,
}

impl Ray {
    pub fn new(a: Vector, b: Vector, time: f64) -> Ray {
        Ray { a, b, time }
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

    pub fn time(&self) -> f64 {
        self.time
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
    time_start: f64,
    time_end: f64,
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
    time_start: f64,
    time_end: f64,
}

impl CameraSave {
    pub fn new(
        look_from: &Vector,
        look_at: &Vector,
        view_up: &Vector,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
        time_start: f64,
        time_end: f64,
    ) -> CameraSave {
        CameraSave {
            look_from: look_from.clone(),
            look_at: look_at.clone(),
            view_up: view_up.clone(),
            vertical_fov,
            aspect,
            aperture,
            focus_distance,
            time_start,
            time_end,
        }
    }

    pub fn into_camera(self) -> Camera {
        let lens_radius = self.aperture / 2.0;

        let theta = self.vertical_fov * PI / 180.0;
        let half_height = f64::tan(theta / 2.0) * self.focus_distance;
        let half_width = self.aspect * half_height;

        let w = (&self.look_from - &self.look_at).unit_vector();
        let u = Vector::cross(&self.view_up, &w).unit_vector();
        let v = Vector::cross(&w, &u);

        let origin = self.look_from.clone();
        let lower_left_corner =
            &origin - half_width * &u - half_height * &v - self.focus_distance * &w;
        let horizontal = 2.0 * half_width * &u;
        let vertical = 2.0 * half_height * &v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time_start: self.time_start,
            time_end: self.time_end,
            save: self,
        }
    }
}

impl Camera {
    pub fn pixels(&self, config: &Config) -> Vec<(u32, u32)> {
        let height = config.height;
        let width = config.width;

        iproduct!(0..height, 0..width).collect()
    }

    pub fn rays(&self, row: u32, col: u32, config: &Config) -> Vec<Ray> {
        let height = config.height;
        let width = config.width;

        (0..config.num_of_rays)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let row_fuzz: f64 = rng.gen();
                let col_fuzz: f64 = rng.gen();

                let v = f64::from(row) + row_fuzz;
                let h = f64::from(col) + col_fuzz;

                self.ray(h / f64::from(width), v / f64::from(height))
            })
            .collect()
    }

    fn ray(&self, h: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_point_in_unit_disk();
        let lens_offset = &self.u * rd.x + &self.v * rd.y;
        let mut rng = rand::thread_rng();
        let time = self.time_start + rng.gen::<f64>() * (self.time_end - self.time_start);
        Ray {
            a: &self.origin + &lens_offset,
            b: &self.lower_left_corner + h * &self.horizontal + v * &self.vertical
                - &self.origin
                - &lens_offset,
            time,
        }
    }
}

fn random_point_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    let centre = Vector {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    };

    loop {
        let point =
            2.0 * Vector {
                x: rng.gen(),
                y: rng.gen(),
                z: 0.0,
            } - &centre;
        if Vector::dot(&point, &point) < 1.0 {
            return point;
        }
    }
}
