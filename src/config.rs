use view::Camera;
use volumes::Hitable;
use data::vector::Vector;
use volumes::*;

pub struct Config {
    pub width: u64,
    pub height: u64,
    pub camera: Camera,
    pub volumes: Vec<Box<Hitable>>,
}

pub fn build_config() -> Config {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vector {x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vector {x: 4.0, y: 0.0, z: 0.0};
    let vertical = Vector {x: 0.0, y: 2.0, z: 0.0};
    let origin = Vector {x: 0.0, y: 0.0, z: 0.0};

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    let volumes: Vec<Box<Hitable>> = vec![
        Box::from(Sphere {
            centre: Vector {x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
        }),
        Box::from(Sphere {
            centre: Vector {x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
        }),
    ];

    Config {width, height, camera, volumes}
}
