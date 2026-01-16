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

## Bonding Curve Model

HEDG uses a linear bonding curve for token pricing:

```
price(s) = base_price + slope * current_supply
```

Where:
- `base_price` is the initial token price in lamports
- `slope` determines how quickly the price increases per token sold
- `current_supply` is the total number of tokens currently in circulation

The curve ensures transparent, deterministic pricing where each successive token costs slightly more than the last.

## Reputation Scoring

Deployer reputation is calculated using on-chain history:

- **Launch Count**: total tokens deployed
- **Rug Count**: number of launches flagged as rugs
- **Success Rate**: percentage of launches where escrow was cleanly released
- **Collateral History**: average and total collateral deposited

The score is rule-based (not ML) and updated in real-time as on-chain events occur.

## Status

Under active development. See the [CHANGELOG](CHANGELOG.md) for the latest updates.

## License

[MIT](LICENSE)

## Architecture

See `docs/` for detailed design documents covering escrow, bonding curves, and refund mechanics.
