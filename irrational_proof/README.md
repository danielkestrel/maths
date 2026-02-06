# 🔢 Irrational Proof

Proves that √n is irrational for any non-perfect-square using **proof by contradiction**.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
irrational_proof 2    # Prove √2 is irrational
irrational_proof 7    # Prove √7 is irrational
irrational_proof 12   # Prove √12 is irrational
irrational_proof 9    # Shows √9 = 3 is rational (perfect square)
```

## The Proof

For any non-perfect-square n:

1. **Assume** √n = p/q where p, q are coprime
2. **Square**: p² = n·q²
3. **Key**: n has some prime with an **odd** exponent
4. **Left side** (p²): all primes have **even** exponents
5. **Right side** (n·q²): that prime has **odd** exponent
6. **EVEN ≠ ODD** → Contradiction! ∎

## Example Output

```
══════════════════════════════════════════════════════
     PROOF BY CONTRADICTION: √2 IS IRRATIONAL
══════════════════════════════════════════════════════

ASSUME: √2 = p/q where GCD(p,q) = 1 (coprime)

Step 1: Square both sides
        2 = p²/q²
        p² = 2·q²

Step 2: Analyze prime factorization
        • In p², every prime has an EVEN exponent
        • In q², every prime has an EVEN exponent
        • So 2·q² has exponents: (even) + (exponents of 2)

Step 3: Find the contradiction
        2 has prime factor 2 with ODD exponent
        • Left side (p²): exponent of 2 is EVEN
        • Right side (2·q²): exponent of 2 is ODD

        EVEN ≠ ODD → CONTRADICTION!

CONCLUSION: Our assumption is false.
           √2 cannot be written as p/q
           ∴ √2 is IRRATIONAL  ∎
```

## License

MIT
