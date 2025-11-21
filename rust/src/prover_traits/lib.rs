use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Proof {
    pub block: u64,
    pub proof: String,
    pub prover: String,
}

#[async_trait::async_trait]
pub trait Prover: Send + Sync {
    async fn prove(&self, block: u64) -> anyhow::Result<Proof>;
    fn name(&self) -> &'static str;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StepProof {
    pub data: Proof,
    pub task_id: String,
    pub group_id: u64,
}

#[async_trait::async_trait]
pub trait MultistepProver: Send + Sync {
    async fn prove(&self, block: u64, step_proofs: Vec<StepProof>) -> anyhow::Result<Proof>;
    async fn prove_step(
        &self,
        block: u64,
        step_id: String,
        payload: String,
    ) -> anyhow::Result<StepProof>;
    fn name(&self) -> &'static str;
}
