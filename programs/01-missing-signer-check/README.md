# 🔐 Missing Signer Check Vulnerability

## Overview

**Risk Level:** 🔴 CRITICAL  
**Real-World Impact:** Wormhole Bridge hack ($325M, 2022)

The Missing Signer Check vulnerability occurs when a program fails to verify that a transaction was signed by the expected authority. This allows unauthorized users to execute privileged operations.

## The Vulnerability

### ❌ Vulnerable Code Pattern

```rust
pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
    // VULNERABILITY: No check that authority actually signed the transaction!
    // Anyone can pass in any authority account and withdraw funds

    let vault = &mut ctx.accounts.vault;

    // This only checks that the authority field matches
    // It does NOT verify the authority actually signed the transaction
    if vault.authority != ctx.accounts.authority.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    // Transfer happens without signature verification
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}
```

### Attack Scenario

1. Attacker discovers vault with `authority = Alice`
2. Attacker creates transaction with `authority` account set to Alice's pubkey
3. Attacker signs with their own key (not Alice's)
4. Program checks `vault.authority == authority.key()` ✅ (passes)
5. Program never checks if Alice actually signed ❌
6. Funds are drained without Alice's permission

## The Fix

### ✅ Secure Implementation

```rust
pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
    // SOLUTION 1: Using Anchor constraint (recommended)
    // The #[account(signer)] constraint in the account validation ensures
    // that the authority account must have signed this transaction

    let vault = &mut ctx.accounts.vault;

    // This check is still good practice for explicitness
    require!(
        vault.authority == ctx.accounts.authority.key(),
        ErrorCode::Unauthorized
    );

    // Now we can safely transfer because we know:
    // 1. The authority field matches (explicit check)
    // 2. The authority actually signed (enforced by Anchor)
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}

// Account validation with signer constraint
#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    // CRITICAL: The 'signer' constraint ensures this account signed the transaction
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}
```

### Alternative: Manual Signer Check

```rust
pub fn withdraw_manual_check(ctx: Context<WithdrawManual>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Manual signer verification
    require!(
        ctx.accounts.authority.is_signer,
        ErrorCode::MissingSigner
    );

    require!(
        vault.authority == ctx.accounts.authority.key(),
        ErrorCode::Unauthorized
    );

    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}
```

## Key Differences

| Aspect              | Vulnerable          | Secure                  |
| ------------------- | ------------------- | ----------------------- |
| Signer Verification | ❌ None             | ✅ `#[account(signer)]` |
| Authority Check     | ✅ Compares pubkeys | ✅ Compares pubkeys     |
| Can be exploited?   | ✅ Yes              | ❌ No                   |
| Anchor Protection   | ❌ No constraints   | ✅ Proper constraints   |

## Real-World Example: Wormhole

In February 2022, the Wormhole bridge was exploited for $325M due to a missing signature verification:

```rust
// Simplified vulnerable pattern
fn verify_signatures(ctx: Context<VerifySignatures>) -> Result<()> {
    // Bug: Program loaded guardian set but didn't verify signatures
    let guardian_set = GuardianSet::load(&ctx.accounts.guardian_set)?;

    // Missing: Actual cryptographic signature verification
    // Attacker forged guardian signatures and minted tokens
}
```

## Testing the Vulnerability

```typescript
// This should FAIL but succeeds in vulnerable version
it("Exploits missing signer check", async () => {
  const attacker = Keypair.generate();

  // Attacker doesn't sign as the real authority
  // They just pass the authority's pubkey
  await program.methods
    .withdrawInsecure(new BN(1000))
    .accounts({
      vault: vaultPda,
      authority: realAuthority.publicKey, // Not signing!
    })
    .signers([attacker]) // Attacker signs instead
    .rpc();

  // ❌ In vulnerable version: succeeds
  // ✅ In secure version: fails with error
});
```

## Best Practices

### ✅ DO

1. **Always use `#[account(signer)]`** for accounts that must sign
2. **Use `Signer<'info>` type** instead of `AccountInfo<'info>` when signature is required
3. **Check `is_signer` manually** if not using Anchor constraints
4. **Test authorization failures** to ensure checks work

### ❌ DON'T

1. **Never assume** an account signed just because it's present
2. **Don't rely only on pubkey comparison** without signature verification
3. **Don't use `AccountInfo`** for authority accounts (use `Signer`)
4. **Don't skip testing** unauthorized access scenarios

## Anchor Constraint Options

```rust
// Option 1: Signer constraint (most common)
#[account(signer)]
pub authority: Signer<'info>,

// Option 2: Signer type (enforces at type level)
pub authority: Signer<'info>,

// Option 3: Manual check (less preferred)
#[account()]
pub authority: AccountInfo<'info>,
// Then check: require!(authority.is_signer, ...)
```

## Impact Assessment

**If exploited:**

- Complete loss of funds
- Unauthorized state mutations
- Loss of user trust
- Potential protocol shutdown

**Cost to fix:**

- Low (add one constraint)
- Zero performance impact
- Backward compatible with careful migration

## Related Vulnerabilities

- [Missing Owner Check](../02-missing-owner-check/) - Related account validation
- [Arbitrary CPI](../05-arbitrary-cpi/) - Program validation
- [Type Cosplay](../07-type-cosplay/) - Account type validation

## References

- [Wormhole Post-Mortem](https://medium.com/certora/the-wormhole-hack-what-happened-and-why-8bcaebc3b00b)
- [Anchor Signer Constraint](https://www.anchor-lang.com/docs/account-constraints#signer)
- [Solana Account Model](https://solana.com/docs/core/accounts)
- [Neodyme Blog: Signer Checks](https://blog.neodyme.io/)

---

**⚠️ Remember:** Always verify signatures for privileged operations. This is the most fundamental security check in Solana programs.
