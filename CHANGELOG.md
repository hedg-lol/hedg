# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-27

### Added

- Escrow vault system with deployer collateral deposit and safe period enforcement
- Linear bonding curve with configurable base price and slope
- Buyer record tracking with weighted average price calculation
- Pro-rata refund distribution from escrow collateral on rug detection
- Trade fee system (1% per trade, 0.5% protocol fee)
- TypeScript SDK (`@hedg/sdk`) with full instruction coverage and account fetchers
- IDL JSON for client generation
- Comprehensive test suite covering escrow, bonding curve, and refund flows
- GitHub Actions CI with Rust clippy checks and SDK type verification
- Architecture documentation with mermaid diagrams

### Security

- All arithmetic operations use checked math to prevent overflow
- PDA derivation with deterministic seeds for account safety
- Escrow release gated by 24-hour safe period
- Deployer identity verified through signer constraints
