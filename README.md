# Spectral

Spectral is an experimental Rust project for decentralized service discovery in small, churn-prone clusters.

The goal is to run one identical agent on every node. Services register with their local agent, and agents gossip membership and service records to the rest of the cluster.

The overlay design is based on expander graphs and spectral graph theory, inspired by [DEX: self-healing expanders](https://link.springer.com/article/10.1007/s00446-015-0258-3).

## What It Does

- Tracks which nodes are alive.
- Stores service records locally.
- Shares updates through gossip.
- Avoids a central registry.
- Avoids consensus for the MVP.

Spectral is designed to favor availability and convergence over strict consistency. A lookup may briefly return stale or incomplete results while the cluster is changing, but the system should keep serving local reads and eventually settle.

## Core Ideas

- **Membership:** nodes detect joins, leaves, and failures.
- **Catalog:** service registrations are merged as eventually consistent records.
- **Overlay:** nodes maintain an expander-inspired routing structure for peer selection and resizing.
- **Actors:** each major part owns its own state and communicates by messages.

## Project Status

Spectral is under development.

## Running

```sh
cargo run
```

## Testing

```sh
cargo test
```

## Configuration

Default settings live in `config.toml`.
