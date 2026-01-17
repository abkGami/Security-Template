import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MissingSignerCheck } from "../target/types/missing_signer_check";
import { expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";

describe("01-missing-signer-check", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .MissingSignerCheck as Program<MissingSignerCheck>;

  let vaultKeypair: Keypair;
  let authority: Keypair;
  let attacker: Keypair;

  beforeEach(async () => {
    vaultKeypair = Keypair.generate();
    authority = Keypair.generate();
    attacker = Keypair.generate();

    // Airdrop SOL to test accounts
    const airdropSig = await provider.connection.requestAirdrop(
      authority.publicKey,
      2 * LAMPORTS_PER_SOL,
    );
    await provider.connection.confirmTransaction(airdropSig);

    const attackerAirdrop = await provider.connection.requestAirdrop(
      attacker.publicKey,
      1 * LAMPORTS_PER_SOL,
    );
    await provider.connection.confirmTransaction(attackerAirdrop);
  });

  describe("❌ VULNERABLE: withdraw_insecure", () => {
    it("Allows unauthorized withdrawal - EXPLOITABLE!", async () => {
      // Initialize vault with authority
      await program.methods
        .initializeVaultSecure(new anchor.BN(1000000))
        .accounts({
          initializer: authority.publicKey,
          vault: vaultKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority, vaultKeypair])
        .rpc();

      // Fund the vault
      const fundTx = await provider.connection.requestAirdrop(
        vaultKeypair.publicKey,
        1 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(fundTx);

      const vaultBalanceBefore = await provider.connection.getBalance(
        vaultKeypair.publicKey,
      );
      const attackerBalanceBefore = await provider.connection.getBalance(
        authority.publicKey, // Attacker sends funds to real authority address
      );

      console.log(
        `    💰 Vault balance before: ${vaultBalanceBefore / LAMPORTS_PER_SOL} SOL`,
      );
      console.log(
        `    💰 Authority balance before: ${attackerBalanceBefore / LAMPORTS_PER_SOL} SOL`,
      );

      try {
        // 🚨 EXPLOIT: Attacker signs the transaction, not the authority
        // But passes authority's pubkey as the authority account
        await program.methods
          .withdrawInsecure(new anchor.BN(0.5 * LAMPORTS_PER_SOL))
          .accounts({
            vault: vaultKeypair.publicKey,
            authority: authority.publicKey, // Real authority's key
          })
          .signers([attacker]) // ⚠️ But attacker signs!
          .rpc();

        // ❌ VULNERABLE: This succeeds when it shouldn't!
        const vaultBalanceAfter = await provider.connection.getBalance(
          vaultKeypair.publicKey,
        );
        const authorityBalanceAfter = await provider.connection.getBalance(
          authority.publicKey,
        );

        console.log(
          `    💰 Vault balance after: ${vaultBalanceAfter / LAMPORTS_PER_SOL} SOL`,
        );
        console.log(
          `    💰 Authority balance after: ${authorityBalanceAfter / LAMPORTS_PER_SOL} SOL`,
        );
        console.log(
          `    🚨 EXPLOIT SUCCESSFUL: Attacker withdrew without authority's signature!`,
        );

        // Verify funds were stolen
        expect(vaultBalanceAfter).to.be.lessThan(vaultBalanceBefore);
        expect(authorityBalanceAfter).to.be.greaterThan(attackerBalanceBefore);
      } catch (error) {
        // This should NOT throw in vulnerable version
        console.log(
          `    ✅ Protection worked (shouldn't happen in vulnerable version)`,
        );
        throw error;
      }
    });
  });

  describe("✅ SECURE: withdraw_secure", () => {
    it("Prevents unauthorized withdrawal", async () => {
      // Initialize vault
      await program.methods
        .initializeVaultSecure(new anchor.BN(1000000))
        .accounts({
          initializer: authority.publicKey,
          vault: vaultKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority, vaultKeypair])
        .rpc();

      // Fund vault
      const fundTx = await provider.connection.requestAirdrop(
        vaultKeypair.publicKey,
        1 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(fundTx);

      try {
        // Try to withdraw with attacker signature
        await program.methods
          .withdrawSecure(new anchor.BN(0.5 * LAMPORTS_PER_SOL))
          .accounts({
            vault: vaultKeypair.publicKey,
            authority: authority.publicKey,
          })
          .signers([attacker])
          .rpc();

        // Should not reach here
        throw new Error("Expected transaction to fail");
      } catch (error: any) {
        // ✅ SECURE: Transaction fails due to missing signer
        console.log(`    ✅ Attack prevented: ${error.message}`);
        expect(error.message).to.include("unknown signer");
      }
    });

    it("Allows authorized withdrawal when properly signed", async () => {
      // Initialize vault
      await program.methods
        .initializeVaultSecure(new anchor.BN(1000000))
        .accounts({
          initializer: authority.publicKey,
          vault: vaultKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority, vaultKeypair])
        .rpc();

      // Fund vault
      const fundTx = await provider.connection.requestAirdrop(
        vaultKeypair.publicKey,
        1 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(fundTx);

      const vaultBalanceBefore = await provider.connection.getBalance(
        vaultKeypair.publicKey,
      );

      // ✅ Legitimate withdrawal with proper signature
      await program.methods
        .withdrawSecure(new anchor.BN(0.5 * LAMPORTS_PER_SOL))
        .accounts({
          vault: vaultKeypair.publicKey,
          authority: authority.publicKey,
        })
        .signers([authority]) // ✅ Proper signature
        .rpc();

      const vaultBalanceAfter = await provider.connection.getBalance(
        vaultKeypair.publicKey,
      );

      console.log(`    ✅ Legitimate withdrawal successful`);
      expect(vaultBalanceAfter).to.be.lessThan(vaultBalanceBefore);
    });
  });

  describe("✅ SECURE: withdraw_manual_check", () => {
    it("Prevents unauthorized withdrawal with manual check", async () => {
      await program.methods
        .initializeVaultSecure(new anchor.BN(1000000))
        .accounts({
          initializer: authority.publicKey,
          vault: vaultKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority, vaultKeypair])
        .rpc();

      const fundTx = await provider.connection.requestAirdrop(
        vaultKeypair.publicKey,
        1 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(fundTx);

      try {
        await program.methods
          .withdrawManualCheck(new anchor.BN(0.5 * LAMPORTS_PER_SOL))
          .accounts({
            vault: vaultKeypair.publicKey,
            authority: authority.publicKey,
          })
          .signers([attacker])
          .rpc();

        throw new Error("Expected transaction to fail");
      } catch (error: any) {
        console.log(`    ✅ Manual check prevented attack`);
        expect(error.message).to.include("unknown signer");
      }
    });
  });

  describe("🎯 Edge Cases", () => {
    it("Handles vault authority updates securely", async () => {
      await program.methods
        .initializeVaultSecure(new anchor.BN(1000000))
        .accounts({
          initializer: authority.publicKey,
          vault: vaultKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority, vaultKeypair])
        .rpc();

      const newAuthority = Keypair.generate();

      // Update authority
      await program.methods
        .updateAuthority(newAuthority.publicKey)
        .accounts({
          vault: vaultKeypair.publicKey,
          authority: authority.publicKey,
        })
        .signers([authority])
        .rpc();

      // Verify old authority can't withdraw
      try {
        await program.methods
          .withdrawSecure(new anchor.BN(100))
          .accounts({
            vault: vaultKeypair.publicKey,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();

        throw new Error("Old authority should not be able to withdraw");
      } catch (error: any) {
        console.log(`    ✅ Old authority correctly denied`);
        expect(error.message).to.include("Error");
      }
    });
  });
});
