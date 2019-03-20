use world::background::Background;
use world::entity::Entity;

pub mod background;
pub mod entity;
pub mod geometry;
pub mod materials;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub background: Background,
    pub volumes: Vec<Entity>,
}
