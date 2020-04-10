use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::world::texture::Texture;

mod dielectric;
mod isotropic;
mod lambertian;
mod metal;

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

    pub fn albedo(&self) -> &Colour {
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
            Material::Lambertian { .. } => lambertian::scattering_pdf(surface_normal, scattered),
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
            Material::Lambertian { albedo } => lambertian::scatter(
                &albedo,
                ray,
                hit_point,
                surface_normal,
                texture_coords,
                assets,
            ),
            Material::Metal { albedo, fuzz } => {
                metal::scatter(&albedo, *fuzz, ray, hit_point, surface_normal)
            }
            Material::Dielectric { refractive_index } => {
                dielectric::scatter(*refractive_index, ray, hit_point, surface_normal)
            }
            Material::DiffuseLight { .. } => None,
            Material::Isotropic { albedo } => {
                isotropic::scatter(&albedo, ray, hit_point, texture_coords, assets)
            }
        }
    }

    pub fn emitted(
        &self,
        front_face: bool,
        texture_coords: (f64, f64),
        point: &Vector,
        assets: &Assets,
    ) -> Colour {
        if !front_face {
            return Colour::new(0.0, 0.0, 0.0);
        }

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
