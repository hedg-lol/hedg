import { PublicKey } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey(
  "CcyByKGzRDK17icyNGAgdUN4q7WzbL1BPi4BNzqytyMP"
);

// Protocol parameters (matching on-chain constants)
export const MIN_COLLATERAL = 1_000_000_000n; // 1 SOL in lamports
export const SAFE_PERIOD = 86_400; // 24 hours in seconds
export const TRADE_FEE_BPS = 100; // 1%
export const PROTOCOL_FEE_BPS = 50; // 0.5%
export const DEPLOY_FEE = 20_000_000n; // 0.02 SOL in lamports
export const LAMPORTS_PER_SOL = 1_000_000_000n;

// PDA seeds
export const ESCROW_SEED = Buffer.from("escrow");
export const CURVE_SEED = Buffer.from("curve");
export const RECORD_SEED = Buffer.from("record");

// Devnet RPC endpoint
export const DEFAULT_RPC = "https://api.devnet.solana.com";
