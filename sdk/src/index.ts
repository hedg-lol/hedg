/**
 * HEDG Protocol SDK
 *
 * TypeScript client for the HEDG on-chain program.
 * Supports escrow management, bonding curve trading,
 * and refund claim operations.
 */

export { HedgClient } from "./client";
export type {
  EscrowVault,
  BondingCurve,
  BuyerRecord,
  TokenLaunch,
  DeployerProfile,
  TradeRecord,
  ProtocolConfig,
} from "./types";
export {
  MIN_COLLATERAL,
  SAFE_PERIOD,
  TRADE_FEE_BPS,
  PROTOCOL_FEE_BPS,
  DEPLOY_FEE,
  PROGRAM_ID,
  ESCROW_SEED,
  CURVE_SEED,
  RECORD_SEED,
} from "./constants";
