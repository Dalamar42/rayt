use view::Ray;
use data::vector::Vector;
use rand::prelude::*;
use data::colour::Colour;
use volumes::geometry::Geometry;

pub trait Material {
    fn scatter(
        &self, geometry: &Box<Geometry>, ray: &Ray, distance: f64,
    ) -> Option<ScatterResult>;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(
        &self, geometry: &Box<Geometry>, ray: &Ray, distance: f64,
    ) -> Option<ScatterResult> {
        let hit_point = &ray.point(distance);
        let surface_normal = &geometry.surface_normal(&ray, distance);

        let diffuse = random_point_in_unit_sphere();
        let target = hit_point + surface_normal + diffuse;

        let ray = Ray::new(hit_point.clone(), target);

        Some(ScatterResult { ray, attenuation: self.albedo.clone() })
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self, geometry: &Box<Geometry>, ray: &Ray, distance: f64,
    ) -> Option<ScatterResult> {
        let hit_point = &ray.point(distance);
        let reflected = &geometry.reflect(&ray, distance);

        let ray = Ray::new(
            hit_point.clone(),
            reflected + self.fuzz * random_point_in_unit_sphere(),
        );

        Some(ScatterResult { ray, attenuation: self.albedo.clone() })
    }
}

#[derive(Debug)]
pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Colour,
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
