mod check;
mod generate;
mod init;
mod license;
mod notice_toml;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[clap(hide = true, num_args = 1, required = true)]
    cargo_cmd: String,

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

    fn handle_subcommand(subcommand: SubCommands) -> anyhow::Result<()> {
        match subcommand {
            SubCommands::Init => init::init()?,
            SubCommands::Check => check::check()?,
            SubCommands::Generate => generate::generate()?,
        }

        Ok(())
    }

    handle_subcommand(args.subcommand)?;

    Ok(())
}
