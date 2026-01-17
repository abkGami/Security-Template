# 🚀 Production Deployment Best Practices

## Pre-Deployment Security Checklist

### Code Review Checklist

#### Signer Verification

- [ ] All authority accounts use `Signer<'info>` or `#[account(signer)]`
- [ ] No privileged operations without signature checks
- [ ] Manual `is_signer` checks where Anchor constraints aren't used
- [ ] Test cases verify unauthorized access fails

#### Owner Validation

- [ ] All SPL token accounts use `Account<'info, TokenAccount>`
- [ ] Custom accounts use `Account<'info, T>` not `AccountInfo`
- [ ] Manual owner checks added where `AccountInfo` is necessary
- [ ] Owner validation tested with fake accounts

#### Arithmetic Safety

- [ ] All financial calculations use `checked_add`, `checked_sub`, `checked_mul`, `checked_div`
- [ ] No unchecked `+`, `-`, `*`, `/` operators on balances
- [ ] `overflow-checks = true` in Cargo.toml release profile
- [ ] Boundary values (0, u64::MAX) tested

#### PDA Verification

- [ ] All PDAs have `seeds` and `bump` constraints
- [ ] Bumps stored in account structs
- [ ] `has_one` used for account relationships
- [ ] Tests verify fake PDAs are rejected

#### CPI Security

- [ ] All external programs use `Program<'info, T>` type
- [ ] No arbitrary program IDs accepted from users
- [ ] Checks-Effects-Interactions pattern followed
- [ ] CPI invocations properly authorized

#### Type Safety

- [ ] All accounts use Anchor's typed `Account<'info, T>`
- [ ] Discriminators automatically handled
- [ ] No manual deserialization without validation
- [ ] Account type confusion tested

#### State Management

- [ ] Checks-Effects-Interactions pattern followed
- [ ] State updated before external calls
- [ ] Re-entrancy protections in place
- [ ] Concurrent access patterns considered

---

## Testing Requirements

### Unit Tests

```bash
# Run all tests
anchor test

# Run with logs
RUST_LOG=debug anchor test

# Run specific test file
anchor test tests/01-missing-signer-check.ts
```

Required test coverage:

- [ ] Happy path (legitimate operations)
- [ ] Authorization failures
- [ ] Arithmetic boundaries
- [ ] Invalid PDAs
- [ ] Malicious CPI attempts
- [ ] Edge cases and error conditions

### Integration Tests

- [ ] Multi-transaction user flows
- [ ] Cross-program interactions
- [ ] Token operations end-to-end
- [ ] Admin operations
- [ ] Emergency procedures

### Security Tests

- [ ] Exploit attempts for each vulnerability
- [ ] Unauthorized access scenarios
- [ ] Account substitution attacks
- [ ] Overflow/underflow conditions
- [ ] Re-entrancy simulations

---

## Static Analysis

### Run Soteria

```bash
# Install
cargo install soteria

# Analyze all programs
soteria -analyzeAll .

# Review findings in report
```

### Run Cargo Audit

```bash
# Check for vulnerable dependencies
cargo audit

# Update dependencies
cargo update
```

### Run Clippy

```bash
# Strict linting
cargo clippy -- -D warnings

# Fix automatically where possible
cargo clippy --fix
```

---

## Build & Verification

### Verifiable Build

```bash
# Build deterministically
anchor build --verifiable

# Verify the program ID matches
anchor keys list

# Update Anchor.toml with correct IDs
```

### Program Verification

```bash
# After deployment, verify on-chain
solana-verify verify-from-repo \
  -um \
  --program-id <PROGRAM_ID> \
  https://github.com/your/repo
```

---

## Deployment Process

### 1. Local Testing

```bash
# Start local validator
solana-test-validator

# Deploy locally
anchor build
anchor deploy

# Run all tests
anchor test --skip-local-validator
```

### 2. Devnet Deployment

```bash
# Switch to devnet
solana config set --url devnet

# Airdrop SOL for deployment
solana airdrop 2

# Deploy
anchor build
anchor deploy --provider.cluster devnet

