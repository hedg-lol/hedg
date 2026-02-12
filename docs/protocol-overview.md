# HEDG Protocol Overview

HEDG is a responsible token launchpad built on Solana.

## Core Components

1. **Escrow System** - Deployer collateral management
2. **Bonding Curve** - Automated market maker for token pricing
3. **Refund Engine** - Pro-rata refund distribution on rug detection
4. **Reputation** - Cross-wallet deployer reputation scoring

## Program Architecture

```
hedg/
  instructions/
    escrow.rs        - create_escrow, release_escrow
    bonding_curve.rs - init, buy_tokens, sell_tokens
    refund.rs        - record_buyer, process_refund
  state.rs           - Account definitions
  constants.rs       - Protocol parameters
  errors.rs          - Custom error types
  lib.rs             - Program entry point
```
