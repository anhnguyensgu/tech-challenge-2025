# 🦀 Rust Backend System with Smart Contracts

This project includes a full-stack system (excluding frontend) built with Rust and Solidity. It is composed of:

- 🧩 **API Service** — A RESTful backend (Axum-based) that interacts with Redis and PostgreSQL.
- 🔍 **Indexer Service** — A background worker for processing and indexing events.
- 🐘 **PostgreSQL** — Relational database used for persistent storage.
- 🚀 **Redis** — In-memory store for caching and task queues.
- 🧱 **Migration Container** — A standalone Docker container that runs `sqlx` migrations.
- 🔐 **Smart Contracts** — Solidity contracts managed via Hardhat.

---

## 🛠 Prerequisites

- **Rust**: Recommended version is `1.85.0` or higher
  ```bash
  rustup install stable
  rustup default stable
  ```

- [Docker & Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js & NPM](https://nodejs.org/) (for Hardhat)
- `sqlx-cli` (if running migrations manually):
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

---

## 🚀 Running the Project Locally

### 1. Clone the repo

```bash
git clone https://github.com/anhnguyensgu/tech-challenge-2025
cd tech-challenge-2025
```

### 2. Start services with Docker Compose

```bash
docker-compose up --build
```

This launches:
- `api` (Rust HTTP server on `localhost:8080`)
- `indexer` (Rust background job processor)
- `postgres` (PostgreSQL database on port `5432`)
- `redis` (Redis cache on port `6379`)
- `migrate` (Runs `sqlx migrate run` once at startup)

---

## 🧪 Running Locally (Without Docker)

### 1. Setup environment variables

Create a `.env.local` file:

```dotenv
PORT=3000
DATABASE_URL=postgres://postgres:postgres@localhost/eth_indexer
REDIS_URL=redis://127.0.0.1:6379/0
CONTRACT_ADDRESS=0x21b06BEc125803635f0a9221655E731f6b0DB478
START_BLOCK=
```

### 2. Run database migrations

```bash
just migrate
```

### 3. Run services

```bash
# API
cargo run --bin api

# Indexer
cargo run --bin indexer
```

---

## 📦 Project Structure

```
.
├── backend/                 # Rust services (API, indexer)
│   ├── migrations/          # SQLx migrations
│   ├── src/bin/api.rs       # Main API binary
│   ├── src/bin/indexer.rs   # Indexer binary
│   ├── Dockerfile
│   └── Dockerfile.migrate
├── smart_contract/          # Hardhat Solidity project
│   ├── contracts/           # Solidity contracts
│   ├── scripts/             # Deployment scripts
│   ├── test/                # Smart contract tests
│   └── hardhat.config.ts
├── docker-compose.yml
```

---

## 🔐 Smart Contract Development

### 1. Install dependencies
```bash
cd smart_contract
npm install
```
Create a `.env.local` file:

```dotenv
RPC_URL=https://ethereum-sepolia-rpc.publicnode.com
PRIVATE_KEY=
CONTRACT_ADDRESS=0x21b06BEc125803635f0a9221655E731f6b0DB478
```

### 2. Compile contracts
```bash
npx hardhat compile
```

### 3. Run tests
```bash
npx hardhat test
```

### 4. Deploy (example)
```bash
npx hardhat run scripts/deploy.ts --network localhost
```
### 5. Mint (example)
```bash
npx hardhat run scripts/mint.ts --network sepolia
```

> You can customize deployment scripts and networks via `hardhat.config.ts`

---

## 📌 Assumptions & Design Decisions

- The system is modular with separate binaries for API and indexer.
- SQLx is used with compile-time verification and a separate migration container.
- Redis and Postgres are used for performance and reliability.
- Smart contracts are managed via Hardhat and written in Solidity.
- I’ve put the frontend tasks on hold for now since I’m currently more focused on backend work.

---

## ⚠️ Known Issues & Limitations

- API has no auth or rate limiting enabled by default.
- Indexer is stateless and logs to stdout only.
- Smart contract deployment is manual (can be automated in CI/CD).

---

