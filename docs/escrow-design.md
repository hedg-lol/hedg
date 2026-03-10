# Escrow Design Notes

## Overview

The escrow vault holds deployer collateral for a configurable safe period (default: 24 hours).
During this period, the collateral is locked and cannot be withdrawn by the deployer.

## PDA Derivation

Seeds: `["escrow", deployer_pubkey, token_mint]`

This ensures each deployer can have exactly one escrow per token launch.

## Collateral Tiers

| Tier     | SOL Required | Trust Signal |
|----------|-------------|--------------|
| Bronze   | 0.01 - 0.99 | Minimal     |
| Silver   | 1.0 - 4.99  | Moderate    |
| Gold     | 5.0 - 9.99  | High        |
| Diamond  | 10.0+       | Maximum     |

## Lifecycle

1. Deployer calls `create_escrow` with collateral amount
2. SOL is transferred to PDA-controlled vault
3. After safe period: deployer can call `release_escrow`
4. If rugged: authority calls `mark_rugged`, then `process_refund` for each buyer
