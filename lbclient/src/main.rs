mod database;
mod pretty;
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

        Commands::ReadKey { blob: _ } => eprintln!("Feature not impld yet."),
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
    #[command(about = "deletes a currently held wallet signing key", long_about = None)]
    DeleteWallet { name: String },
    #[command(about = "lists current wallets held in the local database", long_about = None)]
    #[command(name = "wallets")]
    ReadWalletNames,
    #[command(about = "outputs a secret key with a specifed format", long_about = None)]
    ReadKey {
        #[arg(short, long)]
        blob: bool,
    },
}
