extern crate core;
extern crate assert_approx_eq;
#[macro_use] extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate indicatif;
extern crate console;
#[macro_use] extern crate clap;
#[macro_use] extern crate simple_error;

mod io;
mod data;
mod view;
mod world;
mod config;
mod colouriser;
mod imager;
mod cli;

use config::{build_config, Config};
use imager::build_image;
use colouriser::build_colouriser;
use indicatif::{ProgressBar, ProgressStyle, HumanDuration};
use std::time::Instant;
use console::style;
use cli::{get_cli_config, CliCommand};
use std::error::Error;
use std::process;

const NUM_OF_THREADS: usize = 4;
const PROGRESS_BAR_STYLE: &str =
    "[{elapsed_precise}] [{bar:60.cyan/blue}] {percent}% ({eta})";

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let cli_config = get_cli_config()?;

    match cli_config.command {
        CliCommand::RENDER => {
            render()?;
        }
    };

    Ok(())
}

fn render() -> Result<(), Box<Error>> {
    rayon::ThreadPoolBuilder::new().num_threads(NUM_OF_THREADS).build_global().unwrap();

    let started = Instant::now();
    let config = build_config();
    let colouriser = build_colouriser();

    println!("{} Rendering...", style("[1/2]").bold().dim());
    let test_image = build_image(colouriser, &config, &progress_bar(&config));

    println!("{} Printing image...", style("[2/2]").bold().dim());
    io::write_image_as_ppm(test_image)?;

    println!("Done in {}", HumanDuration(started.elapsed()));

    Ok(())
}

fn progress_bar(config: &Config) -> ProgressBar {
    let progress_style = ProgressStyle::default_bar()
        .template(PROGRESS_BAR_STYLE)
        .progress_chars("##-");
    let bar_size = config.height * config.width;
    let progress_bar = ProgressBar::new(bar_size);
    progress_bar.set_style(progress_style.clone());
    progress_bar.tick();
    progress_bar.set_draw_delta(bar_size / 1000);

    progress_bar
}
