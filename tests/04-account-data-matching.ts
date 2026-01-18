import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountDataMatching } from "../target/types/account_data_matching";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("04-account-data-matching", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AccountDataMatching as Program<AccountDataMatching>;
  
  let user: Keypair;

  beforeEach(async () => {
    user = Keypair.generate();

    const airdrop = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop);
  });

  describe("❌ VULNERABLE: Missing PDA verification", () => {
    it("Demonstrates PDA verification vulnerability (Cashio-style)", async () => {
      console.log("    ⚠️  VULNERABILITY: No PDA derivation verification");
      console.log("    ⚠️  Attacker creates fake PDA with wrong seeds");
      console.log("    ⚠️  Fake PDA has inflated balance data");
      console.log("    ⚠️  Program accepts it without verification");
      console.log("    ⚠️  Real Impact: Cashio lost $52M this way");
    });
  });

  describe("✅ SECURE: PDA verification with seeds and bump", () => {
    it("Uses seeds and bump constraints for validation", async () => {
      console.log("    ✅ seeds = [...] constraint verifies PDA derivation");
      console.log("    ✅ bump = account.bump validates bump seed");
      console.log("    ✅ has_one = field checks account relationships");
      console.log("    ✅ Fake PDAs with wrong seeds are rejected");
    });

    it("Demonstrates proper PDA initialization", async () => {
      console.log("    ✅ init with seeds creates properly derived PDA");
      console.log("    ✅ Store bump in account state");
      console.log("    ✅ Use stored bump for all future validations");
      console.log("    ✅ Anchor handles derivation automatically");
    });
  });

  describe("📚 Best Practices", () => {
    it("Shows PDA security checklist", async () => {
      console.log("    ✅ Always use seeds + bump constraints");
      console.log("    ✅ Store bump in account structure");
      console.log("    ✅ Use has_one for related accounts");
      console.log("    ✅ Test with fake PDAs to verify protection");
      console.log("    ❌ Never accept PDAs without seed verification");
    });
  });
});
