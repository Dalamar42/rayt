use view::Camera;
use data::vector::Vector;
use data::colour::Colour;
use volumes::volume::Volume;
use volumes::geometry::Sphere;
use volumes::materials::{Lambertian, Metal};

pub struct Config {
    pub width: u64,
    pub height: u64,
    pub camera: Camera,
    pub volumes: Vec<Volume>,
}

pub fn build_config() -> Config {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vector {x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vector {x: 4.0, y: 0.0, z: 0.0};
    let vertical = Vector {x: 0.0, y: 2.0, z: 0.0};
    let origin = Vector {x: 0.0, y: 0.0, z: 0.0};

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    let volumes: Vec<Volume> = vec![
        Volume {
            geometry: Box::from(Sphere {
                centre: Vector {x: 0.0, y: 0.0, z: -1.0},
                radius: 0.5,
            }),
            material: Box::from(Lambertian {
                albedo: Colour {r: 0.8, g: 0.3, b: 0.3},
            }),
        },
        Volume {
            geometry: Box::from(Sphere {
                centre: Vector { x: -1.0, y: 0.0, z: -1.0 },
                radius: 0.5,
            }),
            material: Box::from(Metal {
                albedo: Colour { r: 0.8, g: 0.8, b: 0.8 },
                fuzz: 0.3,
            }),
        },
        Volume {
            geometry: Box::from(Sphere {
                centre: Vector { x: 1.0, y: 0.0, z: -1.0 },
                radius: 0.5,
            }),
            material: Box::from(Metal {
                albedo: Colour {r: 0.8, g: 0.6, b: 0.2},
                fuzz: 1.0,
            }),
        },
        Volume {
            geometry: Box::from(Sphere {
                centre: Vector {x: 0.0, y: -100.5, z: -1.0},
                radius: 100.0,
            }),
            material: Box::from(Lambertian {
                albedo: Colour {r: 0.8, g: 0.8, b: 0.0},
            }),
        },
    ];

    Config {width, height, camera, volumes}
}
