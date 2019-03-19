use clap::{App, SubCommand, AppSettings};
use std::error::Error;

pub enum CliCommand {
    RENDER,
}

pub struct CliConfig {
    pub command: CliCommand,
}

pub fn get_cli_config() -> Result<CliConfig, Box<Error>> {
    let matches = App::new("Ray tracer")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .subcommands(vec![
            SubCommand::with_name("render")
                .about("renders an image")
        ])
        .get_matches();

    match matches.subcommand_matches("render") {
        Some(_) => Result::Ok(CliConfig {command: CliCommand::RENDER}),
        None => bail!("Invalid command line arguments"),
    }
}