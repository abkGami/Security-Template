# 🎭 Account Data Matching (PDA Verification) Vulnerability

## Overview

**Risk Level:** 🔴 CRITICAL  
**Real-World Impact:** Cashio exploit ($52M, 2022)

The Account Data Matching vulnerability occurs when a program fails to properly validate that accounts are derived from the correct seeds and bumps, or when related accounts don't match expected relationships. This allows attackers to substitute fake accounts with manipulated data.

## The Vulnerability

### ❌ Vulnerable Code Pattern

```rust
pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>) -> Result<()> {
    // VULNERABILITY: No verification that user_stats PDA is derived correctly
    // Attacker can create fake user_stats account with inflated balance

    let user_stats = &ctx.accounts.user_stats;
    let user_balance = user_stats.balance;

    // Transfer based on unverified balance
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= user_balance;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += user_balance;

    Ok(())
}
```

### Attack Scenario (Cashio-style)

1. Program expects user_stats PDA derived from `[b"stats", user.key()]`
2. Attacker creates account with same structure but different seeds
3. Attacker sets `balance = 1_000_000` in fake account
4. Program doesn't verify PDA derivation
5. Attacker withdraws 1M tokens they don't own

## The Fix

### ✅ Secure Implementation

```rust
pub fn withdraw_secure(ctx: Context<WithdrawSecure>) -> Result<()> {
    // SOLUTION: Anchor verifies PDA derivation automatically with seeds constraint
    // The seeds and bump constraints ensure this account was derived correctly

    let user_stats = &ctx.accounts.user_stats;

    // Safe to use - we know this is the legitimate PDA
    let user_balance = user_stats.balance;

    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= user_balance;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += user_balance;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    pub user: Signer<'info>,

    /// ✅ SOLUTION: seeds + bump constraints verify PDA derivation
    /// Anchor checks that this account was derived from these exact seeds
    #[account(
        seeds = [b"stats", user.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,
}
```

## Real-World Example: Cashio Exploit

In March 2022, Cashio stablecoin was exploited for $52M:

```rust
// Simplified vulnerable pattern
#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    pub collateral_account: Account<'info, CollateralAccount>,
    // Missing: PDA verification for collateral_account
}

// Attacker passed fake collateral account with inflated value
// Minted stablecoins without real collateral
// Drained $52M
```

## Key Validation Methods

### Method 1: Anchor Seeds Constraint (Recommended)

```rust
#[account(
    seeds = [b"user", authority.key().as_ref()],
    bump = user.bump,
)]
pub user: Account<'info, User>,
```

### Method 2: Manual PDA Verification

```rust
let (expected_pda, bump) = Pubkey::find_program_address(
    &[b"user", authority.key().as_ref()],
    ctx.program_id
);

require!(
    ctx.accounts.user.key() == expected_pda,
    ErrorCode::InvalidPDA
);
```

### Method 3: has_one Constraint for Related Accounts

```rust
#[account(
    has_one = authority @ ErrorCode::InvalidAuthority,
    has_one = vault @ ErrorCode::InvalidVault
)]
pub user_stats: Account<'info, UserStats>,
```

## Common Patterns

### ✅ Correct PDA Usage

```rust
#[account(
    init,
    payer = user,
    space = 8 + UserStats::INIT_SPACE,
    seeds = [b"stats", user.key().as_ref()],
    bump
)]
pub user_stats: Account<'info, UserStats>,
```

### ❌ Missing Validation

```rust
// Dangerous! No seeds verification
pub user_stats: Account<'info, UserStats>,
```

## Testing the Vulnerability

```typescript
it("Exploits missing PDA verification", async () => {
  // Create fake user_stats with wrong derivation
  const fakeStats = Keypair.generate();

  await program.account.userStats.create(fakeStats, {
    balance: new BN(1000000), // Fake inflated balance
    user: attacker.publicKey,
  });

  // Try to withdraw using fake stats
  await program.methods
    .withdrawInsecure()
    .accounts({
      userStats: fakeStats.publicKey, // Wrong PDA!
    })
    .rpc();

  // ❌ Vulnerable: succeeds with fake balance
  // ✅ Secure: fails - seeds don't match
});
```

## Best Practices

### ✅ DO

1. **Always use seeds constraint** for PDAs
2. **Store and verify bump** seeds
3. **Use has_one constraint** for related accounts
4. **Verify account relationships** explicitly
5. **Test with fake accounts** to ensure validation works

### ❌ DON'T

1. **Don't skip PDA verification** even for "trusted" inputs
2. **Don't assume** accounts are correct just because they deserialize
3. **Don't forget** to validate all account relationships
4. **Don't use hardcoded** addresses without validation

## Related Vulnerabilities

- [Missing Owner Check](../02-missing-owner-check/) - Owner validation
- [Type Cosplay](../07-type-cosplay/) - Discriminator checks
- [Arbitrary CPI](../05-arbitrary-cpi/) - Program validation

## References

- [Cashio Post-Mortem](https://blog.coinbase.com/how-the-cashio-hack-happened-d87623e7d3f)
- [Anchor PDA Documentation](https://www.anchor-lang.com/docs/pdas)
- [Solana PDA Guide](https://solanacookbook.com/core-concepts/pdas.html)

---

**⚠️ Remember:** Always verify PDAs are derived with correct seeds. Account substitution is a critical attack vector.
