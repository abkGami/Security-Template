# 🎪 Type Cosplay (Account Type Confusion) Vulnerability

## Overview

**Risk Level:** 🟡 MEDIUM-HIGH  
**Impact:** Logic bypass, unauthorized access

Type Cosplay occurs when a program fails to verify account discriminators, allowing attackers to substitute accounts of wrong types.

## The Vulnerability

```rust
pub fn process_vulnerable(ctx: Context<Process>) -> Result<()> {
    // ⚠️ Manual deserialization without discriminator check
    let data = ctx.accounts.account.data.borrow();
    let config: Config = try_from_slice(&data)?;

    // Attacker passed wrong account type with crafted data
}
```

## The Fix

```rust
#[derive(Accounts)]
pub struct ProcessSecure<'info> {
    // ✅ Account<'info, Config> checks discriminator automatically
    pub config: Account<'info, Config>,
}
```

## How Anchor Discriminators Work

Every `#[account]` gets an 8-byte discriminator:

```rust
discriminator = hash("account:AccountName")[..8]
```

Anchor verifies this before deserialization, preventing type confusion.

## Best Practices

- Use `Account<'info, T>` for all typed accounts
- Never manually deserialize without checking discriminator
- Let Anchor handle validation
