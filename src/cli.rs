use crate::io::SUPPORTED_IMAGE_EXT;
use crate::scenes::Scene;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::str::FromStr;
use thiserror::Error;

pub struct ConfigPath(String);
pub struct OutputPath(String);
pub struct ImagePath(String);

impl ConfigPath {
    pub fn path(&self) -> &str {
        &self.0
    }
}

impl OutputPath {
    pub fn path(&self) -> &str {
        &self.0
    }
}

impl ImagePath {
    pub fn path(&self) -> &str {
        &self.0
    }

    pub fn file_name(&self) -> &str {
        &self.0.split('/').last().unwrap()
    }
}

pub enum CliCommand {
    RENDER {
        width: u32,
        output_path: OutputPath,
        num_of_rays: u64,
        num_of_threads: usize,
        asset_paths: Vec<ImagePath>,
    },
    GENERATE {
        scene: Scene,
    },
}

pub struct CliConfig {
    command: CliCommand,
    config_path: ConfigPath,
}

impl CliConfig {
    pub fn command(&self) -> &CliCommand {
        &self.command
    }

    pub fn config_path(&self) -> &ConfigPath {
        &self.config_path
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
enum CliParsingError {
    #[error("invalid value <{value}> for arg <{arg}>")]
    InvalidValue { arg: String, value: String },
    #[error("Config path <{0}> must end in .yaml")]
    InvalidConfigPath(String),
    #[error("Output path <{output_path}> must end in one of {supported_extensions:?}")]
    InvalidOutputPath {
        output_path: String,
        supported_extensions: Vec<String>,
    },
}

pub fn get_cli_config() -> Result<CliConfig, anyhow::Error> {
    let matches = App::new("Ray tracer")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .required(true)
                .help("path to image config yaml"),
        )
        .subcommands(vec![
            SubCommand::with_name("render")
                .about("renders an image")
                .arg(
                    Arg::with_name("width")
                        .short("w")
                        .long("width")
                        .takes_value(true)
                        .required(true)
                        .help("the output image width"),
                )
                .arg(
                    Arg::with_name("output_path")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .required(true)
                        .default_value("image.ppm")
                        .help("the output image path"),
                )
                .arg(
                    Arg::with_name("rays")
                        .short("r")
                        .long("rays")
                        .takes_value(true)
                        .required(true)
                        .default_value("100")
                        .help("the number of rays to generate per pixel"),
                )
                .arg(
                    Arg::with_name("threads")
                        .short("t")
                        .long("threads")
                        .takes_value(true)
                        .required(true)
                        .default_value("4")
                        .help("the number of threads to create for the renderer"),
                )
                .arg(
                    Arg::with_name("asset")
                        .short("a")
                        .long("asset")
                        .takes_value(true)
                        .required(false)
                        .multiple(true)
                        .help(
                            "the paths to image assets needed by the selected scene. The \
                             filename must be unique amongst all loaded assets",
                        ),
                ),
            SubCommand::with_name("generate")
                .about("generate a random image config yaml")
                .arg(
                    Arg::with_name("scene")
                        .short("s")
                        .long("scene")
                        .takes_value(true)
                        .required(true)
                        .possible_values(&Scene::variants())
                        .case_insensitive(true)
                        .help("the name of the scene to generate"),
                ),
        ])
        .get_matches();

    let config_path = String::from(matches.value_of("config").unwrap());

    validate_config_path(&config_path)?;

    if let Some(subcommand) = matches.subcommand_matches("render") {
        let width = parse::<u32>(subcommand, "width")?;
        let output_path = String::from(subcommand.value_of("output_path").unwrap());
        let num_of_rays = parse::<u64>(subcommand, "rays")?;
        let num_of_threads = parse::<usize>(subcommand, "threads")?;
        let asset_paths: Vec<ImagePath> = subcommand
            .values_of("asset")
            .unwrap_or_default()
            .map(|path| ImagePath(String::from(path)))
            .collect();

        validate_output_path(&output_path)?;

        return Ok(CliConfig {
            command: CliCommand::RENDER {
                width,
                output_path: OutputPath(output_path),
                num_of_rays,
                num_of_threads,
                asset_paths,
            },
            config_path: ConfigPath(config_path),
        });
    }
    if let Some(subcommand) = matches.subcommand_matches("generate") {
        let scene = parse::<Scene>(subcommand, "scene")?;

        return Ok(CliConfig {
            command: CliCommand::GENERATE { scene },
            config_path: ConfigPath(config_path),
        });
    }

    // Clap should have errored before we get here
    panic!("Unable to parse CLI args")
}

fn validate_config_path(config_path: &str) -> Result<(), CliParsingError> {
    if !config_path.ends_with(".yaml") {
        return Err(CliParsingError::InvalidConfigPath(config_path.to_string()));
    }
    Ok(())
}

fn validate_output_path(output_path: &str) -> Result<(), CliParsingError> {
    if !SUPPORTED_IMAGE_EXT
        .iter()
        .any(|ext| output_path.ends_with(ext))
    {
        return Err(CliParsingError::InvalidOutputPath {
            output_path: output_path.to_string(),
            supported_extensions: SUPPORTED_IMAGE_EXT
                .iter()
                .map(|ext| (*ext).to_string())
                .collect(),
        });
    }
    Ok(())
}

fn parse<T: FromStr>(matches: &ArgMatches, arg: &str) -> Result<T, CliParsingError> {
    let raw = matches.value_of(arg).unwrap();
    match raw.parse::<T>() {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(CliParsingError::InvalidValue {
            arg: String::from(arg),
            value: String::from(raw),
        }),
    }
}
