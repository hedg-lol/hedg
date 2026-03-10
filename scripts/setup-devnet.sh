#!/bin/bash
set -e

echo "Configuring Solana CLI for devnet..."
solana config set --url https://api.devnet.solana.com
solana config get

echo "Requesting airdrop..."
solana airdrop 2

echo "Balance:"
solana balance

echo "Done. Ready for devnet development."
