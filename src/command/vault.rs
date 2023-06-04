/// Connects to vault
use crate::CommandExecute;
use std::path::PathBuf;

#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Vault {
    #[arg(short, long, required = true, default_value_t = true)]
    dryrun: bool,

    /// The backend polling interval in seconds. (600)
    #[arg(short, long, default_value_t = 600)]
    interval: u16,

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


impl CommandExecute for Vault {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        // todo!("this is the entry into the app where we");
        if self.dryrun {
            println!("dry running here from vault");
        } else {
            println!("running here from vault");
        }

        Ok(())
    }
}
