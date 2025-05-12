# 🏗️ Supply Chain Blockchain (Private, Rust-Based)

A private, permissioned blockchain built in **Rust** for supply chain use cases such as audit trails, product tracking, and secure transaction recording between trusted parties. It features:

- 🚀 Fast, secure transaction recording
- 🌐 Axum-powered REST API
- 🐳 Dockerized multi-node setup
- 🔐 Role-based node authorization (PoA-ready architecture)
- 📦 Persistent ledger with `sled` key-value store

---

## 📚 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Architecture](#architecture)
- [How to Run](#how-to-run)
- [REST API Usage](#rest-api-usage)
- [Docker Compose Nodes](#docker-compose-nodes)
- [Folder Structure](#folder-structure)
- [Next Steps](#next-steps)

---

## ✅ Features

- 🔒 Private and permissioned (config-based node access)
- 💾 Append-only tamper-proof block storage
- ⚡ Fast JSON API (via Axum 0.7)
- 🛠️ CLI for transaction control
- 🐳 Docker Compose support for multi-node simulation
- 📁 Configurable nodes (`node1.toml`, `node2.toml`)

---

## 🧰 Tech Stack

| Component       | Technology         |
|----------------|--------------------|
| Language        | Rust               |
| Web Framework   | Axum 0.7           |
| Storage         | sled               |
| API             | REST (JSON)        |
| CLI Parser      | Clap               |
| Containerization| Docker + Compose   |
| Config Format   | TOML               |

---

## 🏗️ Architecture

- Each node runs an Axum API exposing `/chain` and `/add`
- Transactions are validated and appended to a local blockchain file
- Blocks are hashed using SHA256
- Role-based permissions supported via config
- Nodes can be extended to sync via libp2p or Raft/PBFT

---

## 🚀 How to Run Locally

### 1. Clone the repository

```bash
git clone https://github.com/your-org/private-chain.git
cd private-chain

2. Build and run

cargo build --release
cargo run -- chain

Or add a transaction:

cargo run -- add -s "FactoryA" -r "WarehouseB" -i "Cement"

🌐 REST API Usage
✅ Add a Transaction

curl -X POST http://localhost:3001/add \
  -H "Content-Type: application/json" \
  -d '{"sender": "NodeA", "receiver": "NodeB", "item": "Steel"}'

✅ Get the Chain

curl http://localhost:3001/chain

🐳 Docker Compose Nodes
1. Build and start both nodes

docker compose up --build

2. Access nodes
Node	Chain View	Add Transaction
1	localhost:3001	POST /add, GET /chain
2	localhost:3002	POST /add, GET /chain

Each container mounts a separate config (node1.toml, node2.toml) and persistent sled DB.
📁 Folder Structure

.
├── src/
│   ├── main.rs              # CLI & API entrypoint
│   ├── api.rs               # Axum REST interface
│   ├── blockchain.rs        # Core chain operations
│   ├── block.rs             # Block definition
│   ├── transaction.rs       # Transaction definition
│   ├── storage.rs           # Sled wrapper
│   ├── config.rs            # Node config loader
│   ├── access_control.rs    # Role-based access
│   ├── node.rs              # Node startup logic
│   └── consensus.rs         # (Extensible)
├── config/
│   ├── node1.toml
│   └── node2.toml
├── data/                    # Persistent chain.db volumes
├── Dockerfile               # Multi-stage build
├── docker-compose.yml       # 2-node setup
├── Cargo.toml
└── README.md

🔮 Next Steps

Add peer-to-peer block propagation (libp2p or gossip)

Add /health and /metrics endpoints

Add authentication middleware (JWT or API key)

Implement consensus logic (PoA / PBFT)

    UI dashboard (React or Tauri)

👨‍💻 Author

Built with ❤️ by Anthony Raphasha
Licensed under MIT