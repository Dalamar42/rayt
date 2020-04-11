use crate::data::assets::Assets;
use crate::world::background::Background;
use crate::world::geometry::{Geometry, Hittable};

pub mod background;
pub mod geometry;
pub mod materials;
pub mod texture;

#[derive(Serialize, Deserialize)]
pub struct WorldSave {
    background: Background,
    geometries: Vec<Geometry>,
}

impl WorldSave {
    pub fn new(background: Background, geometries: Vec<Geometry>) -> WorldSave {
        WorldSave {
            background,
            geometries,
        }
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn drain_geometries(&mut self) -> Vec<Geometry> {
        if self.geometries.is_empty() {
            panic!("Geometries have already been drained")
        }
        let range = 0..self.geometries.len();
        self.geometries.drain(range).collect()
    }

    pub fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        for geometry in &self.geometries {
            geometry.validate(assets)?
        }
        Ok(())
    }
}
