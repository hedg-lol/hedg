# ADR-001: Linear Bonding Curve Model

## Status

Accepted

## Context

We needed a pricing model for token launches that provides:
- Predictable price discovery
- Fair early-buyer advantage
- Sufficient liquidity depth

## Decision

Linear bonding curve: `price = base_price + slope * supply`

## Consequences

- Simple to implement and verify
- Predictable cost for large purchases via integral
- Price always increases with supply
- No complex oracle dependencies
