use crate::cli::Cli;
use crate::cli::Commands::{Login, Logout, Up};
use clap::Parser;

mod agent;
mod cli;
mod proxy;
mod tunnel;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Up(args) => {
            agent::run_agent(args.local_port).await;
        }
        Login(args) => {
            proxy::run_login(args.token, args.remote_path).await;
        }
        Logout => {
            println!("Logging out...");
        }
    }
}
