import argparse
import json
import asyncio
from zisk.prover import ZiskProver

async def run():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input-path", required=True)
    parser.add_argument("--output-path", required=True)
    args = parser.parse_args()

    with open(args.input_path) as f:
        block_number = json.load(f)

    prover = ZiskProver()
    proof = await prover.prove(block_number)

    with open(args.output_path, "w") as f:
        f.write(proof.model_dump_json())

def main():
    asyncio.run(run())
