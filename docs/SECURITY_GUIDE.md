# 🛡️ Complete Solana Security Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Solana's Account Model & Security](#solanas-account-model--security)
3. [The 7 Critical Vulnerabilities](#the-7-critical-vulnerabilities)
4. [Anchor Security Framework](#anchor-security-framework)
5. [CPI Security Patterns](#cpi-security-patterns)
6. [Testing Methodologies](#testing-methodologies)
7. [Production Deployment Checklist](#production-deployment-checklist)
8. [Real-World Exploit Analysis](#real-world-exploit-analysis)
9. [Security Tools & Auditing](#security-tools--auditing)
10. [Best Practices Summary](#best-practices-summary)

---

## Introduction

**Why Security Matters in Solana**

Since 2022, Solana programs have lost over $400M to security exploits. Unlike other blockchains, Solana's account model introduces unique security considerations that developers must understand:

- **Everything is an account** - Programs, data, tokens, users
- **Accounts don't validate themselves** - Your code must verify everything
- **Parallel execution** - Re-entrancy and state consistency matter
- **No built-in access control** - You implement all permissions

This guide provides a comprehensive framework for building secure Solana programs by examining real vulnerabilities and their fixes.

---

## Solana's Account Model & Security

### Understanding Accounts

Every account in Solana has:

```rust
struct Account {
    lamports: u64,        // SOL balance
    data: Vec<u8>,        // Arbitrary data
    owner: Pubkey,        // Program that owns this account
    executable: bool,     // Is this a program?
    rent_epoch: u64,      // Rent tracking
}
```

### Key Security Principles

**1. Programs Own Accounts**

Only the owning program can modify an account's data:

```rust
// Token Program owns all token accounts
token_account.owner == spl_token::ID

// Your program owns your custom accounts
custom_account.owner == your_program::ID
```

**2. Anyone Can Pass Any Account**

Transactions can include arbitrary accounts. Your program must verify:

- ✅ Account is owned by expected program
- ✅ Account was signed by correct authority
- ✅ Account relationships are correct (PDAs, has_one, etc.)
- ✅ Account discriminator matches expected type

**3. Account Data is Untrusted**

Never trust account data until verified:

```rust
// ❌ WRONG: Deserialize without checks
let data = Account::try_deserialize(account)?;

// ✅ RIGHT: Use Anchor's Account<'info, T>
// Automatically checks owner and discriminator
pub account: Account<'info, MyAccount>
```

---

## The 7 Critical Vulnerabilities

### 1. Missing Signer Check 🔐

**What:** Failing to verify an account signed the transaction

**Impact:** CRITICAL - Complete loss of funds, unauthorized operations

**Real Exploit:** Wormhole Bridge ($325M, 2022)

**Vulnerable Pattern:**

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    // ❌ No signature verification!
    require!(ctx.accounts.authority.key() == expected_authority)?;
    // Anyone can pass authority pubkey without signing
}
```

**Secure Pattern:**

```rust
#[derive(Accounts)]
pub struct Transfer<'info> {
    // ✅ Enforces signature
    pub authority: Signer<'info>,
    // OR
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}
```

**Detection:**

- Look for `AccountInfo<'info>` used as authority
- Check for manual pubkey comparisons without `is_signer`
- Verify all privileged operations require signatures

**Prevention:**

- Use `Signer<'info>` type for all authorities
- Add `#[account(signer)]` constraint
- Test unauthorized access scenarios

---

### 2. Missing Owner Check 👤

**What:** Not verifying an account is owned by the correct program

**Impact:** CRITICAL - Account substitution, fake data injection

**Real Exploit:** Multiple DeFi protocols

**Vulnerable Pattern:**

```rust
pub fn process(ctx: Context<Process>) -> Result<()> {
    // ❌ No owner verification!
    let data = ctx.accounts.user_account.data.borrow();
    let balance = u64::from_le_bytes(...);
    // Attacker can pass fake account with inflated balance
}
```

**Secure Pattern:**

```rust
#[derive(Accounts)]
pub struct Process<'info> {
    // ✅ Enforces owner == Token Program
    pub user_account: Account<'info, TokenAccount>,
    // OR
    #[account(owner = expected_program)]
    pub user_account: AccountInfo<'info>,
}
```

**Detection:**

- Find `AccountInfo<'info>` without owner constraints
- Look for manual data deserialization
- Check for missing `Account<'info, T>` wrappers

**Prevention:**

- Use Anchor's `Account<'info, T>` types
- Add `owner` constraints when using `AccountInfo`
- Verify owner before any deserialization

---

### 3. Arithmetic Overflow/Underflow 🔢

**What:** Math operations that wrap instead of erroring

**Impact:** HIGH - Unlimited minting, balance manipulation, broken accounting

**Real Exploit:** Multiple token programs ($50M+ total)

**Vulnerable Pattern:**

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // ❌ Wraps on overflow in release mode
    vault.total_deposited = vault.total_deposited + amount;
    user.balance = user.balance + amount;
}
```

**Secure Pattern:**

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // ✅ Returns error on overflow
    vault.total_deposited = vault.total_deposited
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    user.balance = user.balance
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
}
```

**All Checked Operations:**

```rust
// Addition
let sum = a.checked_add(b).ok_or(Error)?;

// Subtraction
let diff = a.checked_sub(b).ok_or(Error)?;

// Multiplication
let product = a.checked_mul(b).ok_or(Error)?;

// Division
let quotient = a.checked_div(b).ok_or(Error)?;

// Power
let power = a.checked_pow(n).ok_or(Error)?;
```

**Detection:**

- Search for `+`, `-`, `*`, `/` operators on financial types
- Look for unchecked math in balance updates
- Check reward calculations and interest formulas

**Prevention:**

- Use `checked_*` for ALL financial math
- Enable `overflow-checks = true` in Cargo.toml
- Test boundary values (0, u64::MAX)
- Consider saturating\_\* for non-financial operations

---

### 4. Account Data Matching (PDA Verification) 🎭

**What:** Not verifying PDAs are derived with correct seeds

**Impact:** CRITICAL - Account substitution, fake state

**Real Exploit:** Cashio ($52M, 2022)

**Vulnerable Pattern:**

```rust
pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    // ❌ No PDA verification!
    let user_stats = &ctx.accounts.user_stats;
    // Attacker passed fake user_stats with inflated balance
    transfer(vault, user, user_stats.balance)?;
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user_stats: Account<'info, UserStats>,  // No seeds check!
}
```

**Secure Pattern:**

```rust
#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: Signer<'info>,

    // ✅ Verifies PDA derivation
    #[account(
        seeds = [b"user_stats", user.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,

    // ✅ Verifies account relationships
    #[account(
        mut,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub vault: Account<'info, Vault>,
}
```

**Manual Verification:**

```rust
pub fn verify_pda(ctx: Context<Process>) -> Result<()> {
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[b"user", ctx.accounts.user.key().as_ref()],
        ctx.program_id
    );

    require!(
        ctx.accounts.user_pda.key() == expected_pda,
        ErrorCode::InvalidPDA
    );
}
```

**Detection:**

- Find PDA accounts without `seeds` constraints
- Look for missing `bump` storage
- Check for `has_one` constraints on related accounts

**Prevention:**

- Always use `seeds` and `bump` constraints
- Store bump in account state
- Use `has_one` to verify account relationships
- Test with fake PDAs derived from wrong seeds

---

### 5. Arbitrary CPI (Cross-Program Invocation) 📞

**What:** Allowing untrusted programs to be invoked

**Impact:** CRITICAL - Malicious code execution, fund drainage

**Real Exploit:** Crema Finance ($8.8M, 2022)

**Vulnerable Pattern:**

```rust
pub fn arbitrary_cpi(ctx: Context<ArbitraryCPI>) -> Result<()> {
    // ❌ Accepts any program!
    let cpi_accounts = SomeAccounts {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
    };

    // Attacker can pass malicious program
    let cpi_ctx = CpiContext::new(
        ctx.accounts.external_program.to_account_info(),
        cpi_accounts
    );

    // Executes attacker's code!
    some_cpi::transfer(cpi_ctx, amount)?;
}
```

**Secure Pattern:**

```rust
#[derive(Accounts)]
pub struct SecureCPI<'info> {
    // ✅ Enforces specific program
    pub token_program: Program<'info, Token>,
    // OR
    #[account(address = spl_token::ID)]
    pub token_program: AccountInfo<'info>,
}

pub fn secure_cpi(ctx: Context<SecureCPI>) -> Result<()> {
    // ✅ Only trusted program can be invoked
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferAccounts { /* ... */ }
    );

    token::transfer(cpi_ctx, amount)?;
}
```

**Detection:**

- Look for `AccountInfo` used as program IDs
- Find CPI calls without program validation
- Check for user-supplied program IDs

**Prevention:**

- Use `Program<'info, T>` type
- Add `address` constraints for known programs
- Never accept arbitrary program IDs from users
- Whitelist allowed programs

---

### 6. Re-entrancy via CPI 🔄

**What:** External calls that allow recursive state manipulation

**Impact:** HIGH - State corruption, double-spending

**Real Pattern:** Adapted from Ethereum's DAO hack

**Vulnerable Pattern:**

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;

    // ⚠️ External call BEFORE state update
    invoke_signed(
        &transfer_instruction,
        &accounts,
        &signer_seeds
    )?;

    // ❌ State updated after external call
    // Attacker can re-enter and withdraw multiple times
    user.balance = user.balance.checked_sub(amount).unwrap();
}
```

**Secure Pattern:**

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;

    // ✅ CHECKS: Verify conditions
    require!(user.balance >= amount, ErrorCode::InsufficientFunds);

    // ✅ EFFECTS: Update state FIRST
    user.balance = user.balance.checked_sub(amount).unwrap();

    // ✅ INTERACTIONS: External calls LAST
    invoke_signed(
        &transfer_instruction,
        &accounts,
        &signer_seeds
    )?;
}
```

