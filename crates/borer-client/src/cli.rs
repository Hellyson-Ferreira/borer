use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Login(LoginArgs),
    Up(UpArgs),
    Logout,
}

#[derive(Args, Debug)]
pub struct LoginArgs {
    pub token: String,
    pub remote_path: String,
}

#[derive(Args, Debug)]
pub struct UpArgs {
    pub local_port: u16,
    pub url: Option<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
