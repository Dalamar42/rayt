use world::background::Background;
use world::entity::Entity;

pub mod background;
pub mod entity;
pub mod geometry;
pub mod materials;

#[derive(Serialize, Deserialize)]
pub struct World {
    background: Background,
    volumes: Vec<Entity>,
}

impl World {
    pub fn new(background: Background, volumes: Vec<Entity>) -> World {
        World {
            background,
            volumes,
        }
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn volumes(&self) -> &Vec<Entity> {
        &self.volumes
    }
}