**Checks-Effects-Interactions Pattern:**

```rust
pub fn secure_operation(ctx: Context<Op>) -> Result<()> {
    // 1. CHECKS - Validate all conditions
    require!(condition1, Error::Invalid);
    require!(condition2, Error::Invalid);

    // 2. EFFECTS - Update all state
    ctx.accounts.state.balance -= amount;
    ctx.accounts.state.last_update = Clock::get()?.unix_timestamp;

    // 3. INTERACTIONS - External calls last
    cpi_call()?;

    Ok(())
}
```

**Detection:**

- Find state updates after CPI calls
- Look for external calls in loops
- Check for missing re-entrancy guards

**Prevention:**

- Follow Checks-Effects-Interactions pattern
- Update state before external calls
- Use re-entrancy guards when needed
- Test with malicious callback scenarios

---

### 7. Type Cosplay (Account Type Confusion) 🎪

**What:** Accepting accounts with wrong discriminators/types

**Impact:** MEDIUM-HIGH - Logic bypass, unauthorized access

**Real Exploit:** Solend, Jet Protocol issues

**Vulnerable Pattern:**

```rust
pub fn process(ctx: Context<Process>) -> Result<()> {
    // ❌ No type verification!
    let account_data = ctx.accounts.any_account.data.borrow();

    // Deserializes whatever data is there
    let config: Config = try_from_slice(&account_data)?;

    // Attacker passed wrong account type with crafted data
}
```

