mod database;

use std::error::Error;
use clap::{Parser, Subcommand};
fn main() -> Result<(), Box<dyn Error>> {
    let cli = MainCLI::parse();
    let connection = database::init_db_conn().unwrap();

    match &cli.command {
        Commands::CreateWallet { name } => {
            database::create_private_key(&connection, name)?;
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(name = "LocalBlock")]
#[command(version = "0.0")]
#[command(about = "Simple CLI for interacting with localblock networks!", long_about = None)]
struct MainCLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateWallet {
        name: String,
    }
}