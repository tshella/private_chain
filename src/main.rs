mod transaction;
mod block;
mod storage;
mod consensus;

use clap::{Parser, Subcommand};
use transaction::Transaction;
use block::Block;
use storage::Storage;

#[derive(Parser)]
#[command(name = "SupplyChainChain")]
#[command(about = "Private blockchain for supply chain management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        sender: String,
        #[arg(short, long)]
        receiver: String,
        #[arg(short, long)]
        item: String,
    },
    Chain,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut storage = Storage::init("chain.db");

    match cli.command {
        Commands::Add { sender, receiver, item } => {
            let tx = Transaction::new(sender, receiver, item);
            let block = Block::new(vec![tx], storage.last_hash());
            storage.save_block(&block);
            println!("✅ Block added: {:?}", block);
        }
        Commands::Chain => {
            let chain = storage.load_chain();
            for block in chain {
                println!("{:#?}", block);
            }
        }
    }
}