# Get program ID
solana program show <PROGRAM_ID>

# Run tests against devnet
anchor test --provider.cluster devnet
```

### 3. Testnet Validation

```bash
# Deploy to testnet
solana config set --url testnet
anchor deploy --provider.cluster testnet

# Community testing period (1-2 weeks)
# Monitor for issues
# Gather feedback
```

### 4. Mainnet Preparation

Before mainnet:

- [ ] External security audit completed
- [ ] All audit findings resolved
- [ ] Bug bounty program active
- [ ] Emergency procedures documented
- [ ] Monitoring infrastructure ready
- [ ] Upgrade authority managed securely
- [ ] Team trained on response procedures

### 5. Mainnet Deployment

```bash
# Final checks
anchor build --verifiable
anchor test

# Switch to mainnet
solana config set --url mainnet-beta

# Deploy
anchor deploy --provider.cluster mainnet-beta

# Verify deployment
solana-verify verify-from-repo \
  -um \
  --program-id <PROGRAM_ID> \
  https://github.com/your/repo

# Transfer upgrade authority to multisig
solana program set-upgrade-authority \
  <PROGRAM_ID> \
  --new-upgrade-authority <MULTISIG_ADDRESS>
```

---

## Security Audits

### Recommended Audit Firms

1. **Neodyme**
   - Specializes in Solana
   - Deep runtime expertise
   - Reasonable pricing

2. **OtterSec**
   - DeFi focused
   - Fast turnaround
   - Detailed reports

3. **Sec3**
   - Automated + manual
   - Continuous monitoring
   - SDK integration

4. **Trail of Bits**
   - Comprehensive audits
   - Formal verification
   - Premium service

5. **Kudelski Security**
   - Enterprise grade
   - Extensive reporting
   - Compliance focus

### Audit Preparation

Before audit:

- [ ] Complete all internal testing
- [ ] Document all known issues
- [ ] Prepare architecture documentation
- [ ] List critical functions
- [ ] Provide threat model
- [ ] Share test environment access

### Post-Audit

After receiving report:

- [ ] Review all findings
- [ ] Prioritize by severity
- [ ] Fix critical and high issues
- [ ] Re-audit if major changes
- [ ] Publish audit report
- [ ] Update documentation

---

## Monitoring & Maintenance

### Transaction Monitoring

Monitor for:

- Failed transaction patterns (potential attacks)
- Unusual value transfers
- Repeated failures from same address
- Unexpected state changes
- CPI call failures

### Performance Monitoring

- Transaction success rate
- Compute unit usage
- Account rent status
- Error rate trends
- User adoption metrics

### Security Monitoring

- Program upgrade events
- Authority changes
- Emergency function calls
- Abnormal transaction patterns
- Cross-program interaction anomalies

### Alerting Setup

Alert on:

- Failed authorization checks
- Arithmetic overflow attempts
- Invalid PDA access
- Unusual CPI patterns
- Large value transfers
- Admin operations

---

## Emergency Procedures

### Incident Response Plan

1. **Detection**
   - Monitoring alerts
   - Community reports
   - Anomaly detection

2. **Assessment**
   - Determine severity
   - Identify affected users
   - Calculate impact

3. **Response**
   - Pause affected functions (if possible)
   - Communicate with community
   - Deploy fix if available
   - Coordinate with exchanges

4. **Recovery**
   - Deploy patched version
   - Resume operations
   - Compensate affected users
   - Post-mortem analysis

### Emergency Contacts

Maintain 24/7 contact list:

- Development team
- Security auditors
- Exchange representatives
- Key community members
- Legal counsel

### Circuit Breakers

Consider implementing:

```rust
#[account]
pub struct Config {
    pub paused: bool,
    pub emergency_admin: Pubkey,
}

pub fn pause(ctx: Context<Pause>) -> Result<()> {
    require!(
        ctx.accounts.authority.key() == emergency_admin,
        ErrorCode::Unauthorized
    );

    ctx.accounts.config.paused = true;
    Ok(())
}

