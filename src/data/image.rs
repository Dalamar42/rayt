use data::colour::Colour;

#[derive(Debug)]
pub struct Image {
    pub pixels: Vec<Colour>,
    pub num_rows: u64,
    pub num_cols: u64,
}
