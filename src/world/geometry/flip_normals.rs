use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::{Geometry, HitResult, Hittable};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlipNormals {
    geometry: Box<Geometry>,
}

impl FlipNormals {
    pub fn build(geometry: Geometry) -> Geometry {
        Geometry::Flip(Box::from(FlipNormals {
            geometry: Box::from(geometry),
        }))
    }
}

impl Hittable for FlipNormals {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        self.geometry.hit(ray, tmin, tmax).map(|hit| HitResult {
            surface_normal: -hit.surface_normal,
            ..hit
        })
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        self.geometry.bounding_box(time_start, time_end)
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        self.geometry.validate(assets)
    }

    fn is_attractor(&self) -> bool {
        self.geometry.is_attractor()
    }

    fn pdf_value(&self, origin: &Vector, direction: &Vector) -> f64 {
        self.geometry.pdf_value(origin, direction)
    }

    fn random(&self, origin: &Vector) -> Vector {
        self.geometry.random(origin)
    }
}
