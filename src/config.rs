use view::Camera;
use data::vector::Vector;
use data::colour::Colour;
use world::entity::Entity;
use world::geometry::Sphere;
use world::materials::{Lambertian, Metal, Dielectric};
use world::World;

pub struct Config {
    pub width: u64,
    pub height: u64,
    pub camera: Camera,
    pub world: World,
}

pub fn build_config() -> Config {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vector { x: -2.0, y: -1.0, z: -1.0 };
    let horizontal = Vector { x: 4.0, y: 0.0, z: 0.0 };
    let vertical = Vector { x: 0.0, y: 2.0, z: 0.0 };
    let origin = Vector { x: 0.0, y: 0.0, z: 0.0 };

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);
    let world = build_world_a();

    Config {width, height, camera, world}
}

fn build_world_a() -> World {
    let volumes: Vec<Entity> = vec![
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector {x: 0.0, y: 0.0, z: -1.0},
                radius: 0.5,
            }),
            material: Box::from(Lambertian {
                albedo: Colour {r: 0.1, g: 0.2, b: 0.5},
            }),
        },
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector {x: 0.0, y: -100.5, z: -1.0},
                radius: 100.0,
            }),
            material: Box::from(Lambertian {
                albedo: Colour {r: 0.8, g: 0.8, b: 0.0},
            }),
        },
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector { x: 1.0, y: 0.0, z: -1.0 },
                radius: 0.5,
            }),
            material: Box::from(Metal {
                albedo: Colour {r: 0.8, g: 0.6, b: 0.2},
                fuzz: 0.1,
            }),
        },
        Entity {
            geometry: Box::from(Sphere {
                centre: Vector { x: -1.0, y: 0.0, z: -1.0 },
                radius: -0.45,
            }),
            material: Box::from(Dielectric{
                refractive_index: 1.5,
            }),
        },
    ];

    World {volumes}
}
