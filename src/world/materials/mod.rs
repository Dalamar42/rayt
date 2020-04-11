use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::pdf::Pdf;
use crate::world::geometry::HitResult;
use crate::world::texture::Texture;

mod dielectric;
mod isotropic;
mod lambertian;
mod metal;

pub enum ScatterResult {
    Specular {
        attenuation: Colour,
        ray: Ray,
    },
    Diffuse {
        attenuation: Colour,
        pdf: Pdf<'static>,
    },
}

impl ScatterResult {
    pub fn specular(attenuation: Colour, ray: Ray) -> ScatterResult {
        ScatterResult::Specular { attenuation, ray }
    }

    pub fn diffuse(attenuation: Colour, pdf: Pdf<'static>) -> ScatterResult {
        ScatterResult::Diffuse { attenuation, pdf }
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

    pub fn scatter(&self, hit: &HitResult, assets: &Assets) -> Option<ScatterResult> {
        match self {
            Material::Lambertian { albedo } => lambertian::scatter(&albedo, hit, assets),
            Material::Metal { albedo, fuzz } => metal::scatter(&albedo, *fuzz, hit),
            Material::Dielectric { refractive_index } => {
                dielectric::scatter(*refractive_index, hit)
            }
            Material::DiffuseLight { .. } => None,
            Material::Isotropic { albedo } => isotropic::scatter(&albedo, hit, assets),
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

    pub fn is_attractor(&self) -> bool {
        match self {
            Material::DiffuseLight { .. } => true,
            _ => false,
        }
    }
}
