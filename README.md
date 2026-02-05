# Confidential Transaction Demo (Pedersen Commitments)

Rust **educational demo** (not production crypto) that shows how confidential transactions work using Pedersen commitments. Toy integers and modular arithmetic only—no crypto libs, no elliptic curves.

## What it demonstrates

- **Pedersen commitment**: `C = v*G + r*H` — a commitment that hides value `v` using random blinding `r`.
- **Homomorphic property**: Sum of commitments = commitment to sum (with combined blinding).
- **Transaction**: Alice spends an input of 10, sends 5 to Bob, keeps 5 as change.
- **Verification**: The network checks `C_input = C_bob + C_change` using only commitments — no amounts are revealed.

## Run it

```bash
cargo run
```

## Requirements

- Rust toolchain. No external crates; uses toy integers and modular arithmetic only.

## Constraints (by design)

- No cryptographic libraries.
- No elliptic curve math — G and H are fixed integers, math is mod a prime.
- For intuition and clarity only; do not use in production.
