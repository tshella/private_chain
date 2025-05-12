mod transaction;
mod block;
mod storage;
mod consensus;
mod blockchain;
mod api;
mod config;
mod access_control;
mod node;

use clap::{Parser, Subcommand};
use transaction::Transaction;
use std::sync::{Arc, Mutex};
use blockchain::Blockchain;

/// CLI definition
#[derive(Parser)]
#[command(name = "SupplyChainChain")]
#[command(about = "Private blockchain for supply chain management")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// CLI commands
#[derive(Subcommand)]
enum Commands {
    /// Add a new transaction to the chain
    Add {
        #[arg(short, long)]
        sender: String,
        #[arg(short, long)]
        receiver: String,
        #[arg(short, long)]
        item: String,
    },
    /// Print the full chain
    Chain,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut blockchain = Blockchain::new("chain.db");

    match cli.command {
        Some(Commands::Add { sender, receiver, item }) => {
            let tx = Transaction::new(sender, receiver, item);
            let block = blockchain.add_block(vec![tx]);
            println!("✅ Block added: {:?}", block);
        }
        Some(Commands::Chain) => {
            let chain = blockchain.chain();
            for block in chain {
                println!("{:#?}", block);
            }
        }
        None => {
            println!("📡 No CLI command provided — starting API service...");
            let shared = Arc::new(Mutex::new(blockchain));
            api::start_api(shared).await;
        }
    }
}
