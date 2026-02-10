# solana-practice
Solana protocol experiments in Rust — native programs, Anchor, PDAs, and CPI.

This repository contains focused Solana protocol experiments.

The goal is to deeply understand:
- Solana account model
- PDA-based authority design
- Cross-Program Invocation (CPI)
- Native Solana programs vs Anchor abstractions

These are not demos or templates.
Each folder explores a specific protocol concept.


## Projects

### counter-contract
Native Solana program (no Anchor).
Explores account ownership, serialization, and authority checks.

### create-data-account
Manual account creation and initialization.
Focus on rent-exemption and correct account sizing.

### pda_without_anchor
Deriving and validating PDAs without Anchor helpers.

### cpi_using_anchor
Cross-program invocation using Anchor.
Focus on signer validation and invoke_signed.

### anchor-calculator
Simple Anchor program to compare native vs Anchor workflows.

## Design Philosophy

- Prefer explicit authority models
- Minimize accounts per instruction
- Treat CPI as a security boundary
- Tests matter more than UI