**Secure Pattern:**

```rust
#[derive(Accounts)]
pub struct Process<'info> {
    // ✅ Verifies discriminator automatically
    pub config: Account<'info, Config>,

    // ✅ Enforces account type
    pub vault: Account<'info, Vault>,
}
```

**How Anchor Discriminators Work:**

```rust
// Anchor adds 8-byte discriminator to each account
// Discriminator = hash("account:AccountName")[..8]

#[account]
pub struct MyAccount {
    pub data: u64,
}

// Stored as: [discriminator: 8 bytes][data: 8 bytes]
```

**Detection:**

- Find manual deserialization without discriminator checks
- Look for `AccountInfo` where `Account<T>` should be used
- Check for missing type validation

**Prevention:**

- Use `Account<'info, T>` for all typed accounts
- Let Anchor handle discriminator validation
- Never manually deserialize without checking discriminator
- Use `#[account]` macro for all custom account types

---

## Anchor Security Framework

### Account Validation Constraints

```rust
#[derive(Accounts)]
pub struct SecureInstruction<'info> {
    // Signer verification
    #[account(signer)]
    pub authority: Signer<'info>,

    // Owner verification
    #[account(owner = token_program.key())]
    pub token_account: Account<'info, TokenAccount>,

    // PDA verification
    #[account(
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    // Relationship verification
    #[account(
        has_one = authority @ ErrorCode::Unauthorized,
        has_one = token_account @ ErrorCode::InvalidToken
    )]
    pub user_account: Account<'info, UserAccount>,

    // Mutability
    #[account(mut)]
    pub mutable_account: Account<'info, SomeAccount>,

    // Program ID verification
    pub token_program: Program<'info, Token>,

    // Address verification
    #[account(address = spl_token::ID)]
    pub verified_program: AccountInfo<'info>,

    // Custom constraints
    #[account(
        constraint = user_account.balance >= 1000 @ ErrorCode::InsufficientBalance
    )]
    pub user_account2: Account<'info, UserAccount>,
}
```

