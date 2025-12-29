import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hedg } from "../target/types/hedg";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";

// Edge case coverage added
describe("hedg", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Hedg as Program<Hedg>;
  const deployer = provider.wallet;
  const tokenMint = Keypair.generate();

  describe("escrow", () => {
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

  describe("bonding curve", () => {
    const curveMint = Keypair.generate();

    it("initializes a bonding curve", async () => {
      const basePrice = new anchor.BN(1_000_000); // 0.001 SOL
      const slope = new anchor.BN(100); // 100 lamports per token

      const [bondingCurve] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("curve"), curveMint.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initBondingCurve(basePrice, slope)
        .accounts({
          deployer: deployer.publicKey,
          tokenMint: curveMint.publicKey,
          bondingCurve,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const curve = await program.account.bondingCurve.fetch(bondingCurve);
      expect(curve.basePrice.toNumber()).to.equal(1_000_000);
      expect(curve.slope.toNumber()).to.equal(100);
      expect(curve.currentSupply.toNumber()).to.equal(0);
      expect(curve.graduated).to.be.false;
    });

    it("allows buying tokens on the bonding curve", async () => {
      const solAmount = new anchor.BN(LAMPORTS_PER_SOL / 10); // 0.1 SOL

      const [bondingCurve] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("curve"), curveMint.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .buyTokens(solAmount)
        .accounts({
          buyer: deployer.publicKey,
          bondingCurve,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const curve = await program.account.bondingCurve.fetch(bondingCurve);
      expect(curve.currentSupply.toNumber()).to.be.greaterThan(0);
      expect(curve.reserveBalance.toNumber()).to.be.greaterThan(0);
    });
  });

  describe("refund", () => {
    const refundMint = Keypair.generate();

    it("records a buyer purchase", async () => {
      const amount = new anchor.BN(1_000_000);
      const price = new anchor.BN(1_000_000);

      const [buyerRecord] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("record"), deployer.publicKey.toBuffer(), refundMint.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .recordBuyer(amount, price)
        .accounts({
          buyer: deployer.publicKey,
          tokenMint: refundMint.publicKey,
          buyerRecord,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const record = await program.account.buyerRecord.fetch(buyerRecord);
      expect(record.totalBought.toNumber()).to.equal(1_000_000);
      expect(record.refundClaimed).to.be.false;
    });
  });
});
