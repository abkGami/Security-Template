# 📞 Arbitrary CPI Vulnerability

## Overview

**Risk Level:** 🔴 CRITICAL  
**Real-World Impact:** Crema Finance ($8.8M, 2022)

The Arbitrary CPI vulnerability occurs when a program accepts user-supplied program IDs for cross-program invocations without validation. This allows attackers to invoke malicious programs that can drain funds, manipulate state, or execute unauthorized operations.

## The Vulnerability

### ❌ Vulnerable Code Pattern

```rust
pub fn invoke_external(ctx: Context<ArbitraryCPI>, amount: u64) -> Result<()> {
    // VULNERABILITY: Accepts any program ID from user
    // Attacker can pass their malicious program

    let ix = solana_program::instruction::Instruction {
        program_id: ctx.accounts.external_program.key(),  // ⚠️ User-controlled!
        accounts: vec![/* ... */],
        data: vec![/* ... */],
    };

    // Executes attacker's code!
    invoke(&ix, &[/* accounts */])?;

    Ok(())
}
```

### Attack Scenario

1. Protocol expects CPI to legitimate Token Program
2. Attacker passes their malicious program instead
3. Malicious program:
   - Returns fake success
   - Doesn't actually transfer tokens
   - Manipulates state incorrectly
   - Drains funds through backdoor
4. Your program trusts the malicious response
5. Accounting becomes corrupted

## The Fix

### ✅ Secure Implementation

```rust
#[derive(Accounts)]
pub struct SecureCPI<'info> {
    // SOLUTION: Use Program<'info, T> to enforce specific program
    pub token_program: Program<'info, Token>,

    // OR: Use address constraint
    #[account(address = spl_token::ID)]
    pub token_program_checked: AccountInfo<'info>,
}

pub fn secure_cpi(ctx: Context<SecureCPI>) -> Result<()> {
    // ✅ Only legitimate Token Program can be invoked
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer { /* ... */ }
        ),
        amount
    )?;

    Ok(())
}
```

## Best Practices

### ✅ DO

1. Use `Program<'info, T>` for all external programs
2. Whitelist allowed program IDs
3. Validate program accounts before CPI
4. Use Anchor's CPI utilities

### ❌ DON'T

1. Accept arbitrary program IDs from users
2. Trust user-provided program accounts
3. Skip program ID validation
4. Use raw `invoke` without checks

## References

- [Crema Finance Exploit](https://www.certik.com/resources/blog/crema-finance-incident-analysis)
- [Anchor CPI](https://www.anchor-lang.com/docs/cross-program-invocations)
