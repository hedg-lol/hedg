# Bonding Curve Model

## Mathematical Foundation

The HEDG bonding curve uses a linear pricing model:

```
price(supply) = base_price + slope * supply
```

## Buy Cost (Integral)

For buying `T` tokens starting at supply `S`:

```
cost = integral(S, S+T) of (base_price + slope * x) dx
     = base_price * T + slope * (2*S*T + T^2) / 2
```

## Sell Return (Integral)

For selling `T` tokens starting at supply `S`:

```
return = integral(S-T, S) of (base_price + slope * x) dx
       = base_price * T + slope * (2*S*T - T^2) / 2
```

## Graduation

When `reserve_balance >= graduation_threshold` (default: 85 SOL),
the token graduates to a DEX. The bonding curve is frozen and
remaining liquidity is migrated.

## Fee Structure

| Fee          | Basis Points | Recipient        |
|--------------|-------------|------------------|
| Trade Fee    | 100 (1%)    | 50% deployer, 50% treasury |
