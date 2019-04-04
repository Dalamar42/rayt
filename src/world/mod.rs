use data::assets::Assets;
use failure::Error;
use world::background::Background;
use world::geometry::Geometry;

pub mod background;
pub mod geometry;
pub mod materials;
pub mod texture;

#[derive(Serialize, Deserialize)]
pub struct WorldSave {
    background: Background,
    geometries: Vec<Box<dyn Geometry>>,
}

impl WorldSave {
    pub fn new(background: Background, geometries: Vec<Box<dyn Geometry>>) -> WorldSave {
        WorldSave {
            background,
            geometries,
        }
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn drain_geometries(&mut self) -> Vec<Box<dyn Geometry>> {
        if self.geometries.is_empty() {
            panic!("Geometries have already been drained")
        }
        let range = 0..self.geometries.len();
        self.geometries.drain(range).collect()
    }

    pub fn validate(&self, assets: &Assets) -> Result<(), Error> {
        for geometry in &self.geometries {
            geometry.validate(assets)?
        }
        Ok(())
    }
}
