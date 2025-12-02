# HEDG Protocol

The first responsible token launchpad on Solana.

## Overview

HEDG Protocol introduces deployer accountability to token launches through mandatory collateral escrow, automated buyer protection, and cross-wallet reputation tracking.

## Architecture

The protocol consists of three layers:

**On-Chain (Solana Program)**
- Escrow Vault: holds deployer collateral during the safe period
- Bonding Curve: manages token pricing via a linear price model
- Buyer Records: tracks purchases for refund eligibility

**Off-Chain Services**
- REST API and WebSocket for real-time data
- Reputation Engine with cross-wallet analysis
- PostgreSQL for historical data

**Client Layer**
- Web Dashboard for browsing launches and checking deployer reputation
- TypeScript SDK for programmatic access

## Status

Under active development. See the [CHANGELOG](CHANGELOG.md) for the latest updates.

## License

[MIT](LICENSE)
