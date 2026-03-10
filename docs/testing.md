# Testing Guide

## Prerequisites

- Solana CLI (v1.17+)
- Anchor Framework (v0.29+)
- Node.js (v18+)

## Running Tests

```bash
# Start local validator
solana-test-validator

# Run Anchor tests
anchor test

# Run SDK type checks
cd sdk && npm run typecheck
```

## Test Coverage

| Module        | Tests | Description                      |
|---------------|-------|----------------------------------|
| Escrow        | 4     | Create, release, safe period     |
| Bonding Curve | 5     | Init, buy, sell, fees, overflow  |
| Refund        | 3     | Record, process, pro-rata math   |
