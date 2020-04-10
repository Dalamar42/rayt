use crate::data::colour::Colour;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    top: Colour,
    bottom: Colour,
}

impl Background {
    pub fn new(top: Colour, bottom: Colour) -> Background {
        Background { top, bottom }
    }

    pub fn top(&self) -> &Colour {
        &self.top
    }

    pub fn bottom(&self) -> &Colour {
        &self.bottom
    }
}