### Init Constraints (Account Creation)

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,                                    // Create new account
        payer = user,                           // Who pays for rent
        space = 8 + UserAccount::INIT_SPACE,   // Account size
        seeds = [b"user", user.key().as_ref()], // PDA seeds
        bump                                     // Find valid bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
```

### Close Constraints (Account Deletion)

```rust
#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut,
        close = receiver,  // Transfer remaining lamports to receiver
        has_one = authority
    )]
    pub account_to_close: Account<'info, UserAccount>,

    pub authority: Signer<'info>,

    /// CHECK: Receives lamports from closed account
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
}
```

---

## CPI Security Patterns

### Secure Token Transfer

```rust
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn secure_token_transfer(ctx: Context<SecureTransfer>, amount: u64) -> Result<()> {
    // ✅ Verified token program
    // ✅ Verified token accounts
    // ✅ Verified authority

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts
    );

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct SecureTransfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
```

### CPI with Signer Seeds (PDA Signing)

```rust
pub fn cpi_with_pda_signer(ctx: Context<PDATransfer>, amount: u64) -> Result<()> {
    let authority_bump = ctx.accounts.authority_pda.bump;
    let authority_seeds = &[
        b"authority",
        ctx.accounts.user.key().as_ref(),
        &[authority_bump]
    ];
    let signer_seeds = &[&authority_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority_pda.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds  // ✅ PDA signs the transaction
    );

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
```

---

## Testing Methodologies

### Test Structure

```typescript
describe("Security Tests", () => {
  // Test legitimate functionality
  it("allows authorized operations", async () => {
    // ✅ Should succeed
  });

  // Test attack vectors
  it("prevents unauthorized operations", async () => {
    try {
      // ❌ Should fail
      await exploitAttempt();
      throw new Error("Exploit should have failed");
    } catch (error) {
      expect(error.message).to.include("Unauthorized");
    }
  });

  // Test boundary conditions
  it("handles edge cases correctly", async () => {
    // Test with 0, MAX values
    // Test with invalid accounts
    // Test with wrong signers
  });
});
```

### Exploit Test Template

```typescript
describe("Vulnerability Test", () => {
  let attacker: Keypair;
  let victim: Keypair;
  let vulnerableAccount: PublicKey;

  before(async () => {
    // Setup
    attacker = Keypair.generate();
    victim = Keypair.generate();
    // ... initialize accounts
  });

  it("demonstrates the exploit", async () => {
    const balanceBefore = await getBalance(victim);

    // Attempt exploit
    try {
      await program.methods
        .vulnerableInstruction()
        .accounts({
          authority: victim.publicKey, // Victim's account
        })
        .signers([attacker]) // But attacker signs!
        .rpc();

      // Check if exploit succeeded
      const balanceAfter = await getBalance(victim);

      if (balanceAfter < balanceBefore) {
        console.log("🚨 EXPLOIT SUCCESSFUL - VULNERABILITY CONFIRMED");
      }
    } catch (error) {
      console.log("✅ Exploit prevented");
    }
  });

  it("secure version prevents exploit", async () => {
    try {
      await program.methods
        .secureInstruction()
        .accounts({
          authority: victim.publicKey,
        })
        .signers([attacker])
        .rpc();

      throw new Error("Should have failed");
    } catch (error) {
      expect(error.message).to.include("unknown signer");
      console.log("✅ Security check working");
    }
  });
});
```

---

## Production Deployment Checklist

### Pre-Deployment Security Checklist

- [ ] **Signer Checks**
  - [ ] All authority accounts use `Signer<'info>`
  - [ ] All privileged operations verify signatures
  - [ ] No `AccountInfo` used for authorities without checks

- [ ] **Owner Checks**
  - [ ] All SPL token accounts use `Account<'info, TokenAccount>`
  - [ ] All custom accounts use `Account<'info, T>`
  - [ ] No manual deserialization without owner verification

- [ ] **Arithmetic Safety**
  - [ ] All financial math uses `checked_*` operations
  - [ ] `overflow-checks = true` in Cargo.toml
  - [ ] Boundary values tested (0, MAX)

- [ ] **PDA Verification**
  - [ ] All PDAs have `seeds` and `bump` constraints
  - [ ] Bumps stored in account state
  - [ ] `has_one` used for related accounts

- [ ] **CPI Security**
  - [ ] All external programs verified with `Program<'info, T>`
  - [ ] No arbitrary program IDs accepted
  - [ ] Checks-Effects-Interactions pattern followed

- [ ] **Type Safety**
  - [ ] All accounts use Anchor's `Account<'info, T>`
  - [ ] Discriminators automatically checked
  - [ ] No manual deserialization without validation

- [ ] **Testing**
  - [ ] Unit tests for all instructions
  - [ ] Integration tests for user flows
  - [ ] Exploit tests for each vulnerability
  - [ ] Edge cases tested
  - [ ] Fuzz testing performed

- [ ] **Code Review**
  - [ ] Peer reviewed by multiple developers
  - [ ] Security-focused review completed
  - [ ] External audit performed (if high-value)

- [ ] **Tools Run**
  - [ ] Soteria static analyzer
  - [ ] Anchor verify
  - [ ] Cargo clippy
  - [ ] Cargo audit

### Deployment Process

1. **Local Testing**

   ```bash
   anchor test
   RUST_LOG=debug anchor test
   ```

2. **Devnet Deployment**

   ```bash
   anchor build
   anchor deploy --provider.cluster devnet
   ```

3. **Testnet Validation**
   - Deploy to testnet
   - Run integration tests
   - Have community test

4. **Mainnet Preparation**
   - Audit complete
   - Bug bounty program active
   - Emergency procedures documented
   - Monitoring in place

5. **Mainnet Deployment**

   ```bash
   anchor build --verifiable
   anchor deploy --provider.cluster mainnet
   solana-verify verify-from-repo <program-id>
   ```

6. **Post-Deployment**
   - Monitor transactions
   - Watch for unusual patterns
   - Have upgrade plan ready
   - Maintain communication channels

---

## Real-World Exploit Analysis

### Wormhole Bridge ($325M) - Missing Signer Check

**What Happened:**

- Guardian set verification didn't check signatures
- Attacker forged guardian signatures
- Minted 120,000 wrapped ETH without collateral

**Root Cause:**

```rust
// Simplified vulnerable code
fn verify_signatures(ctx: Context<Verify>) {
    let guardian_set = load_guardian_set()?;
    // ❌ MISSING: Actual signature verification
    // Program assumed signatures were valid
}
```

**The Fix:**

```rust
fn verify_signatures(ctx: Context<Verify>) {
    let guardian_set = load_guardian_set()?;

    // ✅ Verify each signature cryptographically
    for (i, signature) in signatures.iter().enumerate() {
        let guardian = &guardian_set.keys[i];
        require!(
            verify_signature(message, signature, guardian),
            ErrorCode::InvalidSignature
        );
    }
}
```

**Lessons:**

1. Never assume cryptographic operations succeeded
2. Always verify signatures explicitly
3. Test with invalid signatures
4. Use established libraries for cryptography

---

### Cashio ($52M) - Account Data Matching

**What Happened:**

- Program didn't verify PDA derivation
- Attacker created fake collateral account
- Minted stablecoins without real collateral

**Root Cause:**

```rust
#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    pub collateral: Account<'info, CollateralAccount>,
    // ❌ MISSING: seeds and bump verification
}
```

**The Fix:**

```rust
#[derive(Accounts)]
pub struct MintStablecoin<'info> {
    #[account(
        seeds = [b"collateral", mint.key().as_ref()],
        bump = collateral.bump,
        has_one = mint
    )]
    pub collateral: Account<'info, CollateralAccount>,
}
```

**Lessons:**

1. Always verify PDA derivation
2. Use `seeds` and `bump` constraints
3. Test with fake PDAs
4. Validate all account relationships

---

### Crema Finance ($8.8M) - Arbitrary CPI

**What Happened:**

- Accepted arbitrary program IDs
- Attacker passed malicious program
- Drained liquidity pools

**Root Cause:**

```rust
pub fn swap(ctx: Context<Swap>) {
    // ❌ Accepts any program
    invoke(
        &instruction,
        &[ctx.accounts.external_program]  // Attacker controlled
    )?;
}
```

**The Fix:**

```rust
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(address = ALLOWED_PROGRAM_ID)]
    pub external_program: Program<'info, AllowedProgram>,
}
```

**Lessons:**

1. Never accept arbitrary program IDs
2. Whitelist allowed programs
3. Use `Program<'info, T>` type
4. Validate before every CPI

