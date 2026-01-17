# 📋 Anchor Constraints Reference

## Complete Guide to Anchor Account Validation

### Table of Contents

1. [Signer Constraints](#signer-constraints)
2. [Owner Constraints](#owner-constraints)
3. [PDA Constraints](#pda-constraints)
4. [Relationship Constraints](#relationship-constraints)
5. [State Constraints](#state-constraints)
6. [Initialization Constraints](#initialization-constraints)
7. [Closing Constraints](#closing-constraints)
8. [Token Constraints](#token-constraints)
9. [Custom Constraints](#custom-constraints)

---

## Signer Constraints

### `signer`

Verifies that the account signed the transaction.

```rust
#[account(signer)]
pub authority: AccountInfo<'info>
```

**Alternative:** Use `Signer<'info>` type (preferred)

```rust
pub authority: Signer<'info>
```

---

## Owner Constraints

### `owner`

Verifies the account is owned by the specified program.

```rust
#[account(owner = token_program.key())]
pub token_account: AccountInfo<'info>
```

**Common owners:**

```rust
owner = token_program.key()        // SPL Token Program
owner = crate::ID                  // Your program
owner = system_program.key()       // System Program
```

---

## PDA Constraints

### `seeds` + `bump`

Verifies a PDA was derived with specific seeds.

```rust
#[account(
    seeds = [b"vault", authority.key().as_ref()],
    bump = vault.bump
)]
pub vault: Account<'info, Vault>
```

**Finding bump:**

```rust
let (pda, bump) = Pubkey::find_program_address(
    &[b"vault", authority.key().as_ref()],
    program_id
);
```

**With `init`:**

```rust
#[account(
    init,
    payer = user,
    space = 8 + Vault::INIT_SPACE,
    seeds = [b"vault", user.key().as_ref()],
    bump  // Anchor finds the bump automatically
)]
pub vault: Account<'info, Vault>
```

---

## Relationship Constraints

### `has_one`

Verifies field in account matches another account's key.

```rust
#[account(has_one = authority @ ErrorCode::Unauthorized)]
pub vault: Account<'info, Vault>

// Checks: vault.authority == authority.key()
```

**Multiple relationships:**

```rust
#[account(
    has_one = authority,
    has_one = token_mint,
    has_one = reward_vault
)]
pub config: Account<'info, Config>
```

---

## State Constraints

### `mut`

Marks account as mutable (can be modified).

```rust
#[account(mut)]
pub vault: Account<'info, Vault>
```

**Required for:**

- Modifying account data
- Changing lamport balance
- Closing accounts

### `address`

Verifies account has specific public key.

```rust
#[account(address = KNOWN_ADDRESS)]
pub config: Account<'info, Config>

#[account(address = spl_token::ID)]
pub token_program: AccountInfo<'info>
```

**Use cases:**

- Verify known program IDs
- Check for specific config accounts
- Validate system accounts

---

## Initialization Constraints

### `init`

Creates a new account.

```rust
#[account(
    init,
    payer = user,
    space = 8 + UserAccount::INIT_SPACE
)]
pub user_account: Account<'info, UserAccount>
```

**Required with `init`:**

- `payer` - Who pays rent
- `space` - Account size in bytes
- `system_program` - Must be in accounts

### `init_if_needed`

Initializes account if it doesn't exist.

```rust
#[account(
    init_if_needed,
    payer = user,
    space = 8 + Config::INIT_SPACE
)]
pub config: Account<'info, Config>
```

⚠️ **Warning:** Use carefully - can be exploited if not properly validated.

### `payer`

Specifies who pays for account creation.

```rust
#[account(
    init,
    payer = authority,  // authority pays
    space = 8 + 32
)]
pub new_account: Account<'info, NewAccount>
```

### `space`

Specifies account size in bytes.

```rust
#[account(
    init,
    payer = user,
    space = 8 + 32 + 8 + 1  // discriminator + pubkey + u64 + bool
)]
pub account: Account<'info, MyAccount>
```

**Calculation:**

```rust
#[account]
pub struct MyAccount {
    pub authority: Pubkey,    // 32 bytes
    pub balance: u64,         // 8 bytes
    pub active: bool,         // 1 byte
}

impl MyAccount {
    pub const INIT_SPACE: usize = 32 + 8 + 1;  // 41 bytes
    // Total with discriminator: 8 + 41 = 49 bytes
}
```

---

## Closing Constraints

### `close`

Closes an account and returns lamports.

```rust
#[account(
    mut,
    close = receiver,  // Send lamports here
    has_one = authority
)]
pub account_to_close: Account<'info, MyAccount>

pub authority: Signer<'info>

/// CHECK: Receives lamports
#[account(mut)]
pub receiver: AccountInfo<'info>
```

**Effects:**

1. Transfers all lamports to receiver
2. Sets account data to zero
3. Sets owner to System Program

---

## Token Constraints

### For Token Accounts

```rust
use anchor_spl::token::{Token, TokenAccount};

#[account(
    mut,
    constraint = token_account.owner == authority.key(),
    constraint = token_account.mint == expected_mint.key()
)]
pub token_account: Account<'info, TokenAccount>
```

### For Mints

```rust
use anchor_spl::token::Mint;

#[account(
    init,
    payer = authority,
    mint::decimals = 9,
    mint::authority = authority
)]
pub mint: Account<'info, Mint>
```

### Associated Token Accounts

```rust
use anchor_spl::associated_token::AssociatedToken;

#[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = authority
)]
pub associated_token: Account<'info, TokenAccount>
```

---

## Custom Constraints

### `constraint`

Add arbitrary validation logic.

```rust
#[account(
    constraint = vault.active @ ErrorCode::VaultInactive,
    constraint = vault.balance >= 1000 @ ErrorCode::InsufficientBalance
)]
pub vault: Account<'info, Vault>
```

**Complex constraints:**

```rust
#[account(
    constraint =
        user.level >= 5 &&
        user.reputation > 100
        @ ErrorCode::InsufficientPrivileges
)]
pub user: Account<'info, User>
```

---

## Constraint Combinations

### Secure PDA with Authority

```rust
#[account(
    mut,
    seeds = [b"vault", authority.key().as_ref()],
    bump = vault.bump,
    has_one = authority @ ErrorCode::Unauthorized,
    constraint = vault.active @ ErrorCode::VaultInactive
)]
pub vault: Account<'info, Vault>

pub authority: Signer<'info>
```

### Secure Token Transfer

```rust
#[account(
    mut,
    constraint = from.owner == authority.key() @ ErrorCode::Unauthorized,
    constraint = from.mint == to.mint @ ErrorCode::MintMismatch
)]
pub from: Account<'info, TokenAccount>

#[account(mut)]
pub to: Account<'info, TokenAccount>

pub authority: Signer<'info>

pub token_program: Program<'info, Token>
```

### Secure Initialization

```rust
#[account(
    init,
    payer = payer,
    space = 8 + UserAccount::INIT_SPACE,
    seeds = [b"user", payer.key().as_ref()],
    bump
)]
pub user_account: Account<'info, UserAccount>

#[account(mut)]
pub payer: Signer<'info>

pub system_program: Program<'info, System>
```

---

## Account Types

### `Account<'info, T>`

Type-safe wrapper with automatic validation.

```rust
pub vault: Account<'info, Vault>
```

**Checks:**

- Owner == your program ID
- Discriminator matches
- Data deserializes correctly

### `AccountInfo<'info>`

Raw account with no automatic checks.

```rust
pub raw_account: AccountInfo<'info>
```

**Use when:**

- Working with external programs
- Need manual validation
- Account type unknown

⚠️ **Always validate manually!**

### `Signer<'info>`

Account that signed the transaction.

```rust
pub authority: Signer<'info>
```

Equivalent to:

```rust
#[account(signer)]
pub authority: AccountInfo<'info>
```

### `Program<'info, T>`

Executable program account.

```rust
pub token_program: Program<'info, Token>
```

**Checks:**

- Account is executable
- Owner is BPF Loader
- Address matches expected program

### `SystemAccount<'info>`

Account owned by System Program.

```rust
pub user: SystemAccount<'info>
```

**Checks:**

- Owner == System Program
- Used for SOL transfers

---

## Common Patterns

### Admin-Only Operation

```rust
#[account(
    has_one = admin @ ErrorCode::Unauthorized,
    constraint = config.enabled @ ErrorCode::Disabled
)]
pub config: Account<'info, Config>

pub admin: Signer<'info>
```

### Time-Locked Operation

```rust
#[account(
    constraint = Clock::get()?.unix_timestamp >= vault.unlock_time
        @ ErrorCode::StillLocked
)]
pub vault: Account<'info, Vault>
```

### Whitelist Check

```rust
#[account(
    constraint = whitelist.users.contains(&user.key())
        @ ErrorCode::NotWhitelisted
)]
pub whitelist: Account<'info, Whitelist>

pub user: Signer<'info>
```

---

## Security Best Practices

### ✅ Always Include

1. **Signer verification** for authorities

   ```rust
   pub authority: Signer<'info>
   ```

2. **Owner verification** for typed accounts

   ```rust
   pub token_account: Account<'info, TokenAccount>
   ```

3. **PDA verification** with seeds

   ```rust
   #[account(seeds = [...], bump)]
   ```

4. **Relationship verification** with has_one
   ```rust
   #[account(has_one = authority)]
   ```

### ❌ Never Do

1. Use `AccountInfo` without validation
2. Skip PDA verification for critical accounts
3. Forget `mut` for accounts you modify
4. Use `init_if_needed` without additional checks
5. Accept arbitrary program IDs

---

## Constraint Execution Order

Anchor executes constraints in this order:

1. `init` / `init_if_needed`
2. `mut`
3. `signer`
4. `seeds` + `bump`
5. `has_one`
6. `owner`
7. `address`
8. `constraint`
9. `close`

Understanding order helps debug constraint failures.

---

## Error Messages

### Custom Error Messages

```rust
#[account(
    constraint = vault.balance >= amount @ ErrorCode::InsufficientFunds
)]
pub vault: Account<'info, Vault>

#[error_code]
pub enum ErrorCode {
    #[msg("Vault has insufficient funds")]
    InsufficientFunds,
}
```

### Default Anchor Errors

- `ConstraintMut` - Account not mutable
- `ConstraintSigner` - Account didn't sign
- `ConstraintSeeds` - PDA derivation failed
- `ConstraintHasOne` - Relationship check failed
- `ConstraintOwner` - Wrong owner
- `ConstraintAddress` - Wrong address

---

## Testing Constraints

```typescript
it("enforces signer constraint", async () => {
  try {
    await program.methods
      .secureOperation()
      .accounts({
        authority: unauthorizedUser.publicKey,
      })
      .signers([unauthorizedUser])
      .rpc();

    throw new Error("Should have failed");
  } catch (error) {
    expect(error.message).to.include("unknown signer");
  }
});
```

---

**Remember:** Constraints are your first line of defense. Use them liberally!
