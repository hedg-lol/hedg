# Refund System

## Pro-Rata Distribution

When a token is marked as rugged, buyers receive refunds proportional to their
contribution relative to the total collected SOL.

```
refund_amount = (buyer_sol_spent / total_sol_collected) * escrow_collateral
```

## Process

1. Protocol authority calls `mark_rugged` on the escrow
2. Escrow state flips to `rugged = true`
3. For each buyer record: authority calls `process_refund`
4. Buyer receives pro-rata share from the escrow vault
5. Buyer record is marked as `refund_claimed = true`

## Guard Rails

- Only protocol authority can trigger refunds
- Each buyer can only claim once
- Refund amount is capped by escrow balance
- All math uses u128 intermediates to prevent overflow
