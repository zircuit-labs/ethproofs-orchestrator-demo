use crate::prover::{Proof, Prover};

pub struct Sp1Prover;

#[async_trait::async_trait]
impl Prover for Sp1Prover {
    async fn prove(&self, block_num: u64) -> anyhow::Result<Proof> {
        Ok(Proof {
            block: block_num,
            proof: "0xff".to_owned(),
            prover: self.name().to_owned(),
        })
    }
    fn name(&self) -> &'static str {
        "sp1"
    }
}
