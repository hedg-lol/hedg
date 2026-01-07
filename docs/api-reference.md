# HEDG Program API Reference

## Instructions

### `create_escrow`

Initialize an escrow vault for a token launch.

**Arguments:**
- `collateral_amount: u64` - SOL amount in lamports

**Accounts:**
- `deployer` (signer, writable)
- `token_mint` (read-only)
- `escrow_vault` (PDA, writable)
- `system_program`

### `init_bonding_curve`

Create a bonding curve for token trading.

**Arguments:**
- `base_price: u64` - Starting price in lamports
- `slope: u64` - Price increase per token

**Accounts:**
- `deployer` (signer, writable)
- `token_mint` (read-only)
- `bonding_curve` (PDA, writable)
- `system_program`

### `buy_tokens`

Purchase tokens on the bonding curve.

**Arguments:**
- `sol_amount: u64` - SOL to spend in lamports

### `sell_tokens`

Sell tokens back to the bonding curve.

**Arguments:**
- `token_amount: u64` - Tokens to sell

### `process_refund`

Distribute pro-rata refund to a buyer.

**Arguments:** None (computed from buyer record)
