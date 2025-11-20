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
