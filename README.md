# 🛡️ Solana Security Patterns: A Comprehensive Reference

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Anchor](https://img.shields.io/badge/Anchor-v0.30.0-blueviolet)](https://www.anchor-lang.com/)
[![Solana](https://img.shields.io/badge/Solana-v1.18-green)](https://solana.com/)

> **An educational security reference for Solana developers** - Learn by contrasting vulnerable code with secure implementations.

## 🎯 Overview

Security is the foundation of any blockchain application. This repository provides **7 critical security patterns** in Solana program development, each with:

- ❌ **Vulnerable Implementation** - Real attack vectors
- ✅ **Secure Implementation** - Proper fixes with Anchor constraints
- 📝 **Detailed Explanations** - Why it's dangerous and how to fix it
- ✅ **Exploit Tests** - Demonstrations of attacks and defenses

## 🚨 Vulnerability Patterns Covered

### 1. **Missing Signer Check** 🔐

**Risk Level:** CRITICAL  
**Real Exploit:** Wormhole Bridge ($325M)

Learn why failing to verify transaction signers allows unauthorized fund transfers and state modifications.

[📁 View Example](./programs/01-missing-signer-check/)

---

### 2. **Missing Owner Check** 👤

**Risk Level:** CRITICAL  
**Real Exploit:** Multiple DeFi protocols

Understand how missing program ownership validation enables account substitution attacks.

[📁 View Example](./programs/02-missing-owner-check/)

---

### 3. **Arithmetic Overflow/Underflow** 🔢

**Risk Level:** HIGH  
**Real Exploit:** Numerous token programs

Discover how unchecked math operations can mint unlimited tokens or drain vaults.

[📁 View Example](./programs/03-arithmetic-overflow/)

---

### 4. **Account Data Matching** 🎭

**Risk Level:** HIGH  
**Real Exploit:** Cashio ($52M)

Master the importance of validating account relationships and derived addresses.

[📁 View Example](./programs/04-account-data-matching/)

---

### 5. **Arbitrary CPI (Cross-Program Invocation)** 📞

**Risk Level:** CRITICAL  
**Real Exploit:** Various DeFi protocols

Learn to prevent attackers from invoking malicious programs through your contract.

[📁 View Example](./programs/05-arbitrary-cpi/)

---

### 6. **Re-entrancy via CPI** 🔄

**Risk Level:** HIGH  
**Real Exploit:** Adapted from Ethereum's DAO hack

Understand how external calls can create recursive attack patterns in Solana.

[📁 View Example](./programs/06-reentrance-attack/)

---

### 7. **Type Cosplay (Account Type Confusion)** 🎪

**Risk Level:** MEDIUM-HIGH  
**Real Exploit:** Solend, Jet Protocol issues

Prevent attackers from substituting accounts with wrong discriminators.

[📁 View Example](./programs/07-type-cosplay/)

---

## 🏗️ Repository Structure

```
solana-security-patterns/
│
├── programs/                          # All vulnerability examples
│   ├── 01-missing-signer-check/
│   │   ├── src/
│   │   │   ├── vulnerable.rs         # ❌ Insecure version
│   │   │   ├── secure.rs             # ✅ Fixed version
│   │   │   └── lib.rs
│   │   └── README.md                 # Pattern explanation
│   │
│   ├── 02-missing-owner-check/
│   ├── 03-arithmetic-overflow/
│   ├── 04-account-data-matching/
│   ├── 05-arbitrary-cpi/
│   ├── 06-reentrance-attack/
│   └── 07-type-cosplay/
│
├── tests/                             # Comprehensive exploit tests
│   ├── 01-missing-signer-check.ts
│   ├── 02-missing-owner-check.ts
│   └── ...
│
├── docs/
│   ├── SECURITY_GUIDE.md             # Deep-dive security guide
│   ├── ANCHOR_CONSTRAINTS.md         # Anchor constraint reference
│   └── BEST_PRACTICES.md             # Production checklist
│
├── Anchor.toml
├── Cargo.toml
├── package.json
└── README.md
```

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI (v1.18+)
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"

# Install Anchor CLI (v0.30.0+)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Install Node.js dependencies
npm install
```

### Build Programs

```bash
# Build all programs
anchor build

# Build specific program
anchor build -p missing-signer-check
```

### Run Tests

```bash
# Run all tests
anchor test

# Run specific test
anchor test tests/01-missing-signer-check.ts

# Run with detailed logs
RUST_LOG=debug anchor test
```

## 📚 Learning Path

### For Beginners

1. Start with [Missing Signer Check](./programs/01-missing-signer-check/README.md)
2. Move to [Missing Owner Check](./programs/02-missing-owner-check/README.md)
3. Read the [Security Guide](./docs/SECURITY_GUIDE.md)

### For Intermediate Developers

1. Study [Arithmetic Overflow](./programs/03-arithmetic-overflow/README.md)
2. Understand [Account Data Matching](./programs/04-account-data-matching/README.md)
3. Review [Anchor Constraints](./docs/ANCHOR_CONSTRAINTS.md)

### For Advanced Developers

1. Master [Arbitrary CPI](./programs/05-arbitrary-cpi/README.md)
2. Analyze [Re-entrancy Attacks](./programs/06-reentrance-attack/README.md)
3. Study [Type Cosplay](./programs/07-type-cosplay/README.md)

## 🎓 Deep-Dive Content

### 📖 Written Guide

[**Complete Security Guide**](./docs/SECURITY_GUIDE.md) - A comprehensive 10,000+ word guide covering:

- Solana's account model security implications
- Common attack vectors with real examples
- Anchor constraint deep-dive
- CPI security patterns
- Testing methodologies
- Production deployment checklist

### 🔍 Key Takeaways

**Always Remember:**

1. **Every account needs validation** - Never trust account inputs
2. **Signers must be verified** - Use `#[account(signer)]` or manual checks
3. **Math must be checked** - Use `checked_*` operations or Anchor's `SafeMath`
4. **PDAs need verification** - Always verify seeds and bumps
5. **CPI targets must be validated** - Never accept arbitrary program IDs
6. **State updates come last** - Follow checks-effects-interactions pattern
7. **Discriminators matter** - Type confusion is a real attack vector

## 🧪 Test Coverage

Each vulnerability includes tests demonstrating:

- ✅ **Exploit Demonstration** - How the vulnerability can be exploited
- ✅ **Failed Attack** - How the secure version prevents the attack
- ✅ **Edge Cases** - Boundary conditions and special scenarios

```bash
# Example test output
✓ Missing Signer Check - Exploit Test (vulnerable)
✓ Missing Signer Check - Protection Test (secure)
✓ Missing Owner Check - Exploit Test (vulnerable)
✓ Missing Owner Check - Protection Test (secure)
...
```

## 🏆 Real-World Impact

This repository is inspired by actual exploits:

| Vulnerability         | Real Exploit  | Amount Lost | Year      |
| --------------------- | ------------- | ----------- | --------- |
| Missing Signer Check  | Wormhole      | $325M       | 2022      |
| Account Data Matching | Cashio        | $52M        | 2022      |
| Arbitrary CPI         | Crema Finance | $8.8M       | 2022      |
| Arithmetic Issues     | Various       | $50M+       | 2021-2023 |

## 🛠️ Best Practices

### Security Checklist

Before deploying any Solana program:

- [ ] All accounts have owner checks
- [ ] All authorities have signer checks
- [ ] All math operations use `checked_*` methods
- [ ] All PDAs are properly derived and verified
- [ ] All CPI targets are validated
- [ ] State updates follow checks-effects-interactions
- [ ] Account discriminators are checked
- [ ] Tests cover both exploit and defense scenarios
- [ ] Code has been audited by multiple developers
- [ ] Security tools have been run (soteria, sec3, etc.)

See [Best Practices Guide](./docs/BEST_PRACTICES.md) for details.

## 📖 Additional Resources

### Official Documentation

- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Program Library](https://spl.solana.com/)

### Security Resources

- [Neodyme Security Audits](https://neodyme.io/)
- [Sec3 Security Suite](https://www.sec3.dev/)
- [Soteria Static Analyzer](https://github.com/igneous-labs/soteria)
- [Sealevel Attacks](https://github.com/coral-xyz/sealevel-attacks)

### Learning Resources

- [Ackee Blockchain School](https://ackeeblockchain.com/)
- [Solana Security Workshop](https://www.youtube.com/watch?v=XqTEwo7JNjQ)
- [Paulx's Security Guide](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/)

## 🤝 Contributing

This repository is open source and welcomes contributions:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-vulnerability`)
3. Add your vulnerability example with tests
4. Update documentation
5. Submit a pull request

### Contribution Guidelines

- Each vulnerability must have vulnerable + secure versions
- Include comprehensive inline comments
- Add tests demonstrating the exploit
- Update the main README
- Follow Rust and Anchor best practices

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Coral-xyz** for Anchor framework
- **Neodyme** for security research
- **Solana Foundation** for comprehensive documentation
- **SuperteamNG** for organizing this educational bounty
- The Solana security community for sharing vulnerability research

## 👥 Team

Built with ❤️ by security-focused Solana developers passionate about making the ecosystem safer.

## ⚠️ Disclaimer

**This repository contains intentionally vulnerable code for educational purposes.**

- ⚠️ **NEVER** deploy the vulnerable versions to mainnet
- ⚠️ **NEVER** use vulnerable patterns in production
- ⚠️ All examples are for learning only
- ⚠️ Always get professional security audits before mainnet deployment

---

**Star ⭐ this repository if you find it helpful!**

**Report issues or suggest improvements via GitHub Issues.**

---

_Last Updated: January 2026_
_Anchor Version: 0.30.0_
_Solana Version: 1.18.0_
