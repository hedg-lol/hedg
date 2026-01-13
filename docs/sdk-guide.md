# SDK Usage Guide

## Installation

```bash
git clone https://github.com/hedg-lol/hedg.git
cd hedg/sdk
npm install
npm run build
```

## Quick Start

```typescript
import {{ HedgClient }} from "@hedg/sdk";

const client = new HedgClient(wallet, connection);

// Create escrow
const tx = await client.createEscrow(tokenMint, 1.0);

// Buy tokens
const buyTx = await client.buyTokens(tokenMint, 0.5);

// Sell tokens
const sellTx = await client.sellTokens(tokenMint, 1000);
```

## Error Handling

All methods throw `HedgError` on failure. Common errors:

| Code | Name                   | Description                    |
|------|------------------------|--------------------------------|
| 6000 | InsufficientCollateral | Collateral below minimum       |
| 6001 | SafePeriodNotExpired   | Cannot release during safe     |
| 6003 | SlippageExceeded       | Price moved beyond tolerance   |
| 6008 | MathOverflow           | Arithmetic overflow detected   |
