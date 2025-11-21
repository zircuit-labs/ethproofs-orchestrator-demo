# Zircuit Orchestrator

An orchestration framework for composing zkProof pipelines using dispatchers and collectors. This demo uses NATS for message passing.

## Requirements

- Docker & Docker Compose
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (`cast` for block queries)

## Examples

All examples share the same core infrastructure ([core-docker-compose.yml](compose/core-docker-compose.yml)):
- **NATS**: Message broker with JetStream
- **block-collector**: Fetches Ethereum blocks and publishes to queue
- **proof-collector**: Matches blocks with their proofs

### 1. Multi-Prover Setup ([docker-compose-ethproofs.yml](compose/docker-compose-ethproofs.yml))
```bash
make start-services
```
Demonstrates **parallel proving** with different proof systems. Runs SP1, RISC0, and Zisk provers simultaneously. Each prover has a **unique consumer name**, meaning every block is processed by all three provers independently.

**Use case**: Generate multiple proofs for the same block using different proving systems.

### 2. Workload Splitting ([docker-compose-split-work.yml](compose/docker-compose-split-work.yml))
```bash
make start-split-work
```
Demonstrates **horizontal scaling** for a single prover type. Runs multiple mock prover instances with the **same consumer name**, distributing blocks across instances via NATS consumer groups—each block is processed by only one prover.

**Use case**: Scale proving capacity by adding more workers.

### 3. Single Mock Prover ([docker-compose-mock.yml](compose/docker-compose-mock.yml))
```bash
make start-mock-proving
```
Minimal setup with one mock prover for testing the pipeline without real proof generation.

**Use case**: Test pipeline flow and message routing.

### 4. AMD64 Architecture ([docker-compose-split-work-amd64.yml](compose/docker-compose-split-work-amd64.yml))
```bash
make start-split-work-amd64
```
Same as workload splitting but uses SP1 prover binaries compiled for `linux/amd64` (x86_64).

**Use case**: Run on AMD64 servers instead of ARM64.

### Stopping Services
```bash
make stop-services
```
Stops all services and removes containers and volumes.

## Custom Block Range

By default, pipelines start from the latest finalized Ethereum block. To specify a custom starting block:

```bash
export NEXT_BLOCK=23839700
docker compose -f compose/core-docker-compose.yml up -d
docker compose -f compose/docker-compose-ethproofs.yml up -d
```

## How It Works

The orchestrator models proving pipelines as a graph of **dispatchers** and **collectors**:

- **Dispatcher**: Consumes messages from a queue, invokes a binary with standardized I/O (`-i input.json -o output.json`), and publishes results to output queues
- **Collector**: Aggregates messages from multiple sources using configurable strategies (field matching, sequencing) and forwards to the next stage

This pattern enables arbitrary pipeline composition—dispatchers can invoke any binary (provers, preprocessors, aggregators), while collectors handle data dependencies and synchronization.

### Execution Model

Dispatchers use `JsonExecutor` to invoke binaries with JSON I/O. The executor interface is extensible—custom composers can read from arbitrary sources and route by message type.

### Dispatcher-Collector Pattern

```mermaid
graph LR
    C1[Collector] -->|aggregates| D1[Dispatcher]
    D1 -->|invokes| BIN[Binary]
    BIN -->|output| D1
    D1 -->|publishes| C2[Collector]
    C2 -->|aggregates| D2[Dispatcher]

    style C1 fill:#bbf,stroke:#333,stroke-width:2px
    style C2 fill:#bbf,stroke:#333,stroke-width:2px
    style D1 fill:#f9f,stroke:#333,stroke-width:2px
    style D2 fill:#f9f,stroke:#333,stroke-width:2px
    style BIN fill:#ddd,stroke:#333,stroke-width:1px
```

### Pipeline Architecture

```mermaid
graph TD
    ETH[Ethereum Network] -->|fetch blocks| BC[Block Collector]
    BC -->|publish| QUEUE[blocks queue]

    QUEUE -->|consume| P1[SP1 Prover Dispatcher]
    QUEUE -->|consume| P2[RISC0 Prover Dispatcher]
    QUEUE -->|consume| P3[Zisk Prover Dispatcher]

    P1 -->|invoke| B1[sp1_prover binary]
    P2 -->|invoke| B2[risc0_prover binary]
    P3 -->|invoke| B3[zisk_prover binary]

    B1 -.->|proof| P1
    B2 -.->|proof| P2
    B3 -.->|proof| P3

    P1 -->|publish| PQUEUE[block_proofs queue]
    P2 -->|publish| PQUEUE
    P3 -->|publish| PQUEUE

    BC -->|publish| BQUEUE[blocks metadata]
    BQUEUE -->|consume| PC[Proof Collector]
    PQUEUE -->|consume| PC

    PC -->|publish| OUT[completed_proofs queue]

    style BC fill:#bbf,stroke:#333,stroke-width:2px
    style PC fill:#bbf,stroke:#333,stroke-width:2px
    style P1 fill:#f9f,stroke:#333,stroke-width:2px
    style P2 fill:#f9f,stroke:#333,stroke-width:2px
    style P3 fill:#f9f,stroke:#333,stroke-width:2px
    style B1 fill:#ddd,stroke:#333,stroke-width:1px
    style B2 fill:#ddd,stroke:#333,stroke-width:1px
    style B3 fill:#ddd,stroke:#333,stroke-width:1px
    style QUEUE fill:#ffe,stroke:#333,stroke-width:1px
    style PQUEUE fill:#ffe,stroke:#333,stroke-width:1px
    style BQUEUE fill:#ffe,stroke:#333,stroke-width:1px
    style OUT fill:#dfd,stroke:#333,stroke-width:2px
```