---

## Security Tools & Auditing

### Static Analysis Tools

**Soteria**

```bash
cargo install soteria
soteria -analyzeAll .
```

**Cargo Audit**

```bash
cargo install cargo-audit
cargo audit
```

**Clippy**

```bash
cargo clippy -- -D warnings
```

### Verification Tools

**Anchor Verify**

```bash
anchor build --verifiable
solana-verify verify-from-repo \
  -um --program-id <PROGRAM_ID> \
  https://github.com/your/repo
```

### Runtime Monitoring

Monitor for:

- Unusual transaction patterns
- Failed transactions (attempted exploits)
- Large value transfers
- Repeated access patterns
- Account balance changes

### Audit Providers

1. **Neodyme** - Specialized in Solana
2. **Sec3** - Automated + manual audits
3. **OtterSec** - DeFi focused
4. **Trail of Bits** - Comprehensive audits
5. **Kudelski Security** - Enterprise audits

### Bug Bounty Programs

Set up on:

- Immunefi
- HackerOne
- Self-hosted

Typical rewards:

- Critical: $50K - $1M+
- High: $10K - $50K
- Medium: $5K - $10K
- Low: $1K - $5K

---

## Best Practices Summary

### Account Validation

```rust
// ✅ Perfect account validation
#[derive(Accounts)]
pub struct PerfectValidation<'info> {
    // 1. Signer verification
    #[account(mut)]
    pub authority: Signer<'info>,

    // 2. Owner verification (automatic with Account type)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    // 3. PDA verification
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub vault: Account<'info, Vault>,

    // 4. Program verification
    pub token_program: Program<'info, Token>,

    // 5. Address verification
    #[account(address = KNOWN_PROGRAM_ID)]
    pub known_program: AccountInfo<'info>,

    // 6. Custom validation
    #[account(
        constraint = vault.active @ ErrorCode::VaultInactive
    )]
    pub active_vault: Account<'info, Vault>,
}
```

