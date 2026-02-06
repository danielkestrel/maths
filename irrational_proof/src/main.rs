/// # Proof that √n is Irrational (for non-perfect-squares)
///
/// Uses the classic proof by contradiction:
/// If √n = p/q (coprime), then p² = n·q²
/// But this leads to a contradiction in prime factorization parity.

use std::env;

/// Check if n is a perfect square
fn is_perfect_square(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    sqrt * sqrt == n
}

/// The core proof by contradiction
fn prove_irrational(n: u64) {
    println!("\n══════════════════════════════════════════════════════");
    println!("     PROOF BY CONTRADICTION: √{} IS IRRATIONAL", n);
    println!("══════════════════════════════════════════════════════\n");

    // Step 0: Check if it's actually irrational
    if is_perfect_square(n) {
        let sqrt = (n as f64).sqrt() as u64;
        println!("⚠️  √{} = {} (perfect square, so it's RATIONAL)\n", n, sqrt);
        return;
    }

    println!("ASSUME: √{} = p/q where GCD(p,q) = 1 (coprime)\n", n);
    
    println!("Step 1: Square both sides");
    println!("        {} = p²/q²", n);
    println!("        p² = {}·q²\n", n);

    println!("Step 2: Analyze prime factorization");
    println!("        • In p², every prime has an EVEN exponent");
    println!("        • In q², every prime has an EVEN exponent");
    println!("        • So {}·q² has exponents: (even) + (exponents of {})", n, n);
    
    // Find a prime factor of n with odd exponent
    let mut temp_n = n;
    let mut prime = 2u64;
    let mut odd_prime = 0u64;
    
    while temp_n > 1 {
        let mut count = 0;
        while temp_n % prime == 0 {
            temp_n /= prime;
            count += 1;
        }
        if count % 2 == 1 {
            odd_prime = prime;
            break;
        }
        prime += 1;
    }

    println!("\nStep 3: Find the contradiction");
    println!("        {} has prime factor {} with ODD exponent", n, odd_prime);
    println!("        • Left side (p²): exponent of {} is EVEN", odd_prime);
    println!("        • Right side ({}·q²): exponent of {} is ODD", n, odd_prime);
    
    println!("\n        EVEN ≠ ODD → CONTRADICTION!\n");
    
    println!("CONCLUSION: Our assumption is false.");
    println!("           √{} cannot be written as p/q", n);
    println!("           ∴ √{} is IRRATIONAL  ∎\n", n);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("\nUsage: irrational_proof <n>");
        println!("       Proves that √n is irrational\n");
        println!("Examples:");
        println!("  irrational_proof 2    # Prove √2 is irrational");
        println!("  irrational_proof 3    # Prove √3 is irrational");
        println!("  irrational_proof 7    # Prove √7 is irrational\n");
        return;
    }

    match args[1].parse::<u64>() {
        Ok(n) if n > 1 => prove_irrational(n),
        _ => println!("Please provide an integer > 1"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_squares() {
        assert!(is_perfect_square(4));
        assert!(is_perfect_square(9));
        assert!(is_perfect_square(16));
        assert!(!is_perfect_square(2));
        assert!(!is_perfect_square(3));
        assert!(!is_perfect_square(7));
    }
}
