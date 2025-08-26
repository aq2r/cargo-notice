mod check;
mod generate;
mod init;
mod license;
mod notice_toml;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(about = "Perform initial settings before using the check and generate commands.")]
    Init,

    #[command(about = "Check whether any licenses other than the one you set are included")]
    Check,

    #[command(
        about = "Outputs a list of licenses of dependent libraries to Markdown. (example: cargo notice generate > ThirdPartyLicense.md)"
    )]
    Generate,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        SubCommands::Init => init::init()?,
        SubCommands::Check => check::check()?,
        SubCommands::Generate => generate::generate()?,
    }

    Ok(())
}
