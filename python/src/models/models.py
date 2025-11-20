from pydantic import BaseModel

class Proof(BaseModel):
    block: int
    proof: str
    prover: str
