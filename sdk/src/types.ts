import { PublicKey } from "@solana/web3.js";

export interface EscrowVault {
  deployer: PublicKey;
  tokenMint: PublicKey;
  collateralAmount: bigint;
  createdAt: bigint;
  released: boolean;
  rugged: boolean;
  bump: number;
}

export interface BondingCurve {
  tokenMint: PublicKey;
  currentSupply: bigint;
  basePrice: bigint;
  slope: bigint;
  reserveBalance: bigint;
  graduated: boolean;
  deployer: PublicKey;
  bump: number;
}

export interface BuyerRecord {
  buyer: PublicKey;
  tokenMint: PublicKey;
  totalBought: bigint;
  totalSolSpent: bigint;
  avgPrice: bigint;
  refundClaimed: boolean;
  firstBuyAt: bigint;
  bump: number;
}

export interface TokenLaunch {
  mint: PublicKey;
  deployer: PublicKey;
  name: string;
  symbol: string;
  escrow: EscrowVault;
  bondingCurve: BondingCurve;
  createdAt: Date;
}

export interface DeployerProfile {
  wallet: PublicKey;
  totalLaunches: number;
  rugCount: number;
  successfulReleases: number;
  reputationScore: number;
}

export interface TradeRecord {
  buyer: PublicKey;
  tokenMint: PublicKey;
  solAmount: bigint;
  tokenAmount: bigint;
  price: bigint;
  side: "buy" | "sell";
  timestamp: Date;
  txSignature: string;
}

export interface ProtocolConfig {
  minCollateral: bigint;
  safePeriod: number;
  tradeFee: number;
  protocolFee: number;
  deployFee: bigint;
}
