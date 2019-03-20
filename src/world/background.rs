use data::colour::Colour;

#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    pub top: Colour,
    pub bottom: Colour,
}
