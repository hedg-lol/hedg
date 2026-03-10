# PDA Seed Reference

## Escrow Vault
- Seeds: `["escrow", deployer, token_mint]`
- Bump: stored in account

## Bonding Curve
- Seeds: `["curve", token_mint]`
- Bump: stored in account

## Buyer Record
- Seeds: `["record", buyer, token_mint]`
- Bump: stored in account

## Protocol Config
- Seeds: `["protocol"]`
- Bump: stored in account

All PDAs use canonical bumps derived by `find_program_address`.
