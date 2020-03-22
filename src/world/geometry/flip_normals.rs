use camera::Ray;
use data::assets::Assets;
use failure::Error;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::geometry::{Geometry, HitResult};

#[derive(Serialize, Deserialize)]
pub struct FlipNormals {
    geometry: Box<dyn Geometry>,
}

impl FlipNormals {
    pub fn new(geometry: Box<dyn Geometry>) -> FlipNormals {
        FlipNormals { geometry }
    }
}

#[typetag::serde]
impl Geometry for FlipNormals {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult {
        match self.geometry.hit(ray, tmin, tmax) {
            HitResult::Miss => HitResult::Miss,
            HitResult::Hit {
                distance,
                ray,
                point,
                surface_normal,
                material,
                texture_coords,
            } => HitResult::Hit {
                distance,
                ray,
                point,
                surface_normal: -surface_normal,
                material,
                texture_coords,
            },
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        self.geometry.bounding_box(time_start, time_end)
    }

    fn validate(&self, assets: &Assets) -> Result<(), Error> {
        self.geometry.validate(assets)
    }
}
