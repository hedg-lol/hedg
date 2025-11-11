# Migration Guide

## Account Versioning

When upgrading the program, account state changes require migration.

## Strategy

1. Add new fields to end of account structs
2. Increase `LEN` constant accordingly
3. Use `realloc` for existing accounts if needed
4. Never remove or reorder existing fields

## Compatibility

| Version | State Size | Notes |
|---------|-----------|-------|
| 0.1.0 | EscrowVault: 91 bytes | Initial release |
| 0.1.0 | BondingCurve: 106 bytes | Initial release |
| 0.1.0 | BuyerRecord: 98 bytes | Initial release |
