import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hedg } from "../target/types/hedg";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";

describe("hedg", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Hedg as Program<Hedg>;
  const deployer = provider.wallet;
  const tokenMint = Keypair.generate();

  it("creates an escrow vault with sufficient collateral", async () => {
    const collateral = new anchor.BN(LAMPORTS_PER_SOL); // 1 SOL

    const [escrowVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), deployer.publicKey.toBuffer(), tokenMint.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createEscrow(collateral)
      .accounts({
        deployer: deployer.publicKey,
        tokenMint: tokenMint.publicKey,
        escrowVault,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const escrow = await program.account.escrowVault.fetch(escrowVault);
    expect(escrow.deployer.toBase58()).to.equal(deployer.publicKey.toBase58());
    expect(escrow.collateralAmount.toNumber()).to.equal(LAMPORTS_PER_SOL);
    expect(escrow.released).to.be.false;
    expect(escrow.rugged).to.be.false;
  });

  it("rejects escrow creation with insufficient collateral", async () => {
    const newMint = Keypair.generate();
    const collateral = new anchor.BN(100_000); // 0.0001 SOL - below minimum

    const [escrowVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), deployer.publicKey.toBuffer(), newMint.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .createEscrow(collateral)
        .accounts({
          deployer: deployer.publicKey,
          tokenMint: newMint.publicKey,
          escrowVault,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      expect.fail("Should have thrown InsufficientCollateral error");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("InsufficientCollateral");
    }
  });

  it("prevents early escrow release before safe period", async () => {
    const [escrowVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), deployer.publicKey.toBuffer(), tokenMint.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .releaseEscrow()
        .accounts({
          deployer: deployer.publicKey,
          escrowVault,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      expect.fail("Should have thrown SafePeriodActive error");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("SafePeriodActive");
    }
  });
});
