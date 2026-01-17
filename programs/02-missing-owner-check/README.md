# 👤 Missing Owner Check Vulnerability

## Overview

**Risk Level:** 🔴 CRITICAL  
**Real-World Impact:** Multiple DeFi protocol exploits

The Missing Owner Check vulnerability occurs when a program fails to verify that an account is owned by the expected program. This allows attackers to substitute malicious accounts that your program will treat as legitimate.

## The Vulnerability

### ❌ Vulnerable Code Pattern

```rust
pub fn transfer_tokens_insecure(ctx: Context<TransferInsecure>, amount: u64) -> Result<()> {
    // VULNERABILITY: No verification that token_account is owned by Token Program
    // Attacker can pass a fake account that looks like a token account
    // but is actually controlled by their malicious program

    let token_account_data = ctx.accounts.token_account.try_borrow_data()?;

    // Program deserializes data assuming it's a valid token account
    // If attacker controls the account, they control the data
    let balance = u64::from_le_bytes(token_account_data[64..72].try_into().unwrap());

    require!(balance >= amount, ErrorCode::InsufficientBalance);

    // Transfer logic proceeds with potentially fake data
    Ok(())
}
```

### Attack Scenario

1. Attacker creates account owned by their malicious program
2. Attacker structures data to look like a token account with high balance
3. Attacker passes this fake account to your program
4. Your program treats it as legitimate because you didn't check the owner
5. Attacker bypasses balance checks and other validations

## The Fix

### ✅ Secure Implementation

```rust
use anchor_spl::token::{Token, TokenAccount};

pub fn transfer_tokens_secure(ctx: Context<TransferSecure>, amount: u64) -> Result<()> {
    // SOLUTION 1: Using Anchor's Account<'info, TokenAccount> type
    // Automatically verifies:
    // 1. Account is owned by Token Program
    // 2. Account data deserializes correctly as TokenAccount
    // 3. Account discriminator matches

    let token_account = &ctx.accounts.token_account;

    require!(
        token_account.amount >= amount,
        ErrorCode::InsufficientBalance
    );

    // Safe to proceed - we know this is a real token account
    Ok(())
}

#[derive(Accounts)]
pub struct TransferSecure<'info> {
    // ✅ Account<'info, TokenAccount> enforces:
    // - owner == Token Program ID
    // - data deserializes correctly
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
}
```

### Alternative: Manual Owner Check

```rust
pub fn transfer_manual_check(ctx: Context<TransferManual>, amount: u64) -> Result<()> {
    // Manual owner verification
    require!(
        ctx.accounts.token_account.owner == &spl_token::ID,
        ErrorCode::InvalidOwner
    );

    // Now safe to deserialize and use
    let token_account = TokenAccount::try_deserialize(
        &mut &ctx.accounts.token_account.data.borrow()[..]
    )?;

    require!(
        token_account.amount >= amount,
        ErrorCode::InsufficientBalance
    );

    Ok(())
}
```

## Key Differences

| Aspect             | Vulnerable        | Secure                        |
| ------------------ | ----------------- | ----------------------------- |
| Owner Verification | ❌ None           | ✅ Automatic via Account type |
| Data Validation    | ❌ Assumes valid  | ✅ Checked deserialization    |
| Exploitable?       | ✅ Yes            | ❌ No                         |
| Type Safety        | ❌ Manual parsing | ✅ Anchor types               |

## Common Mistakes

### ❌ Wrong: Using AccountInfo without checks

```rust
#[derive(Accounts)]
pub struct Vulnerable<'info> {
    // Dangerous! No owner check
    pub token_account: AccountInfo<'info>,
}
```

### ✅ Right: Using typed Account

```rust
#[derive(Accounts)]
pub struct Secure<'info> {
    // Safe! Owner automatically verified
    pub token_account: Account<'info, TokenAccount>,
}
```

### ✅ Also Right: Manual verification

```rust
pub fn with_manual_check(ctx: Context<Manual>) -> Result<()> {
    require!(
        ctx.accounts.token_account.owner == &expected_program_id,
        ErrorCode::InvalidOwner
    );
    // ... rest of logic
}
```

## Real-World Example

Many early Solana programs suffered from this:

```rust
// Vulnerable pattern from early programs
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let account = &accounts[0];

    // ❌ DANGER: No owner check!
    // Attacker can pass any account here
    let state = State::unpack(&account.data.borrow())?;

    // Program makes decisions based on potentially fake data
    if state.is_authorized {
        // Execute privileged operation
    }

    Ok(())
}
```

## Anchor Protection Mechanisms

Anchor provides automatic owner checks through account types:

```rust
// These all enforce owner checks automatically:

Account<'info, YourState>          // Checks owner == your program
Account<'info, TokenAccount>       // Checks owner == Token Program
Account<'info, Mint>               // Checks owner == Token Program
Program<'info, Token>              // Checks owner == BPF Loader

// This does NOT check owner (use with caution):
AccountInfo<'info>                 // No automatic checks
```

## Testing the Vulnerability

```typescript
it("Exploits missing owner check", async () => {
  // Create fake token account controlled by attacker
  const fakeAccount = Keypair.generate();

  // Attacker's malicious program owns this account
  await createAccountWithFakeData(
    fakeAccount,
    attackerProgram.programId, // Wrong owner!
    fakeTokenAccountData,
  );

  // Try to use fake account
  await program.methods
    .transferTokensInsecure(new BN(1000))
    .accounts({
      tokenAccount: fakeAccount.publicKey,
    })
    .rpc();

  // ❌ Vulnerable version: succeeds with fake account
  // ✅ Secure version: fails with "InvalidOwner" error
});
```

## Best Practices

### ✅ DO

1. **Use Anchor Account types** for automatic owner validation
2. **Use `#[account(owner = ExpectedProgram)]`** constraint when needed
3. **Verify owner manually** if using AccountInfo
4. **Check owner before deserializing** data

### ❌ DON'T

1. **Don't use AccountInfo** for structured data without checks
2. **Don't assume** an account is what it claims to be
3. **Don't skip** owner validation for "trusted" inputs
4. **Don't deserialize** before checking owner

## Related Constraints

```rust
// Enforce specific owner
#[account(owner = token_program)]
pub token_account: AccountInfo<'info>,

// Enforce owner is your program
#[account(owner = crate::ID)]
pub my_account: AccountInfo<'info>,

// Verify account is executable program
pub some_program: Program<'info, SomeProgram>,
```

## Impact Assessment

**If exploited:**

- Arbitrary data injection
- Bypass of business logic checks
- Token theft or minting
- State corruption
- Complete program compromise

**Cost to fix:**

- Low (change AccountInfo to Account)
- May affect account size calculations
- Easy to test and verify

## Related Vulnerabilities

- [Missing Signer Check](../01-missing-signer-check/) - Signature verification
- [Type Cosplay](../07-type-cosplay/) - Account type confusion
- [Arbitrary CPI](../05-arbitrary-cpi/) - Program validation

## References

- [Anchor Account Types](https://www.anchor-lang.com/docs/account-types)
- [Solana Program Ownership](https://solana.com/docs/core/accounts#owner)
- [SPL Token Program](https://spl.solana.com/token)
- [Sealevel Attacks](https://github.com/coral-xyz/sealevel-attacks)

---

**⚠️ Remember:** Always verify account ownership before trusting account data. Never use AccountInfo without explicit owner checks.
