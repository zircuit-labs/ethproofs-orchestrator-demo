use prover_traits::{MultistepProver, Proof, StepProof};

pub struct MockMultistepProver;

#[async_trait::async_trait]
impl MultistepProver for MockMultistepProver {
    async fn prove_step(
        &self,
        block_num: u64,
        step_id: String,
        _payload: String,
    ) -> anyhow::Result<StepProof> {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let proof = Proof {
            block: block_num,
            proof: "0xff".to_owned(),
            prover: self.name().to_owned(),
        };

        Ok(StepProof {
            group_id: block_num,
            task_id: step_id,
            data: proof,
        })
    }

    async fn prove(&self, block_num: u64, step_proofs: Vec<StepProof>) -> anyhow::Result<Proof> {
        tracing::info!(
            "Aggregating {} proofs into single block proof",
            step_proofs.len()
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

        Ok(Proof {
            block: block_num,
            proof: "0xff".to_owned(),
            prover: self.name().to_owned(),
        })
    }

    fn name(&self) -> &'static str {
        "mock-multistep-prover"
    }
}
