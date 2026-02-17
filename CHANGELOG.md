# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-27

### Added

- Escrow vault system with deployer collateral deposit
- Bonding curve with linear pricing model (base_price + slope * supply)
- Buyer record tracking for refund eligibility
- Pro-rata refund distribution from escrow collateral
- TypeScript SDK with full instruction coverage
- Comprehensive test suite with 9 test cases
- CI pipeline with Rust and TypeScript checks
