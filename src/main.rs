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
extern crate anyhow;
extern crate image;
extern crate thiserror;
extern crate typetag;

mod camera;
mod cli;
mod config;
mod data;
mod float;
mod io;
mod renderer;
mod scenes;
mod world;

use cli::{get_cli_config, CliCommand, ConfigPath, ImagePath, OutputPath};
use config::Config;
use console::style;
use data::assets::Assets;
use indicatif::{FormattedDuration, ProgressBar, ProgressStyle};
use io::{load_config, save_config};
use renderer::render;
use scenes::{build_scene_config, Scene};
use std::process;
use std::time::Instant;

const PROGRESS_BAR_STYLE: &str = "[{elapsed_precise}] [{bar:60.cyan/blue}] {percent}% ({eta})";

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", style("error:").red(), e);
        process::exit(1);
    }
}

fn run() -> Result<(), anyhow::Error> {
    let cli_config = get_cli_config()?;

    match cli_config.command() {
        CliCommand::RENDER {
            width,
            output_path,
            num_of_rays,
            num_of_threads,
            asset_paths,
        } => {
            run_render(
                &cli_config.config_path(),
                *width,
                &output_path,
                *num_of_rays,
                *num_of_threads,
                asset_paths,
            )?;
        }
        CliCommand::GENERATE { scene } => {
            run_generate(&scene, &cli_config.config_path())?;
        }
    };

    Ok(())
}

fn run_render(
    config_path: &ConfigPath,
    width: u32,
    output_path: &OutputPath,
    num_of_rays: u64,
    num_of_threads: usize,
    asset_paths: &[ImagePath],
) -> Result<(), anyhow::Error> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_of_threads)
        .build_global()?;

    let started = Instant::now();

    let mut step_logger = StepLogger::new(6);

    step_logger.log("Loading image yaml");
    let config_save = load_config(config_path)?;

    step_logger.log("Loading assets");
    let assets = Assets::new(asset_paths)?;

    step_logger.log("Validating assets");
    config_save.validate(&assets)?;

    step_logger.log("Creating config (constructing BVH)");
    let config = config_save.into_config(width, num_of_rays, assets);

    step_logger.log("Rendering");
    let progress_bar = progress_bar(&config);
    let image = render(&config, &progress_bar);

    step_logger.log("Printing image");
    io::write_image(image, output_path)?;

    println!("Done in {}", FormattedDuration(started.elapsed()));

    Ok(())
}

fn run_generate(scene: &Scene, config_path: &ConfigPath) -> Result<(), anyhow::Error> {
    let mut step_logger = StepLogger::new(2);

    step_logger.log("Generating scene");
    let config_save = build_scene_config(scene)?;

    step_logger.log("Writing image yaml");
    save_config(config_path, config_save)?;
    Ok(())
}

fn progress_bar(config: &Config) -> ProgressBar {
    let progress_style = ProgressStyle::default_bar()
        .template(PROGRESS_BAR_STYLE)
        .progress_chars("##-");
    let bar_size = u64::from(config.height() * config.width());
    let progress_bar = ProgressBar::new(bar_size);
    progress_bar.set_style(progress_style);
    progress_bar.tick();
    progress_bar.set_draw_delta(bar_size / 1000);

    progress_bar
}

struct StepLogger {
    step: u8,
    num_of_steps: u8,
}

impl StepLogger {
    fn new(num_of_steps: u8) -> StepLogger {
        StepLogger {
            step: 1,
            num_of_steps,
        }
    }

    fn log(&mut self, msg: &str) {
        assert!(self.step <= self.num_of_steps);

        println!(
            "{}{}{}{}{} {}...",
            style("[").bold().dim(),
            style(self.step.to_string()).bold().dim(),
            style("/").bold().dim(),
            style(self.num_of_steps.to_string()).bold().dim(),
            style("]").bold().dim(),
            msg,
        );

        self.step += 1
    }
}
