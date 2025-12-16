# HEDG Protocol Tests

Integration tests for the on-chain program.

## Running

```bash
anchor test
```

## Test Structure

| Suite | Tests | Description |
|-------|-------|-------------|
| Escrow | 4 | Vault lifecycle |
| Bonding Curve | 5 | AMM operations |
| Refund | 3 | Refund distribution |
| Edge Cases | 2 | Overflow and limits |

## Coverage

All instruction handlers are tested with both success and failure paths.