// Check pause state in critical functions
require!(!config.paused, ErrorCode::Paused);
```

---

## Upgrade Strategy

### Upgrade Authority Management

**Never use a single keypair for mainnet upgrade authority!**

Options:

1. **Multisig** (recommended)
   - Requires multiple signatures
   - Squads protocol
   - Distributed keys

2. **Governance**
   - Token-based voting
   - Time-locked upgrades
   - Community control

3. **Timelock**
   - Delay between proposal and execution
   - Emergency cancellation
   - Transparent process

### Upgrade Process

1. **Preparation**
   - Test extensively on devnet
   - Audit new changes
   - Prepare rollback plan
   - Communicate with users

2. **Deployment**
   - Deploy to buffer
   - Verify build matches source
   - Execute upgrade transaction
   - Verify deployment

3. **Validation**
   - Run smoke tests
   - Monitor for issues
   - Be ready to rollback

4. **Communication**
   - Announce upgrade
   - Document changes
   - Update SDK/docs
   - Notify integrators

---

## Documentation Requirements

### User Documentation

- [ ] Setup instructions
- [ ] Usage examples
- [ ] Error handling
- [ ] Security considerations
- [ ] FAQ

### Developer Documentation

- [ ] Architecture overview
- [ ] Program structure
- [ ] Account layouts
- [ ] Instruction parameters
- [ ] Error codes
- [ ] Integration guide

### Security Documentation

- [ ] Threat model
- [ ] Security controls
- [ ] Known limitations
- [ ] Audit reports
- [ ] Bug bounty program
- [ ] Responsible disclosure policy

---

## Bug Bounty Program

### Recommended Platforms

- Immunefi (crypto-focused)
- HackerOne (general)
- Self-hosted program

### Severity Classification

**Critical** ($50K - $1M+)

- Theft of funds
- Unauthorized minting
- Program takeover

**High** ($10K - $50K)

- Protocol manipulation
- Access control bypass
- State corruption

**Medium** ($5K - $10K)

- Griefing attacks
- DoS conditions
- Information disclosure

**Low** ($1K - $5K)

- Best practice violations
- Minor logic errors
- UI issues

### Scope Definition

In scope:

- All smart contract code
- Critical business logic
- Authorization mechanisms
- Financial calculations

Out of scope:

- Frontend code
- Known issues
- Social engineering
- Physical security

---

## Compliance Considerations

### Legal Review

- [ ] Terms of service
- [ ] Privacy policy
- [ ] Jurisdiction compliance
- [ ] Token regulations
- [ ] KYC/AML if applicable

### Risk Management

- [ ] Insurance evaluation
- [ ] Liability assessment
- [ ] User protection measures
- [ ] Dispute resolution process

---

## Post-Launch

### First 48 Hours

- Monitor 24/7
- Fast response team ready
- Transaction analysis
- User feedback collection

### First Week

- Daily team check-ins
- Performance metrics
- Security monitoring
- Bug reports triage

### First Month

- Weekly security reviews
- Community feedback integration
- Performance optimization
- Documentation updates

### Ongoing

- Regular audits
- Continuous monitoring
- Community engagement
- Security education

---

## Rollback Plan

### When to Rollback

- Critical vulnerability discovered
- Funds at immediate risk
- Systemic failure
- Unrecoverable state

### Rollback Process

1. Assess situation
2. Notify stakeholders
3. Deploy previous version
4. Verify rollback success
5. Communicate status
6. Fix and re-deploy

### Prevention

- Test upgrades thoroughly
- Staged rollout
- Canary deployments
- Feature flags

---

## Final Checklist

Before going live:

- [ ] All tests passing
- [ ] Security audit complete
- [ ] Bug bounty active
- [ ] Monitoring configured
- [ ] Emergency procedures documented
- [ ] Team trained
- [ ] Documentation complete
- [ ] Community informed
- [ ] Upgrade authority secured
- [ ] Rollback plan ready

---

**Remember: Security is not a one-time task. It requires ongoing vigilance and maintenance.**

**When in doubt, delay launch. The cost of rushing is far greater than the cost of caution.**
