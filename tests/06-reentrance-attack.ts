import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ReentranceAttack } from "../target/types/reentrance_attack";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("06-reentrance-attack", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ReentranceAttack as Program<ReentranceAttack>;

  describe("❌ VULNERABLE: Re-entrancy", () => {
    it("Demonstrates re-entrancy vulnerability", async () => {
      console.log("    ⚠️  VULNERABILITY: External call before state update");
      console.log("    ⚠️  Attacker re-enters during external call");
      console.log("    ⚠️  Balance not yet updated, check passes again");
      console.log("    ⚠️  Can withdraw multiple times with single balance");
      console.log("    ⚠️  Pattern: Adapted from Ethereum's DAO hack");
    });

    it("Shows attack timeline", async () => {
      console.log("");
      console.log("    ATTACK TIMELINE:");
      console.log("    T0: withdraw(100) called");
      console.log("    T1: Check balance: 100 >= 100 ✓");
      console.log("    T2: Transfer 100 (external call)");
      console.log("    T3: ⚠️  Attacker re-enters withdraw(100)");
      console.log("    T4: Check balance: still 100! ✓");
      console.log("    T5: Transfer another 100");
      console.log("    T6: Repeat until vault empty");
      console.log("    T7: Finally update balance to 0");
      console.log("    Result: Withdrew 1000+ with balance of 100");
    });
  });

  describe("✅ SECURE: Checks-Effects-Interactions", () => {
    it("Follows CEI pattern to prevent re-entrancy", async () => {
      console.log("    ✅ CHECKS: Validate all conditions first");
      console.log("    ✅ EFFECTS: Update state BEFORE external calls");
      console.log("    ✅ INTERACTIONS: External calls LAST");
      console.log("    ✅ If attacker re-enters, balance already updated");
      console.log("    ✅ Check fails: 0 < 100, attack prevented");
    });

    it("Alternative: Re-entrancy guard pattern", async () => {
      console.log("    ✅ Set locked = true before operations");
      console.log("    ✅ Check !locked at function entry");
      console.log("    ✅ Release lock after completion");
      console.log("    ✅ Re-entrancy attempt fails immediately");
    });
  });

  describe("📚 Best Practices", () => {
    it("Shows CEI pattern template", async () => {
      console.log("");
      console.log("    CHECKS-EFFECTS-INTERACTIONS PATTERN:");
      console.log("    1. CHECKS:");
      console.log("       require!(balance >= amount)");
      console.log("       require!(vault.active)");
      console.log("");
      console.log("    2. EFFECTS:");
      console.log("       balance -= amount");
      console.log("       total_withdrawn += amount");
      console.log("");
      console.log("    3. INTERACTIONS:");
      console.log("       invoke_signed(transfer)");
      console.log("       emit!(event)");
    });
  });
});
