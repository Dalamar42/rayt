use camera::Ray;
use data::assets::Assets;
use data::colour::Colour;
use data::vector::Vector;
use sampling::{random_point_in_unit_sphere, uniform};
use std::f64::consts::PI;
use world::texture::Texture;

#[derive(Debug)]
pub struct ScatterResult {
    ray: Ray,
    attenuation: Colour,
    pdf: f64,
}

impl ScatterResult {
    pub fn new(ray: Ray, attenuation: Colour, pdf: f64) -> ScatterResult {
        ScatterResult {
            ray,
            attenuation,
            pdf,
        }
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn attenuation(&self) -> &Colour {
        &self.attenuation
    }

    pub fn pdf(&self) -> f64 {
        self.pdf
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Material {
    Lambertian {
        albedo: Texture,
    },
    Metal {
        albedo: Colour,
        fuzz: f64,
    },
    Dielectric {
        // Air: 1.0, Glass: 1.3-1.7, Diamond: 2.4
        refractive_index: f64,
    },
    DiffuseLight {
        emit: Texture,
    },
    Isotropic {
        albedo: Texture,
    },
}

impl Material {
    pub fn scattering_pdf(&self, surface_normal: &Vector, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian { .. } => scattering_pdf_lambertian(surface_normal, scattered),
            _ => 1.0,
        }
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        hit_point: &Vector,
        surface_normal: &Vector,
        texture_coords: (f64, f64),
        assets: &Assets,
    ) -> Option<ScatterResult> {
        match self {
            Material::Lambertian { albedo } => scatter_lambertian(
                &albedo,
                ray,
                hit_point,
                surface_normal,
                texture_coords,
                assets,
            ),
            Material::Metal { albedo, fuzz } => {
                scatter_metal(&albedo, *fuzz, ray, hit_point, surface_normal)
            }
            Material::Dielectric { refractive_index } => {
                scatter_dielectric(*refractive_index, ray, hit_point, surface_normal)
            }
            Material::DiffuseLight { .. } => None,
            Material::Isotropic { albedo } => {
                scatter_isotropic(&albedo, ray, hit_point, texture_coords, assets)
            }
        }
    }

    pub fn emitted(&self, texture_coords: (f64, f64), point: &Vector, assets: &Assets) -> Colour {
        match self {
            Material::DiffuseLight { emit } => emit.value(texture_coords, point, assets),
            _ => Colour::new(0.0, 0.0, 0.0),
        }
    }

    pub fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        match self {
            Material::Lambertian { albedo } => {
                albedo.validate(assets)?;
                Ok(())
            }
            _ => Ok(()),
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
        return Some(refracted);
    }

    None
}

fn scattering_pdf_lambertian(surface_normal: &Vector, scattered: &Ray) -> f64 {
    let mut cosine = Vector::dot(surface_normal, &scattered.direction().unit_vector());
    if cosine < 0.0 {
        cosine = 0.0;
    }
    cosine / PI
}

fn scatter_lambertian(
    albedo: &Texture,
    ray: &Ray,
    hit_point: &Vector,
    surface_normal: &Vector,
    texture_coords: (f64, f64),
    assets: &Assets,
) -> Option<ScatterResult> {
    let diffuse = random_point_in_unit_sphere();
    let target = hit_point + surface_normal + diffuse;

    let ray = Ray::new(*hit_point, target - hit_point, ray.time());
    let pdf = Vector::dot(surface_normal, &ray.direction().unit_vector()) / PI;

    Some(ScatterResult::new(
        ray,
        albedo.value(texture_coords, &hit_point, &assets),
        pdf,
    ))
}

fn scatter_metal(
    albedo: &Colour,
    fuzz: f64,
    ray: &Ray,
    hit_point: &Vector,
    surface_normal: &Vector,
) -> Option<ScatterResult> {
    let unit_vector = ray.direction().unit_vector();
    let reflected = reflect(&unit_vector, &surface_normal);

    let ray = Ray::new(
        *hit_point,
        reflected + fuzz * random_point_in_unit_sphere(),
        ray.time(),
    );

    if Vector::dot(&ray.direction(), &surface_normal) <= 0.0 {
        return None;
    }

    Some(ScatterResult::new(ray, *albedo, 1.0))
}

const REFRACTIVE_INDEX_OF_AIR: f64 = 1.0;
const DIELECTRIC_ATTENUATION: [f64; 3] = [1.0, 1.0, 1.0];

fn reflectivity_schlick_approx(cosine: f64, n_i: f64, n_t: f64) -> f64 {
    let r0 = (n_i - n_t) / (n_i + n_t);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

fn scatter_dielectric(
    refractive_index: f64,
    ray: &Ray,
    hit_point: &Vector,
    surface_normal: &Vector,
) -> Option<ScatterResult> {
    let unit_vector = ray.direction().unit_vector();
    let reflected = reflect(&unit_vector, &surface_normal);

    let uvn = Vector::dot(&unit_vector, &surface_normal);

    // Determine whether we are going from air to the geometry or vv
    // This current does not support refraction from inside one geometry to another
    let (sign, n_i, n_t) = if uvn > 0.0 {
        (-1.0, refractive_index, REFRACTIVE_INDEX_OF_AIR)
    } else {
        (1.0, REFRACTIVE_INDEX_OF_AIR, refractive_index)
    };

    let cosine = -sign * uvn;
    let reflect_prob = reflectivity_schlick_approx(cosine, n_i, n_t);
    let reflect_rand: f64 = uniform();
    let should_reflect = reflect_rand < reflect_prob;

    let maybe_refracted = if should_reflect {
        None
    } else {
        refract(&unit_vector, &(sign * surface_normal), n_i / n_t)
    };

    let ray = match maybe_refracted {
        Some(refracted) => Ray::new(*hit_point, refracted, ray.time()),
        None => Ray::new(*hit_point, reflected, ray.time()),
    };

    Some(ScatterResult::new(
        ray,
        Colour::new(
            DIELECTRIC_ATTENUATION[0],
            DIELECTRIC_ATTENUATION[1],
            DIELECTRIC_ATTENUATION[2],
        ),
        1.0,
    ))
}

fn scatter_isotropic(
    albedo: &Texture,
    ray: &Ray,
    hit_point: &Vector,
    texture_coords: (f64, f64),
    assets: &Assets,
) -> Option<ScatterResult> {
    let scattered = Ray::new(*hit_point, random_point_in_unit_sphere(), ray.time());
    let attenuation = albedo.value(texture_coords, hit_point, assets);
    Some(ScatterResult::new(scattered, attenuation, 1.0))
}
