/**
 * HEDG Protocol client.
 * High-level interface for interacting with the on-chain program.
 */

import { Program, AnchorProvider, BN, Idl } from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  SystemProgram,
  Keypair,
  TransactionSignature,
} from "@solana/web3.js";
import { PROGRAM_ID, ESCROW_SEED, CURVE_SEED, RECORD_SEED, DEFAULT_RPC } from "./constants";

/**
 * HedgClient provides methods to interact with the HEDG Protocol on-chain program.
 */
export class HedgClient {
  private program: Program;
  private connection: Connection;
  private provider: AnchorProvider;

  constructor(provider: AnchorProvider, idl: Idl) {
    this.provider = provider;
    this.connection = provider.connection;
    this.program = new Program(idl, provider);
  }

  // ── PDA Derivation ──────────────────────────────────────────────────────

  /**
   * Derive the escrow vault PDA for a deployer and token mint.
   */
  findEscrowPda(deployer: PublicKey, tokenMint: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [ESCROW_SEED, deployer.toBuffer(), tokenMint.toBuffer()],
      PROGRAM_ID
    );
  }

  /**
   * Derive the bonding curve PDA for a token mint.
   */
  findCurvePda(tokenMint: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [CURVE_SEED, tokenMint.toBuffer()],
      PROGRAM_ID
    );
  }

  /**
   * Derive the buyer record PDA for a buyer and token mint.
   */
  findRecordPda(buyer: PublicKey, tokenMint: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [RECORD_SEED, buyer.toBuffer(), tokenMint.toBuffer()],
      PROGRAM_ID
    );
  }

  // ── Escrow Instructions ─────────────────────────────────────────────────

  /**
   * Create an escrow vault with collateral deposit.
   */
  async createEscrow(
    tokenMint: PublicKey,
    collateralAmount: BN
  ): Promise<TransactionSignature> {
    const deployer = this.provider.wallet.publicKey;
    const [escrowVault] = this.findEscrowPda(deployer, tokenMint);

    return this.program.methods
      .createEscrow(collateralAmount)
      .accounts({
        deployer,
        tokenMint,
        escrowVault,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  /**
   * Release escrow after the safe period has elapsed.
   */
  async releaseEscrow(tokenMint: PublicKey): Promise<TransactionSignature> {
    const deployer = this.provider.wallet.publicKey;
    const [escrowVault] = this.findEscrowPda(deployer, tokenMint);

    return this.program.methods
      .releaseEscrow()
      .accounts({
        deployer,
        escrowVault,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  // ── Bonding Curve Instructions ──────────────────────────────────────────

  /**
   * Initialize a bonding curve for a token.
   */
  async initBondingCurve(
    tokenMint: PublicKey,
    basePrice: BN,
    slope: BN
  ): Promise<TransactionSignature> {
    const deployer = this.provider.wallet.publicKey;
    const [bondingCurve] = this.findCurvePda(tokenMint);

    return this.program.methods
      .initBondingCurve(basePrice, slope)
      .accounts({
        deployer,
        tokenMint,
        bondingCurve,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  /**
   * Buy tokens on the bonding curve.
   * The sol_amount includes the trade fee (1%), which is deducted on-chain.
   */
  async buyTokens(
    tokenMint: PublicKey,
    solAmount: BN
  ): Promise<TransactionSignature> {
    const buyer = this.provider.wallet.publicKey;
    const [bondingCurve] = this.findCurvePda(tokenMint);

    return this.program.methods
      .buyTokens(solAmount)
      .accounts({
        buyer,
        bondingCurve,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  /**
   * Sell tokens on the bonding curve.
   */
  async sellTokens(
    tokenMint: PublicKey,
    tokenAmount: BN
  ): Promise<TransactionSignature> {
    const seller = this.provider.wallet.publicKey;
    const [bondingCurve] = this.findCurvePda(tokenMint);

    return this.program.methods
      .sellTokens(tokenAmount)
      .accounts({
        seller,
        bondingCurve,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  // ── Refund Instructions ─────────────────────────────────────────────────

  /**
   * Record a buyer's purchase for refund eligibility.
   */
  async recordBuyer(
    tokenMint: PublicKey,
    amount: BN,
    price: BN
  ): Promise<TransactionSignature> {
    const buyer = this.provider.wallet.publicKey;
    const [buyerRecord] = this.findRecordPda(buyer, tokenMint);

    return this.program.methods
      .recordBuyer(amount, price)
      .accounts({
        buyer,
        tokenMint,
        buyerRecord,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  /**
   * Process a refund for a buyer from a rugged escrow.
   */
  async processRefund(
    buyer: PublicKey,
    deployer: PublicKey,
    tokenMint: PublicKey
  ): Promise<TransactionSignature> {
    const authority = this.provider.wallet.publicKey;
    const [escrowVault] = this.findEscrowPda(deployer, tokenMint);
    const [buyerRecord] = this.findRecordPda(buyer, tokenMint);

    return this.program.methods
      .processRefund()
      .accounts({
        authority,
        buyer,
        escrowVault,
        buyerRecord,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  // ── Account Fetchers ────────────────────────────────────────────────────

  /**
   * Fetch an escrow vault account.
   */
  async fetchEscrow(deployer: PublicKey, tokenMint: PublicKey) {
    const [escrowVault] = this.findEscrowPda(deployer, tokenMint);
    return (this.program.account as any).escrowVault.fetch(escrowVault);
  }

  /**
   * Fetch a bonding curve account.
   */
  async fetchBondingCurve(tokenMint: PublicKey) {
    const [bondingCurve] = this.findCurvePda(tokenMint);
    return (this.program.account as any).bondingCurve.fetch(bondingCurve);
  }

  /**
   * Fetch a buyer record account.
   */
  async fetchBuyerRecord(buyer: PublicKey, tokenMint: PublicKey) {
    const [buyerRecord] = this.findRecordPda(buyer, tokenMint);
    return (this.program.account as any).buyerRecord.fetch(buyerRecord);
  }
}
