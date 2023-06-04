mod command;

use clap::Parser;
use eyre::{eyre, WrapErr};
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::prelude::*;

trait CommandExecute {
    fn execute(self) -> eyre::Result<()>;
}


/// CLI setup with sub commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[clap(subcommand)]
    subcommand: ConfdSubcommands,

    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
}

impl CommandExecute for CLI {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        self.subcommand.execute()
    }
}

#[derive(clap::Subcommand, Debug)]
enum ConfdSubcommands {
    Etcd(command::etcd::Etcd),
    Vault(command::vault::Vault),
}

impl CommandExecute for ConfdSubcommands {
    fn execute(self) -> eyre::Result<()> {
        use ConfdSubcommands::*;
        match self {
            Etcd(c) => c.execute(),
            Vault(c) => c.execute()
        }
    }
}


fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let confd_cli = CLI::parse();

    let fmt_layer = tracing_subscriber::fmt::Layer::new()
        // .with_ansi(atty::is(Stream::Stderr))
        .with_writer(std::io::stderr)
        .pretty();

    let filter_layer = match EnvFilter::try_from_default_env() {
        Ok(filter_layer) => filter_layer,
        Err(_) => {
            let log_level = match confd_cli.verbose {
                0 => "info",
                1 => "debug",
                _ => "trace",
            };
            let filter_layer = EnvFilter::new("warn");
            let filter_layer =
                filter_layer.add_directive(format!("confd-rs={}", log_level).parse()?);
            filter_layer
        }
    };

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    confd_cli.execute()
}