### Safe Math

```rust
// ✅ Always use checked operations
pub fn safe_math_example(a: u64, b: u64, c: u64) -> Result<u64> {
    let step1 = a.checked_add(b).ok_or(ErrorCode::Overflow)?;
    let step2 = step1.checked_mul(c).ok_or(ErrorCode::Overflow)?;
    let step3 = step2.checked_sub(100).ok_or(ErrorCode::Underflow)?;
    Ok(step3)
}
```

### Checks-Effects-Interactions

```rust
// ✅ Perfect CEI pattern
pub fn perfect_withdrawal(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // CHECKS
    require!(ctx.accounts.vault.active, ErrorCode::VaultInactive);
    require!(ctx.accounts.user.balance >= amount, ErrorCode::Insufficient);
    require!(amount > 0, ErrorCode::InvalidAmount);

    // EFFECTS
    ctx.accounts.user.balance = ctx.accounts.user.balance
        .checked_sub(amount)
        .ok_or(ErrorCode::Underflow)?;

    ctx.accounts.vault.total_withdrawn = ctx.accounts.vault.total_withdrawn
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    emit!(WithdrawalEvent {
        user: ctx.accounts.user.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    // INTERACTIONS
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            }
        ),
        amount
    )?;

    Ok(())
}
```

### Error Handling

