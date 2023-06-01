use clap::Parser;
use eyre::{eyre, WrapErr};
use std::path::PathBuf;
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// validate argument without actual side effects.
    #[arg(short, long, required = true, default_value_t = true)]
    dryrun: bool,

    /// The backend to use. ("redis")
    #[arg(short, long)]
    backend: String,

    /// The path to confd configs. ("/etc/confd")
    #[arg(short, long, value_name = "FILE", required = true)]
    confdir: Option<PathBuf>,

    /// The backend polling interval in seconds. (600)
    #[arg(short, long, default_value_t = 600)]
    interval: u16,

    /// List of backend nodes. (["http://127.0.0.1:4001"])
    #[arg(short, long)]
    nodes: Vec<String>,

    /// Enable watch support.
    #[arg(short, long)]
    watch: bool,

    /// The backend URI scheme. ("http" or "https"). defaults to "http"
    #[arg(short, long, default_value_t = String::from("http"))]
    scheme: String,

    /// level which confd should log messages ("info")
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    // non-MVP related configurations
    /// The name of the resource record.
    #[arg(short, long)]
    srv_domain: Option<String>,

    /// The SRV record to search for backends nodes.
    #[arg(short, long)]
    srv_record: Option<String>,

    /// Auth bearer token to use.
    #[arg(short, long)]
    auth_token: Option<String>,

    /// sync without check_cmd and reload_cmd.
    #[arg(short, long)]
    sync_only: Option<String>,

    /// The client CA key file.
    #[arg(short, long)]
    client_cakeys: Option<PathBuf>,

    /// The client cert file.
    #[arg(short, long)]
    client_cert: Option<PathBuf>,

    /// The client key file.
    #[arg(short, long)]
    client_key: Option<PathBuf>,

    // **Redis related config**
    /// The separator to replace '/' with when looking up keys in the backend, prefixed '/' will also be removed (only used with -backend=redis)
    #[arg(short, long, default_value_t=String::from("/"))]
    separator: String,

    // **Dynamo related config**
    /// The name of the DynamoDB table (only used with -backend=dynamodb).
    #[arg(short, long)]
    username: Option<String>,

    // Shared-only config
    /// The username to authenticate as (only used with vault and etcd backends).
    #[arg(short, long)]
    table: Option<String>,

    /// The password to authenticate with (only used with vault and etcd backends).
    #[arg(short, long)]
    password: Option<String>,

    // **Vault related config**
    /// Vault auth backend type to use.
    #[arg(short, long)]
    auth_type: Option<String>,

    /// Vault app-id to use with the app-id backend (only used with -backend=vault and auth-type=app-id).
    #[arg(short, long)]
    app_id: Option<String>,

    /// Vault user-id to use with the app-id backend (only used with -backend=value and auth-type=app-id).
    #[arg(short, long)]
    user_id: Option<String>,

    /// Vault role-id to use with the AppRole, Kubernetes backends (only used with -backend=vault and either auth-type=app-role or auth-type=kubernetes).
    #[arg(short, long)]
    role_id: Option<String>,

    /// Vault secret-id to use with the AppRole backend (only used with -backend=vault and auth-type=app-role).
    #[arg(short, long)]
    secret_id: Option<String>,

    /// Vault mount path of the auth method (only used with -backend=vault).
    #[arg(short, long)]
    path: Option<PathBuf>,
}

trait CommandExecute {
    fn execute(self) -> eyre::Result<()>;
}

impl CommandExecute for CLI {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        // todo!("this is the entry into the app where we");
        if self.dryrun {
            println!("dry running here");
        } else {
            println!("running here");

        }

        Ok(())
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
