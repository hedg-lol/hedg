# ADR-002: Pro-Rata Refund Distribution

## Status

Accepted

## Context

When a rug pull is detected, buyers need compensation from the deployer collateral.

## Decision

Refund proportional to SOL spent:
`refund = (buyer_spent / total_collected) * escrow_balance`

## Consequences

- Fair distribution regardless of buy timing
- Uses u128 intermediate math to prevent overflow
- Each buyer can only claim once
- Total refunds capped by escrow balance
