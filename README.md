# Online Voting (Rust POC)

> ⚠️ **Work in Progress**
> This project is a learning exercise and proof-of-concept. It is not intended for production use.

## Overview

This repository is an experimental implementation of a secure online voting system, written in Rust. The goal is to explore how modern Rust patterns and libraries can be used to build a system that could, in theory, meet the stringent requirements of French electoral regulations.

**Key objectives:**
- Learn Rust by building a real-world, security-focused application
- Experiment with cryptographic primitives and secure protocols
- Model a system architecture that could scale and be auditable
- Document challenges and solutions for educational purposes

## Project Structure

```
online-voting/
├── src/                  # Core shared Rust code
├── voting-core/          # Core domain logic, models, and cryptography
│   ├── src/
│   └── tests/
├── voting-server/        # Server-side logic (API, orchestration)
│   └── src/
├── voting-client-cli/    # CLI client for interacting with the system
│   └── src/
├── Cargo.toml            # Rust workspace manifest
├── Cargo.lock            # Dependency lockfile
└── digest.txt            # (WIP) Documentation, notes, or digests
```

## Features

### Implemented
- Modular Rust workspace structure
- Core voting models (Election, Voter, Ballot)
- Cryptographic primitives (ElGamal encryption, commitments, zero-knowledge proof skeletons)
- Secure vote casting and tallying logic (in-memory, not networked)
- CLI client for simulating voting and tallying
- Unit tests for core election and crypto flows
- Prevention of double voting
- Ballot receipts and inclusion proofs (basic)

### In Progress / Planned
- [ ] Real server API for vote orchestration (currently only CLI and in-memory)
- [ ] Persistent storage (database integration)
- [ ] End-to-end cryptographic proofs (full ZK, verifiable tally)
- [ ] Compliance with all French legal requirements (currently only modeled, not enforced)
- [ ] User authentication and authorization
- [ ] Secure network communication (TLS, etc.)
- [ ] Audit logging and transparency features
- [ ] More robust error handling and input validation
- [ ] Scalability and performance testing
- [ ] Documentation of security and regulatory considerations

## Why Rust?

- **Memory safety**: Rust's ownership model helps prevent common bugs and vulnerabilities.
- **Performance**: Suitable for high-throughput, low-latency systems.
- **Modern ecosystem**: Great libraries for cryptography, web, and testing.

## Security & Compliance

This project aims to:
- Use strong cryptographic primitives (see `voting-core/src/crypto/`)
- Model auditable and transparent voting flows
- Explore regulatory requirements for online voting in France

**Disclaimer:**
This code is for educational purposes only. It is not audited, not production-ready, and should not be used for real elections.

## Getting Started

1. **Install Rust** (see [rustup.rs](https://rustup.rs/))
2. Clone the repository:
   ```sh
   git clone https://github.com/your-username/online-voting.git
   cd online-voting
   ```
3. Build the workspace:
   ```sh
   cargo build
   ```
4. Run tests:
   ```sh
   cargo test
   ```

## Contributing

Contributions, feedback, and suggestions are welcome!
Please open issues or pull requests to discuss improvements or questions.

## License

[MIT](LICENSE) (or specify your preferred license)

---

**Note:**
This project is not affiliated with any official French electoral body.
It is a personal learning project and a technical exploration.
