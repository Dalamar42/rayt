extern crate core;
extern crate assert_approx_eq;
#[macro_use] extern crate itertools;
extern crate rand;

mod io;
mod data;
mod view;
mod world;
mod config;
mod colouriser;
mod imager;

use config::build_config;
use imager::build_image;
use colouriser::build_colouriser;

fn main() {
    let config = build_config();
    let colouriser = build_colouriser();
    let test_image = build_image(colouriser, &config);

    io::write_image_as_ppm(test_image).expect("Error");
}
