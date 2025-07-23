mod database;
use clap::{Parser, Subcommand};
fn main() {
    let cli = MainCLI::parse();
    let connection = match database::init_db_conn() {
        Ok(c) => c,
        Err(e) => panic!("Failed to load local database: {}", e),
    };

    match &cli.command {
        Commands::CreateWallet { name } => match database::create_private_key(&connection, name) {
            Ok(_) => println!("Wallet {} successfully created!", name),
            Err(e) => eprintln!("Error when creating wallet: {}", e),
        },
    }
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
    #[command(about = "creates a new wallet signing key", long_about = None)]
    CreateWallet { name: String },
}
