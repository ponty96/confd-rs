/// Connects to etcdb
use crate::CommandExecute;

#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Etcd {
    // Shared-only config
    /// The username to authenticate as (only used with vault and etcd backends).
    #[arg(short, long)]
    table: Option<String>,

    /// The password to authenticate with (only used with vault and etcd backends).
    #[arg(short, long)]
    password: Option<String>,

    #[arg(short, long, required = true, default_value_t = true)]
    dryrun: bool,

    /// The backend polling interval in seconds. (600)
    #[arg(short, long, default_value_t = 600)]
    interval: u16,

    /// Enable watch support.
    #[arg(short, long)]
    watch: bool,
}


impl CommandExecute for Etcd {
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
