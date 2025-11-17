mod mock_prover;
mod prover;
mod sp1_prover;

use clap::Parser;
use serde::{Deserialize, Serialize};
use crate::mock_prover::MockProver;
use crate::prover::Prover;
use crate::sp1_prover::Sp1Prover;

/// Example CLI with one optional and one required string argument
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Optional input file path
    #[arg(long = "input-path")]
    input_path: Option<String>,

    /// Required output file path
    #[arg(long = "output-path")]
    output_path: String,
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running prover.");
    let args = Args::parse();
    let block_number = if let Some(input_path) = args.input_path {
        let content = std::fs::read_to_string(input_path)?;

        serde_json::from_str(&content)?
    } else {
        println!("No input file provided. Defaulting to block number 1");
        1
    };

    let prover = MockProver {};
    let prover = Sp1Prover {};
    match prover.prove(block_number).await {
        Ok(proof) => {
            let result = serde_json::to_vec(&proof)?;
            std::fs::write(args.output_path, result)?;
            Ok(())
        }
        Err(error) => {
            eprintln!("Prover failure: {error:?}");
            Err(anyhow::anyhow!("Prover failure: {error:?}"))
        }
    }
}
