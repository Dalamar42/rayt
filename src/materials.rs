use view::Ray;
use data::vector::Vector;
use rand::prelude::*;
use data::colour::Colour;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian {albedo: Colour},
    Metal {albedo: Colour},
}

#[derive(Debug)]
pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Colour,
}

pub fn scatter_lambertian(
    albedo: &Colour, hit_point: &Vector, surface_normal: &Vector,
) -> Option<ScatterResult> {
    let diffuse = random_point_in_unit_sphere();
    let target = hit_point + surface_normal + diffuse;

    let ray = Ray::new(hit_point.clone(), target);

    Some(ScatterResult { ray, attenuation: albedo.clone() })
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

pub fn scatter_metal(
    albedo: &Colour, hit_point: &Vector, reflected: &Vector,
) -> Option<ScatterResult> {
    let ray = Ray::new(hit_point.clone(), reflected.clone());

    Some(ScatterResult { ray, attenuation: albedo.clone() })
}
