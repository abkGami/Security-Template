# 🔢 Arithmetic Overflow/Underflow Vulnerability

## Overview

**Risk Level:** 🔴 HIGH  
**Real-World Impact:** Numerous token programs, $50M+ in losses

Arithmetic overflow and underflow vulnerabilities occur when mathematical operations exceed the boundaries of their data types without proper checks. In Rust, this can lead to wrapping behavior in release mode, causing unexpected results like unlimited token minting or balance manipulation.

## The Vulnerability

### ❌ Vulnerable Code Pattern

```rust
pub fn deposit_insecure(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // VULNERABILITY: Using unchecked arithmetic
    // In release mode, this wraps on overflow
    vault.total_deposited = vault.total_deposited + amount;  // ⚠️ Can overflow!

    // If total_deposited = u64::MAX and amount = 1
    // Result wraps to 0 instead of erroring

    Ok(())
}
```

### Attack Scenario

1. Vault has `total_deposited = u64::MAX - 100`
2. Attacker deposits 200 tokens
3. Instead of getting overflow error, value wraps to 99
4. Accounting is now broken - attacker "deposited" 200 but vault shows only 99 increase
5. Attacker can exploit this to drain funds or mint unlimited tokens

## The Fix

### ✅ Secure Implementation

```rust
pub fn deposit_secure(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // SOLUTION 1: Using checked_add (recommended)
    vault.total_deposited = vault.total_deposited
        .checked_add(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // If overflow would occur, checked_add returns None
    // We convert to error and transaction fails safely

    Ok(())
}
```

## Key Differences

| Operation      | Vulnerable | Secure                           |
| -------------- | ---------- | -------------------------------- |
| Addition       | `a + b`    | `a.checked_add(b).ok_or(Error)?` |
| Subtraction    | `a - b`    | `a.checked_sub(b).ok_or(Error)?` |
| Multiplication | `a * b`    | `a.checked_mul(b).ok_or(Error)?` |
| Division       | `a / b`    | `a.checked_div(b).ok_or(Error)?` |

## Real-World Examples

### Token Minting Overflow

```rust
// Vulnerable token mint
pub fn mint_tokens_vulnerable(amount: u64) {
    token.supply = token.supply + amount;  // Can wrap!
    user.balance = user.balance + amount;  // Can wrap!
}

// Attacker mints u64::MAX tokens, supply wraps to near 0
// Now attacker has tokens without increasing supply correctly
```

### Reward Calculation Underflow

```rust
// Vulnerable reward distribution
pub fn claim_rewards_vulnerable(user: &mut User) {
    let rewards = calculate_rewards();
    user.pending_rewards = user.pending_rewards - rewards;  // Can underflow!

    // If rewards > pending_rewards, wraps to huge number
    // Attacker can drain reward pool
}
```

## Testing the Vulnerability

```typescript
it("Exploits arithmetic overflow", async () => {
  // Set vault to near maximum
  vault.totalDeposited = new BN("18446744073709551615"); // u64::MAX

  // Try to deposit 1 more
  await program.methods
    .depositInsecure(new BN(1))
    .accounts({ vault: vaultPda })
    .rpc();

  // ❌ Vulnerable: total wraps to 0
  // ✅ Secure: transaction fails with MathOverflow error
});
```

## Best Practices

### ✅ DO

1. **Always use checked arithmetic** in financial calculations
2. **Use `checked_*` methods** for all math operations
3. **Test boundary conditions** (0, u64::MAX, etc.)
4. **Consider using SafeMath** or similar libraries

### ❌ DON'T

1. **Don't use standard operators** (`+`, `-`, `*`, `/`) for financial math
2. **Don't assume** values will stay in range
3. **Don't use `wrapping_*`** unless you specifically want wrap behavior
4. **Don't skip** overflow testing

## Anchor Safe Math

Anchor provides macros for safe arithmetic:

```rust
use anchor_lang::prelude::*;

// Option 1: checked_* methods
let result = a.checked_add(b).ok_or(ErrorCode::Overflow)?;

// Option 2: saturating operations (caps at max/min)
let result = a.saturating_add(b);

// Option 3: Manually check before operation
require!(a <= u64::MAX - b, ErrorCode::Overflow);
let result = a + b;
```

## Related Vulnerabilities

- [Account Data Matching](../04-account-data-matching/) - State validation
- [Re-entrancy](../06-reentrance-attack/) - State consistency

## References

- [Rust Overflow Documentation](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
- [Anchor Overflow Checks](https://www.anchor-lang.com/docs/overflow-checks)
- [Solana Math Safety](https://docs.rs/solana-program/latest/solana_program/index.html)

---

**⚠️ Remember:** Always use checked arithmetic for financial operations. Unchecked math is the source of many blockchain exploits.
