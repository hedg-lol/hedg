# Deployment Checklist

## Prerequisites

- Solana CLI configured for target cluster
- Anchor CLI installed
- Deployer keypair funded

## Steps

1. Build the program: `anchor build`
2. Deploy to devnet: `anchor deploy --provider.cluster devnet`
3. Initialize protocol: call `init_protocol` with authority keypair
4. Verify on Solscan: check program account
5. Update IDL: copy `target/idl/fyrst.json` to `idl/`

## Post-Deployment

- Verify all PDAs are derivable
- Test escrow creation with minimum collateral
- Confirm fee collection in treasury
