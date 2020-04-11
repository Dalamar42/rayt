use crate::camera::{CameraSave, Lens};
use crate::config::ConfigSave;
use crate::data::colour::Colour;
use crate::data::vector::Vector;
use crate::sampling::uniform;
use crate::world::background::Background;
use crate::world::geometry::bounding_volume_hierarchy::BoundingVolumeHierarchyNode;
use crate::world::geometry::cube::Cube;
use crate::world::geometry::medium::ConstantMedium;
use crate::world::geometry::rectangle::XzRect;
use crate::world::geometry::sphere::{MovingSphere, Sphere};
use crate::world::geometry::Geometry;
use crate::world::materials::Material;
use crate::world::texture::perlin::build_noise_config;
use crate::world::texture::Texture;
use crate::world::WorldSave;

pub fn build() -> Result<ConfigSave, anyhow::Error> {
    let aspect = 1.0;

    let camera = CameraSave::new(
        &Vector::new(478.0, 278.0, -600.0),
        &Vector::new(278.0, 278.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(40.0, 0.0, 10.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Geometry> = Vec::with_capacity(30);

    geometries.push(BoundingVolumeHierarchyNode::build(ground_boxes(), 0.0, 1.0));
    geometries.push(light());
    geometries.push(moving_sphere());
    geometries.push(dielectric_a());
    geometries.push(metal_a());

    let (boundary, medium) = subsurface_material_a();
    geometries.push(boundary);
    geometries.push(medium);

    let (boundary, medium) = subsurface_material_b();
    geometries.push(boundary);
    geometries.push(medium);

    geometries.push(earth());
    geometries.push(perlin());

    geometries.push(
        BoundingVolumeHierarchyNode::build(sphere_cube(), 0.0, 1.0)
            .rotate_y(15.0)
            .unwrap()
            .translate(Vector::new(-100.0, 270.0, 395.0)),
    );

    let black = Colour::new(0.0, 0.0, 0.0);
    let background = Background::new(black, black);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}

fn ground_boxes() -> Vec<Geometry> {
    let ground = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.48, 0.83, 0.53),
        },
    };
    let mut boxlist: Vec<Geometry> = Vec::with_capacity(10000);
    let nb = 20;
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (uniform::<f64>() + 0.01);
            let z1 = z0 + w;
            boxlist.push(Cube::build(
                Vector::new(x0, y0, z0),
                Vector::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    boxlist
}

fn light() -> Geometry {
    let light = Material::DiffuseLight {
        emit: Texture::Constant {
            colour: Colour::new(7.0, 7.0, 7.0),
        },
    };
    XzRect::build((123.0, 423.0), (147.0, 412.0), 554.0, light).flip()
}

fn moving_sphere() -> Geometry {
    let centre_start = Vector::new(400.0, 400.0, 200.0);
    let centre_end = centre_start + Vector::new(30.0, 0.0, 0.0);
    let material = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.7, 0.3, 0.1),
        },
    };
    MovingSphere::build(centre_start, 0.0, centre_end, 1.0, 50.0, material)
}

fn dielectric_a() -> Geometry {
    Sphere::build(
        Vector::new(260.0, 150.0, 45.0),
        50.0,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    )
}

fn metal_a() -> Geometry {
    Sphere::build(
        Vector::new(0.0, 150.0, 145.0),
        50.0,
        Material::Metal {
            albedo: Colour::new(0.8, 0.8, 0.9),
            fuzz: 10.0,
        },
    )
}

fn subsurface_material_a() -> (Geometry, Geometry) {
    let boundary = Sphere::build(
        Vector::new(360.0, 150.0, 145.0),
        70.0,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    );
    let medium = ConstantMedium::build(
        boundary.clone(),
        0.2,
        Texture::Constant {
            colour: Colour::new(0.2, 0.4, 0.9),
        },
    );
    (boundary, medium)
}

fn subsurface_material_b() -> (Geometry, Geometry) {
    let boundary = Sphere::build(
        Vector::new(0.0, 0.0, 0.0),
        5000.0,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    );
    let medium = ConstantMedium::build(
        boundary.clone(),
        0.0001,
        Texture::Constant {
            colour: Colour::new(1.0, 1.0, 1.0),
        },
    );
    (boundary, medium)
}

fn earth() -> Geometry {
    let material = Material::Lambertian {
        albedo: Texture::Image {
            asset_name: String::from("earth.jpg"),
        },
    };
    Sphere::build(Vector::new(400.0, 200.0, 400.0), 100.0, material)
}

fn perlin() -> Geometry {
    let material = Material::Lambertian {
        albedo: Texture::Noise {
            base_colour: Colour::new(1.0, 1.0, 1.0),
            scale: 5.0,
            noisiness: 10.0,
            noise_config: build_noise_config(),
        },
    };
    Sphere::build(Vector::new(220.0, 280.0, 300.0), 80.0, material)
}

fn sphere_cube() -> Vec<Geometry> {
    let white = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.73, 0.73, 0.73),
        },
    };

    let mut boxlist: Vec<Geometry> = Vec::with_capacity(1000);

    for _ in 0..1000 {
        boxlist.push(Sphere::build(
            Vector::new(
                165.0 * uniform::<f64>(),
                165.0 * uniform::<f64>(),
                165.0 * uniform::<f64>(),
            ),
            10.0,
            white.clone(),
        ))
    }

    boxlist
}
