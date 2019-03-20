use world::entity::Entity;

pub mod entity;
pub mod geometry;
pub mod materials;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub volumes: Vec<Entity>,
}
