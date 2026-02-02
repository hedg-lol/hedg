# Fee Structure

## Trade Fees

| Component | Rate | Recipient |
|-----------|------|-----------|
| Trade Fee | 1.0% | Split below |
| - Deployer Share | 0.5% | Token deployer |
| - Protocol Share | 0.5% | Treasury PDA |

## Deployment Fee

A flat fee of 0.02 SOL is charged on escrow creation.

## Buyback Mechanism

30% of accumulated treasury fees are periodically used
to buy back $HEDG tokens on Jupiter, creating sustained
demand pressure.

## Fee Flow Diagram

```
Buyer/Seller
  |-- 1% trade fee -->
       |-- 0.5% --> Deployer wallet
       |-- 0.5% --> Treasury PDA
                      |-- 30% --> $HEDG buyback
                      |-- 70% --> Protocol operations
```