```rust
#[error_code]
pub enum ErrorCode {
    // Specific, descriptive errors
    #[msg("Signer verification failed")]
    MissingSigner,

    #[msg("Account owner must be Token Program")]
    InvalidOwner,

    #[msg("Math operation overflowed")]
    MathOverflow,

    #[msg("PDA derivation does not match")]
    InvalidPDA,

    #[msg("Balance insufficient for operation")]
    InsufficientBalance,
}
```

### Testing

```typescript
// ✅ Comprehensive test suite
describe("Comprehensive Security Tests", () => {
  // Happy path
  it("allows legitimate operations", async () => {
    // Test normal usage
  });

  // Attack vectors
  it("prevents unauthorized signer", async () => {
    // Test missing signer check
  });

  it("prevents fake account substitution", async () => {
    // Test missing owner check
  });

  it("prevents arithmetic overflow", async () => {
    // Test with MAX values
  });

  it("prevents fake PDA", async () => {
    // Test with wrong seeds
  });

  it("prevents arbitrary CPI", async () => {
    // Test with malicious program
  });

  // Edge cases
  it("handles zero values", async () => {});
  it("handles maximum values", async () => {});
  it("handles concurrent operations", async () => {});
});
```

---

## Conclusion

Security in Solana requires:

1. **Deep understanding** of the account model
2. **Systematic validation** of all inputs
3. **Safe arithmetic** in all calculations
4. **Proper PDA verification**
5. **Careful CPI handling**
6. **Comprehensive testing**
7. **Professional auditing**

**Remember:**

- 🔒 Verify everything - trust nothing
- 🧮 Check all math operations
- 🎯 Validate all PDAs
- 🔑 Verify all signers
- 👤 Check all owners
- 📞 Validate all CPIs
- 🧪 Test all attacks

**The cost of security is low. The cost of insecurity is everything.**

---

_Last Updated: January 2026_
_Anchor Version: 0.30.0_
_Solana Version: 1.18.0_

---

**Additional Resources:**

- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Sealevel Attacks Repository](https://github.com/coral-xyz/sealevel-attacks)
- [Neodyme Blog](https://blog.neodyme.io/)
- [Sec3 Documentation](https://www.sec3.dev/docs)
