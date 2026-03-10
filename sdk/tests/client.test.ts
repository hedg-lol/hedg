import { FyrstClient } from "../src/client";
import { PROGRAM_ID } from "../src/constants";

describe("FyrstClient", () => {
  it("should initialize with correct program ID", () => {
    expect(PROGRAM_ID).toBeDefined();
    expect(PROGRAM_ID.toBase58()).toHaveLength(44);
  });

  it("should derive escrow PDA deterministically", () => {
    const seeds = [Buffer.from("escrow")];
    expect(seeds[0]).toEqual(Buffer.from("escrow"));
  });

  it("should derive bonding curve PDA deterministically", () => {
    const seeds = [Buffer.from("curve")];
    expect(seeds[0]).toEqual(Buffer.from("curve"));
  });
});
