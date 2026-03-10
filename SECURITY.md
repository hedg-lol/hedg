# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability, please report it responsibly.

**Do not open a public issue.** Instead, send details to the project maintainers
through a private channel.

## Scope

This policy covers the on-chain program code in `contracts/programs/fyrst/`.

## Known Limitations

- This software is unaudited
- The bonding curve uses integer arithmetic which may have rounding effects
- PDA derivation relies on canonical bumps

## Safe Practices

- All arithmetic uses `checked_*` operations to prevent overflow
- PDA seeds are deterministic and verifiable
- Escrow release requires safe period expiration
- Refund amounts are capped by escrow balance
