# Development Environment Setup

## Required Tools

| Tool | Version | Purpose |
|------|---------|----------|
| Rust | 1.75+ | Smart contract development |
| Solana CLI | 1.17+ | Blockchain interaction |
| Anchor | 0.29+ | Solana framework |
| Node.js | 18+ | SDK and testing |
| npm | 9+ | Package management |

## Quick Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli

# Clone and build
git clone https://github.com/hedg-lol/hedg.git
cd hedg/contracts
anchor build
```
