use crate::{
    cli::Cli,
    utils::{connect, PrintResult as _},
};
use clap::Subcommand;
use kuma_client::Config;

#[derive(Subcommand, Clone, Debug)]
#[command(arg_required_else_help = true)]
pub(crate) enum Command {
    /// Get current heartbeat status of all monitors
    List {},
}

pub(crate) async fn handle(command: &Option<Command>, config: &Config, cli: &Cli) {
    match command {
        Some(Command::List {}) => connect(config, cli)
            .await
            .get_heartbeats()
            .await
            .print_result(cli),

        None => {}
    }
}
