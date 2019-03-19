use world::entity::Entity;

pub mod geometry;
pub mod materials;
pub mod entity;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub volumes: Vec<Entity>,
}
