use clap::Parser;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Proof {
    pub block: u64,
    pub proof: String,
    pub prover: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let block_number = if let Some(input_path) = args.input_path {
        let content = std::fs::read_to_string(input_path)?;

        serde_json::from_str(&content)?
    } else {
        // TODO fetch latest
        1
    };

    // Prover logic

    let proof = Proof {
        block: block_number,
        proof: "0xff".to_owned(),
        prover: "mock_prover".to_owned(),
    };

    let result = serde_json::to_vec(&proof)?;

    std::fs::write(args.output_path, result)?;

    Ok(())
}
