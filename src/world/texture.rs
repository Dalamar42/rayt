use data::colour::Colour;
use data::vector::Vector;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Texture {
    Constant { colour: Colour },
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Vector) -> &Colour {
        match self {
            Texture::Constant { colour } => &colour,
        }
    }
}
