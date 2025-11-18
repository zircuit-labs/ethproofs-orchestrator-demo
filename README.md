# EthProofs: Modular Proving Orchestrator

This repository contains a template for implementing a Prover to use in the proving pipeline
as well as a docker compose setup with all services needed for an example proving pipeline.

## Proving pipeline

The example proving pripeline starts the following services:
1. **NATS**: The messaging system to distribute messages to the proving pipeline services.
2. **Block collector**: Retrieves the blocks from a network and dispatches a message
for every new block.
3. **Provers**: A proving service which invokes an arbitrary proving binary to generate proofs.
Proof messages are dispatched once binary generates the proof. We can run multiple proving dispatchers
if needed. They can either consume messages from the same queue (same consumer) or each prover may
consume its own sets of messages. By default, the same queue is shared across provers. To create separate
consumers for messages add a unique `CONSUMER_NAME` to the prover service in the
[docker-compose.yml](compose/docker-compose.yml).
4. **Proof Collector**: A service which matches the received proofs to the blocks dispatched by the 
Block Collector.
