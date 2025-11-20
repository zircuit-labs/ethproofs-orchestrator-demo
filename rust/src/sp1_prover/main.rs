mod prover;

use crate::prover::Sp1Prover;
use clap::Parser;
use prover_traits::Prover;

/// Example CLI with two required string arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Required input file path
    #[arg(long = "input-path")]
    input_path: String,

    /// Required output file path
    #[arg(long = "output-path")]
    output_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Running prover.");
    let args = Args::parse();
    let content = std::fs::read_to_string(args.input_path)?;
    let block_number = serde_json::from_str(&content)?;

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
