# HEDG SDK

TypeScript SDK for interacting with the HEDG on-chain program.

## Build

```bash
npm install
npm run build
```

## Usage

```typescript
import { HedgClient } from "@hedg/sdk";

const client = new HedgClient(wallet, connection);
await client.createEscrow(tokenMint, 1.0);
```

## API

| Method | Description |
|--------|-------------|
| `createEscrow(mint, sol)` | Lock deployer collateral |
| `releaseEscrow(mint)` | Reclaim after safe period |
| `buyTokens(mint, sol)` | Buy on bonding curve |
| `sellTokens(mint, amount)` | Sell on bonding curve |
| `claimRefund(mint)` | Claim pro-rata refund |
