mod database;
mod pretty;
use std::io::Write;

use clap::{Parser, Subcommand};
use p256::ecdsa::VerifyingKey;
fn main() {
    let cli = MainCLI::parse();
    let database_path = cli.database.unwrap_or("client.db".to_owned());

    let connection = match database::init_db_conn(&database_path) {
        Ok(c) => c,
        Err(e) => panic!("Failed to load local database: {}", e),
    };

    match &cli.command {
        Commands::CreateWallet { name } => match database::create_private_key(&connection, name) {
            Ok(_) => println!("Wallet {} successfully created!", name),
            Err(e) => eprintln!("Error when creating wallet: {}", e),
        },

        Commands::DeleteWallet { name } => match database::delete_signing_key(&connection, name) {
            Ok(_) => println!("Wallet {} successfully deleted!", name),
            Err(e) => eprintln!("Error when deleting wallet: {}", e),
        },

        // TODO: Config.yaml to toggle pretty print, might break some systeams
        Commands::ReadWalletNames => match database::get_wallet_names(&connection) {
            Ok(names) => {
                if let Err(e) = pretty::show_wallet_names(&names) {
                    eprintln!("Some error occured when formatting: {}", e);
                }
            }
            Err(e) => eprintln!("Error when reading wallet names: {}", e),
        },

        Commands::ReadKey { name } => match database::get_key_blob(&connection, name) {
            Ok(blob) => {
                let mut stdout = std::io::stdout();
                stdout.write_all(&blob[..]);
                stdout.flush();
            }

            Err(e) => eprintln!("Some error occured when retrieving {}: {}", name, e),
        },

        Commands::SendCoin { recip } => eprintln!("This does nothing currently..."),
        
        Commands::PublicKey { name } => match database::load_verifying_key(&connection, name) {
            Ok(VerifyingKey) => {

            },
            Err(e) => eprintln!("Error when loading verifying key..."),
        }
    }
}

#[derive(Parser)]
#[command(name = "LocalBlock")]
#[command(version = "0.0")]
#[command(about = "Simple CLI for interacting with localblock networks!", long_about = None)]
struct MainCLI {
    #[arg(long, short)]
    database: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "creates a new wallet signing key", long_about = None)]
    CreateWallet { name: String },
    #[command(about = "deletes a currently held wallet signing key", long_about = None)]
    DeleteWallet { name: String },
    #[command(about = "lists current wallets held in the local database", long_about = None)]
    #[command(name = "wallets")]
    ReadWalletNames,
    #[command(about = "outputs a secret key as bytes for storage elsewhere", long_about = None)]
    ReadKey { name: String },
    #[command(about = "transfers one coin to a recipient at the specified auth node", long_about = None)]
    SendCoin { recip: String },
    #[command(about = "outputs a wallets public key")]
    PublicKey { name: String },
}
