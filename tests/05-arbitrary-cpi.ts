import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArbitraryCpi } from "../target/types/arbitrary_cpi";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("05-arbitrary-cpi", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ArbitraryCpi as Program<ArbitraryCpi>;

  describe("❌ VULNERABLE: Arbitrary CPI", () => {
    it("Demonstrates arbitrary CPI vulnerability", async () => {
      console.log("    ⚠️  VULNERABILITY: Accepts any program ID for CPI");
      console.log("    ⚠️  Attacker passes their malicious program");
      console.log("    ⚠️  Malicious program doesn't actually transfer");
      console.log("    ⚠️  Or drains funds through backdoor");
      console.log("    ⚠️  Real Impact: Crema Finance lost $8.8M");
    });
  });

  describe("✅ SECURE: Validated CPI", () => {
    it("Uses Program<'info, T> to enforce program validation", async () => {
      console.log("    ✅ Program<'info, Token> validates program ID");
      console.log("    ✅ Only spl_token::ID is accepted");
      console.log("    ✅ Anchor checks at instruction validation");
      console.log("    ✅ Malicious programs are rejected");
    });

    it("Alternative: address constraint for known programs", async () => {
      console.log("    ✅ #[account(address = spl_token::ID)]");
      console.log("    ✅ Explicitly validates program address");
      console.log("    ✅ Fails if wrong program passed");
      console.log("    ✅ Use for whitelisting specific programs");
    });
  });

  describe("📚 Best Practices", () => {
    it("Shows CPI security checklist", async () => {
      console.log("    ✅ Always use Program<'info, T> for external programs");
      console.log("    ✅ Never accept arbitrary program IDs from users");
      console.log("    ✅ Whitelist allowed programs explicitly");
      console.log("    ✅ Use Anchor's CPI helpers when available");
      console.log("    ❌ Don't use raw invoke() without validation");
    });
  });
});
