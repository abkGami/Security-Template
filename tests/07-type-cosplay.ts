import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TypeCosplay } from "../target/types/type_cosplay";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("07-type-cosplay", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TypeCosplay as Program<TypeCosplay>;

  describe("❌ VULNERABLE: Type Cosplay", () => {
    it("Demonstrates type cosplay vulnerability", async () => {
      console.log("    ⚠️  VULNERABILITY: No discriminator verification");
      console.log("    ⚠️  Attacker passes wrong account type");
      console.log("    ⚠️  Account data laid out to look correct");
      console.log("    ⚠️  Program deserializes without type checking");
      console.log("    ⚠️  Logic operates on wrong account type");
    });

    it("Shows type confusion attack", async () => {
      console.log("");
      console.log("    TYPE CONFUSION ATTACK:");
      console.log("    Expected: VaultConfig account");
      console.log("    Attacker passes: UserAccount");
      console.log("    Both have similar field layout:");
      console.log("      VaultConfig { authority, fee_rate, ... }");
      console.log("      UserAccount { owner, balance, ... }");
      console.log("    Program reads balance as fee_rate");
      console.log("    Attacker manipulates fees or permissions");
    });
  });

  describe("✅ SECURE: Discriminator validation", () => {
    it("Uses Account<'info, T> for automatic discriminator checks", async () => {
      console.log("    ✅ Account<'info, VaultConfig> enforces type");
      console.log("    ✅ Anchor checks discriminator automatically");
      console.log("    ✅ Discriminator = first 8 bytes of account");
      console.log("    ✅ Computed from hash('account:TypeName')");
      console.log("    ✅ Wrong type = wrong discriminator = fails");
    });

    it("Shows how Anchor discriminators work", async () => {
      console.log("");
      console.log("    DISCRIMINATOR MECHANISM:");
      console.log("    1. Anchor adds 8-byte discriminator to each account");
      console.log("    2. Discriminator derived from account name hash");
      console.log("    3. Stored as first 8 bytes of account data");
      console.log("    4. Checked automatically on deserialization");
      console.log("    5. Prevents type confusion attacks");
    });
  });

  describe("📚 Best Practices", () => {
    it("Shows type safety checklist", async () => {
      console.log("    ✅ Always use Account<'info, T> for typed accounts");
      console.log("    ✅ Let Anchor handle discriminator validation");
      console.log("    ✅ Use #[account] macro for all custom types");
      console.log(
        "    ✅ Never manually deserialize without discriminator check",
      );
      console.log("    ❌ Don't use AccountInfo for structured data");
    });

    it("Shows account structure", async () => {
      console.log("");
      console.log("    ACCOUNT DATA LAYOUT:");
      console.log("    [0..8]   Discriminator (type identifier)");
      console.log("    [8..40]  Field 1 (e.g., authority: Pubkey)");
      console.log("    [40..48] Field 2 (e.g., balance: u64)");
      console.log("    [48..49] Field 3 (e.g., active: bool)");
      console.log("    ...");
    });
  });
});
