mod prover;

use crate::prover::MockMultistepProver;
use clap::Parser;
use prover_traits::{MultistepProver, StepProof};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct Step {
    block: u64,
    step_id: String,
    payload: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum TaskPayload {
    Block(u64),
    Step(Step),
    StepProofs(Vec<StepProof>),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let content = std::fs::read_to_string(args.input_path)?;
    let payload = serde_json::from_str(&content)?;
    let prover = MockMultistepProver {};

    let result: Vec<u8> = match payload {
        TaskPayload::Block(block_number) => {
            tracing::info!("Splitting block {block_number} into subtasks");
            let mut steps = vec![];
            for i in 0..10 {
                steps.push(Step {
                    block: block_number,
                    step_id: format!("example_step_{i}"),
                    payload: format!("tx_{i}_data"),
                });
            }
            serde_json::to_vec(&steps)?
        }
        TaskPayload::Step(step) => {
            match prover.prove_step(step.block, step.step_id, step.payload).await {
                Ok(step_proof) => serde_json::to_vec(&step_proof)?,
                Err(error) => {
                    eprintln!("Prover failure: {error:?}");
                    return Err(anyhow::anyhow!("Prover failure: {error:?}"));
                }
            }
        }
        TaskPayload::StepProofs(step_proofs) => {
            let block_number = step_proofs[0].group_id;
            match prover.prove(block_number, step_proofs).await {
                Ok(proof) => serde_json::to_vec(&proof)?,
                Err(error) => {
                    eprintln!("Prover failure: {error:?}");
                    return Err(anyhow::anyhow!("Prover failure: {error:?}"));
                }
            }
        }
    };
    std::fs::write(args.output_path, result)?;
    Ok(())
}
