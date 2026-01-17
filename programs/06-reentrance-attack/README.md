# 🔄 Re-entrancy Attack Vulnerability

## Overview

**Risk Level:** 🔴 HIGH  
**Pattern:** Adapted from Ethereum's DAO hack

Re-entrancy occurs when external calls allow recursive invocations before state is fully updated, leading to double-spending and state corruption.

## The Vulnerability

```rust
pub fn withdraw_vulnerable(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // ⚠️ External call BEFORE state update
    invoke_signed(&transfer_ix, &accounts, &seeds)?;

    // State updated after - attacker can re-enter
    ctx.accounts.user.balance -= amount;
}
```

## The Fix

```rust
pub fn withdraw_secure(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // ✅ CHECKS
    require!(user.balance >= amount, Error::Insufficient);

    // ✅ EFFECTS - Update state FIRST
    ctx.accounts.user.balance -= amount;

    // ✅ INTERACTIONS - External calls LAST
    invoke_signed(&transfer_ix, &accounts, &seeds)?;
}
```

## Checks-Effects-Interactions Pattern

Always follow this order:

1. **Checks** - Validate conditions
2. **Effects** - Update state
3. **Interactions** - Make external calls
