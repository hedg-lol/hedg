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
}
