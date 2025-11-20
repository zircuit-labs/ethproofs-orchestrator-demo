# EthProofs: Modular Proving Orchestrator

This repository contains a template for implementing a Prover to use in the proving pipeline
as well as a docker compose setup with all services needed for an example proving pipeline.

## Proving pipeline

The example proving proving starts the following services:

1. **NATS**: The messaging system to distribute messages to the proving pipeline services.
2. **Block collector**: Retrieves the blocks from a network and dispatches a message
   for every new block.
3. **Provers**: A proving service which invokes an arbitrary proving binary to generate proofs.
   Proof messages are dispatched once binary generates the proof. We can run multiple proving dispatchers
   if needed. They can either consume messages from the same queue (same consumer) or each prover may
   consume its own sets of messages. By default, the same queue is shared across provers.
4. **Proof Collector**: A service which matches the received proofs to the blocks dispatched by the
   Block Collector.

## Running the pipeline

The Makefile for running different configurations and the docker compose files are found in `compose/`.

The Makefile provides different targets for running the core orchestrator services with various prover configurations.
All targets start the same core services (NATS, block collector, and proof collector) but differ in their prover setup.

### Available Targets

#### Multi prover setup 
```shell
make start-services
```

Runs the full multi-prover setup from `docker-compose-ethproofs.yml`. Each prover (SP1, RISC0, and Zisk) has a unique
`CONSUMER_NAME`, meaning all provers will independently process every block.

___

#### Split work setup
```shell
make start-split-work
```

Runs multiple mock provers from `docker-compose-split-work.yml` that share the same `CONSUMER_NAME`. This distributes
the workload across prover instances - each block is processed by only one prover.

___

#### Single mock prover
```shell
make start-mock-proving
```

Runs a single mock prover from `docker-compose-mock.yml` for basic testing without real proof generation.

___

#### Split work setup on amd64 architecture
```shell
make start-split-work-amd64
````

Same as `start-split-work` but runs SP1 provers on `linux/amd64` platform with appropriate binaries for x86_64
architecture.

___

#### Stopping the services
```shell
make stop-services
````

Stops all running services and removes containers, and volumes.

### Manual Block Configuration

All start targets automatically fetch the latest finalized Ethereum block using `cast`. To specify a custom starting
block:

```shell
export NEXT_BLOCK=23839700
docker compose -f core-docker-compose.yml up -d
docker compose -f docker-compose-ethproofs.yml up -d
```
