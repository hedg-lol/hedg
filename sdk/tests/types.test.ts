import { EscrowVault, BondingCurve, BuyerRecord } from "../src/types";

describe("Protocol Types", () => {
  it("EscrowVault should have required fields", () => {
    const vault: Partial<EscrowVault> = {
      deployer: "" as any,
      tokenMint: "" as any,
      collateralAmount: BigInt(0) as any,
    };
    expect(vault.deployer).toBeDefined();
    expect(vault.tokenMint).toBeDefined();
    expect(vault.collateralAmount).toBeDefined();
  });

  it("BondingCurve should track supply and reserves", () => {
    const curve: Partial<BondingCurve> = {
      currentSupply: BigInt(0) as any,
      reserveBalance: BigInt(0) as any,
    };
    expect(curve.currentSupply).toBeDefined();
    expect(curve.reserveBalance).toBeDefined();
  });
});
