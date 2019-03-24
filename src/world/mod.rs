use world::background::Background;
use world::geometry::Geometry;

pub mod background;
pub mod geometry;
pub mod materials;

#[derive(Serialize, Deserialize)]
pub struct World {
    background: Background,
    geometries: Vec<Box<dyn Geometry>>,
}

impl World {
    pub fn new(background: Background, geometries: Vec<Box<dyn Geometry>>) -> World {
        World {
            background,
            geometries,
        }
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn geometries(&self) -> &Vec<Box<dyn Geometry>> {
        &self.geometries
    }
}
