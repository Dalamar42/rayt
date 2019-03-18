use world::entity::Entity;

pub mod geometry;
pub mod materials;
pub mod entity;

pub struct World {
    pub volumes: Vec<Entity>,
}
