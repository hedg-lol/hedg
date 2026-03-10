import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export function getProvider(): anchor.AnchorProvider {
  return anchor.AnchorProvider.env();
}

export function findPDA(
  seeds: Buffer[],
  programId: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(seeds, programId);
}
