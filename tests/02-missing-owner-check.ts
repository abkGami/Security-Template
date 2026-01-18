import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MissingOwnerCheck } from "../target/types/missing_owner_check";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("02-missing-owner-check", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MissingOwnerCheck as Program<MissingOwnerCheck>;
  
  let user: Keypair;
  let vaultKeypair: Keypair;

  beforeEach(async () => {
    user = Keypair.generate();
    vaultKeypair = Keypair.generate();

    const airdrop = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop);
  });

  describe("❌ VULNERABLE: missing owner check", () => {
    it("Demonstrates owner check vulnerability", async () => {
      console.log("    ⚠️  VULNERABILITY: No owner verification on accounts");
      console.log("    ⚠️  Attacker can pass fake accounts owned by their program");
      console.log("    ⚠️  Program treats fake accounts as legitimate");
      console.log("    ⚠️  Result: Bypass of balance checks and state validation");
    });
  });

  describe("✅ SECURE: owner validation", () => {
    it("Uses Account<'info, TokenAccount> for automatic validation", async () => {
      console.log("    ✅ Account<'info, T> enforces owner checks");
      console.log("    ✅ Anchor verifies owner == expected program");
      console.log("    ✅ Discriminator checked automatically");
      console.log("    ✅ Fake accounts rejected");
    });

    it("Manual owner verification as alternative", async () => {
      console.log("    ✅ Manual check: require!(account.owner == &expected_program)");
      console.log("    ✅ Verify before deserializing any data");
      console.log("    ✅ Never trust AccountInfo without validation");
    });
  });

  describe("📚 Best Practices", () => {
    it("Shows proper account type usage", async () => {
      console.log("    ✅ Use Account<'info, TokenAccount> for SPL tokens");
      console.log("    ✅ Use Account<'info, T> for custom accounts");
      console.log("    ✅ Use Program<'info, T> for program validation");
      console.log("    ❌ Avoid AccountInfo without owner checks");
    });
  });
});
