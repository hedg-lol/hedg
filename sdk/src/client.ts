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
}
