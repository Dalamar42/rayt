use data::colour::Colour;
use data::vector::Vector;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Texture {
    Constant {
        colour: Colour,
    },
    Checker {
        even: Box<Texture>,
        odd: Box<Texture>,
    },
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Vector) -> Colour {
        match self {
            Texture::Constant { colour } => colour.clone(),
            Texture::Checker { odd, even } => checker_texture(&odd, &even, u, v, &p),
        }
    }
}

fn checker_texture(odd: &Texture, even: &Texture, u: f64, v: f64, p: &Vector) -> Colour {
    let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
    if sines < 0.0 {
        odd.value(u, v, &p)
    } else {
        even.value(u, v, &p)
    }
}
