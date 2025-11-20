use crate::prover::{Proof, Prover};

pub struct ZiskProver;

#[async_trait::async_trait]
impl Prover for ZiskProver {
    async fn prove(&self, block_num: u64) -> anyhow::Result<Proof> {
        let proving_time = rand::random_range(1..5);
        tokio::time::sleep(tokio::time::Duration::from_secs(proving_time)).await;
        Ok(Proof {
            block: block_num,
            proof: "0xff".to_owned(),
            prover: self.name().to_owned(),
        })
    }
    fn name(&self) -> &'static str {
        "zisk"
    }
}
