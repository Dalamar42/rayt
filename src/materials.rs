use view::Ray;
use data::vector::Vector;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian {albedo: f64},
}

#[derive(Debug)]
pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: f64,
}

pub fn scatter_lambertian(
    albedo: f64, hit_point: &Vector, surface_normal: &Vector,
) -> Option<ScatterResult> {
    let diffuse = random_point_in_unit_sphere();
    let target = hit_point + surface_normal + diffuse;

    let ray = Ray::new(hit_point.clone(), target);

    Some(ScatterResult { ray, attenuation: albedo })
}

fn random_point_in_unit_sphere() -> Vector {
    let mut rng = rand::thread_rng();
    let unit_vector = Vector {x: 1.0, y: 1.0, z: 1.0};

    loop {
        let point = 2.0 * Vector {x: rng.gen(), y: rng.gen(), z: rng.gen()} - &unit_vector;
        if point.len_squared() < 1.0 {
            return point
        }
    }
}
