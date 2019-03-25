extern crate assert_approx_eq;
extern crate core;
#[macro_use]
extern crate itertools;
extern crate console;
extern crate indicatif;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate clap;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate typetag;
#[macro_use]
extern crate failure;
extern crate image;

mod camera;
mod cli;
mod config;
mod data;
mod generator;
mod io;
mod renderer;
mod world;

use cli::{get_cli_config, CliCommand};
use config::Config;
use console::style;
use failure::Error;
use generator::build_book_cover_config;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use io::{load_config, save_config};
use renderer::render;
use std::process;
use std::time::Instant;

const PROGRESS_BAR_STYLE: &str = "[{elapsed_precise}] [{bar:60.cyan/blue}] {percent}% ({eta})";

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", style("error:").red(), e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let cli_config = get_cli_config()?;

    match cli_config.command() {
        CliCommand::RENDER {
            width,
            output_path,
            num_of_rays,
            num_of_threads,
        } => {
            run_render(
                &cli_config.config_path(),
                *width,
                &output_path,
                *num_of_rays,
                *num_of_threads,
            )?;
        }
        CliCommand::GENERATE => {
            run_generate(&cli_config.config_path())?;
        }
    };

    Ok(())
}

fn run_render(
    config_path: &str,
    width: u32,
    output_path: &str,
    num_of_rays: u64,
    num_of_threads: usize,
) -> Result<(), Error> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_of_threads)
        .build_global()?;

    let started = Instant::now();

    println!("{} Loading image yaml...", style("[1/4]").bold().dim());
    let config_save = load_config(config_path)?;

    println!(
        "{} Creating config (constructing BVH)...",
        style("[2/4]").bold().dim()
    );
    let config = config_save.into_config(width, num_of_rays);

    println!("{} Rendering...", style("[3/4]").bold().dim());
    let progress_bar = progress_bar(&config);
    let test_image = render(&config, &progress_bar);

    println!("{} Printing image...", style("[4/4]").bold().dim());
    io::write_image(test_image, output_path)?;

    println!("Done in {}", HumanDuration(started.elapsed()));

    Ok(())
}

fn run_generate(config_path: &str) -> Result<(), Error> {
    let config_save = build_book_cover_config();
    save_config(config_path, config_save)?;
    Ok(())
}

fn progress_bar(config: &Config) -> ProgressBar {
    let progress_style = ProgressStyle::default_bar()
        .template(PROGRESS_BAR_STYLE)
        .progress_chars("##-");
    let bar_size = u64::from(config.height() * config.width());
    let progress_bar = ProgressBar::new(bar_size);
    progress_bar.set_style(progress_style.clone());
    progress_bar.tick();
    progress_bar.set_draw_delta(bar_size / 1000);

    progress_bar
}
