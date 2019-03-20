use clap::{App, AppSettings, Arg, SubCommand};

pub enum CliCommand {
    RENDER { width: u64, output_path: String },
    GENERATE,
}

pub struct CliConfig {
    pub command: CliCommand,
    pub config_path: String,
}

pub fn get_cli_config() -> CliConfig {
    let matches = App::new("Ray tracer")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
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
                ),
            SubCommand::with_name("generate").about("generate a random image config yaml"),
        ])
        .get_matches();

    let config_path = String::from(matches.value_of("config").unwrap());

    if let Some(subcommand) = matches.subcommand_matches("render") {
        let width = subcommand
            .value_of("width")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let output_path = String::from(subcommand.value_of("output_path").unwrap());
        assert!(output_path.ends_with(".ppm"));

        return CliConfig {
            command: CliCommand::RENDER { width, output_path },
            config_path,
        };
    }
    if matches.subcommand_matches("generate").is_some() {
        return CliConfig {
            command: CliCommand::GENERATE,
            config_path,
        };
    }

    panic!("Unable to parse CLI args")
}
