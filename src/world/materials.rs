use view::Ray;
use data::vector::Vector;
use rand::prelude::*;
use data::colour::Colour;
use world::geometry::Geometry;

#[derive(Debug)]
pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Colour,
}

pub trait Material: Sync {
    fn scatter(
        &self, geometry: &Box<Geometry>, ray: &Ray, distance: f64,
    ) -> Option<ScatterResult>;
}

fn random_point_in_unit_sphere() -> Vector {
    let mut rng = rand::thread_rng();
    let centre = Vector {x: 1.0, y: 1.0, z: 1.0};

    loop {
        let point = 2.0 * Vector {x: rng.gen(), y: rng.gen(), z: rng.gen()} - &centre;
        if point.len_squared() < 1.0 {
            return point
        }
    }
}

fn reflect(unit_vector: &Vector, surface_normal: &Vector) -> Vector {
    let uv = unit_vector;
    let n = surface_normal;

    let b = Vector::dot(uv, n) * n;

    uv - 2.0 * b
}

fn refract(
    unit_vector: &Vector,
    surface_normal: &Vector,
    refractive_index_ratio: f64,
) -> Option<Vector> {
    let uv = unit_vector;
    let n = surface_normal;

    let dt = Vector::dot(uv, n);

    let ni_over_nt = refractive_index_ratio;
    let discriminant = 2.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        return Some(refracted)
    }

    None
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

        let ray = Ray::new(hit_point.clone(), target - hit_point);

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
        let unit_vector = ray.direction().unit_vector();
        let surface_normal = geometry.surface_normal(&ray, distance);
        let reflected = reflect(&unit_vector, &surface_normal);

        let hit_point = &ray.point(distance);
        let ray = Ray::new(
            hit_point.clone(),
            reflected + self.fuzz * random_point_in_unit_sphere(),
        );

        Some(ScatterResult { ray, attenuation: self.albedo.clone() })
    }
}

const REFRACTIVE_INDEX_OF_AIR: f64 = 1.0;
const DIELECTRIC_ATTENUATION: Colour = Colour {r: 1.0, g: 1.0, b: 1.0};

#[derive(Debug)]
pub struct Dielectric {
    // Air: 1.0, Glass: 1.3-1.7, Diamond: 2.4
    pub refractive_index: f64,
}

fn reflectivity_schlick_approx(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(
        &self, geometry: &Box<Geometry>, ray: &Ray, distance: f64,
    ) -> Option<ScatterResult> {
        let unit_vector = &ray.direction().unit_vector();
        let surface_normal = geometry.surface_normal(&ray, distance);
        let reflected = reflect(&unit_vector, &surface_normal);

        let mut rng = rand::thread_rng();

        let uvn = Vector::dot(&unit_vector, &surface_normal);

        // Determine whether we are going from air to the entity or vv
        // TODO This current does not support refraction from inside one entity to another
        let (sign, n_i, n_t) = if uvn > 0.0 {
            (-1.0, self.refractive_index, REFRACTIVE_INDEX_OF_AIR)
        } else {
            (1.0, REFRACTIVE_INDEX_OF_AIR, self.refractive_index)
        };

        let cosine = - sign * n_i * uvn;
        let reflect_prob = reflectivity_schlick_approx(cosine, n_i);
        let reflect_rand: f64 = rng.gen();
        let should_reflect = reflect_rand < reflect_prob;

        let maybe_refracted = if should_reflect {
            None
        } else {
            refract(
                &unit_vector,
                &(sign * surface_normal),
                n_i / n_t,
            )
        };

        let hit_point = ray.point(distance);
        let ray = match maybe_refracted {
            Some(refracted) => {
                Ray::new(hit_point, refracted)
            },
            None => {
                Ray::new(hit_point, reflected)
            },
        };

        Some(ScatterResult { ray, attenuation: DIELECTRIC_ATTENUATION.clone() })
    }
}
