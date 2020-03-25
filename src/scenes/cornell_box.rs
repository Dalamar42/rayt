use camera::{CameraSave, Lens};
use config::ConfigSave;
use data::colour::Colour;
use data::vector::Vector;
use failure::Error;
use world::background::Background;
use world::geometry::cube::Cube;
use world::geometry::rectangle::{XyRect, XzRect, YzRect};
use world::geometry::Geometry;
use world::materials::Material;
use world::texture::Texture;
use world::WorldSave;

pub fn build() -> Result<ConfigSave, Error> {
    let aspect = 1.0;

    let camera = CameraSave::new(
        &Vector::new(278.0, 278.0, -800.0),
        &Vector::new(278.0, 278.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
        aspect,
        Lens::new(40.0, 0.0, 10.0),
        0.0,
        1.0,
    );

    let mut geometries: Vec<Box<dyn Geometry>> = Vec::with_capacity(8);

    let red = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.65, 0.05, 0.05),
        },
    };
    let white = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.73, 0.73, 0.73),
        },
    };
    let green = Material::Lambertian {
        albedo: Texture::Constant {
            colour: Colour::new(0.12, 0.45, 0.15),
        },
    };
    let light = Material::DiffuseLight {
        emit: Texture::Constant {
            colour: Colour::new(15.0, 15.0, 15.0),
        },
    };

    geometries.push(Box::new(
        YzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green).flip(),
    ));
    geometries.push(Box::new(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red)));
    geometries.push(Box::new(
        XzRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light).flip(),
    ));
    geometries.push(Box::new(
        XzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()).flip(),
    ));
    geometries.push(Box::new(XzRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    )));
    geometries.push(Box::new(
        XyRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()).flip(),
    ));
    geometries.push(Box::new(
        Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(165.0, 165.0, 165.0),
            white.clone(),
        )
        .rotate_y(-18.0)?
        .translate(Vector::new(130.0, 0.0, 65.0)),
    ));
    geometries.push(Box::new(
        Cube::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(165.0, 330.0, 165.0),
            white,
        )
        .rotate_y(15.0)?
        .translate(Vector::new(265.0, 0.0, 295.0)),
    ));

    let black = Colour::new(0.0, 0.0, 0.0);
    let background = Background::new(black, black);

    let world = WorldSave::new(background, geometries);

    Ok(ConfigSave::new(aspect, camera, world))
}
