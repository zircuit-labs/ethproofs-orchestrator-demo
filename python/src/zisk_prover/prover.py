import asyncio
from models.models import Proof


class ZiskProver:
    async def prove(self, block: int) -> Proof:
        await asyncio.sleep(2)
        return Proof(block=block, proof="0xff", prover=self.name())

    def name(self):
        return "zisk"
