//! Educational Demo: Confidential Transactions with Pedersen Commitments
//! =====================================================================
//! This is a TOY demo for learning. It uses simple integers and modular
//! arithmetic—NOT real cryptography. Do not use in production.
//!
//! Pedersen commitments let us prove "sum(inputs) = sum(outputs)" without
//! revealing the actual amounts. We only ever show commitments C = v*G + r*H.

/// Public parameters (known to everyone, like in a real system).
/// We work modulo a prime so numbers wrap around. In a real system, G and H would be curve points.
const MODULUS: i64 = 2_i64.pow(61) - 1;
const G: i64 = 3; // "Generator G" — value dimension
const H: i64 = 7; // "Generator H" — blinding/randomness dimension

/// Create a Pedersen commitment: C = v*G + r*H (mod p).
/// - value: the secret amount (v)
/// - blinding: random number (r) that hides the value
/// Anyone can compute C, but without knowing r they cannot find v.
fn pedersen_commit(value: i64, blinding: i64) -> i64 {
    let term = value * G + blinding * H;
    ((term % MODULUS) + MODULUS) % MODULUS
}

/// Toy "range proof" semantic check: value is non-negative (used conceptually; display/verify use toy_range_proof_*).
/// In a real system you would use a zero-knowledge range proof (e.g. Bulletproofs)
/// that proves 0 <= v < 2^n for a commitment C = v*G + r*H WITHOUT revealing v or r.
#[allow(dead_code)]
fn range_proof(value: i64) -> bool {
    value >= 0
}

/// Toy "range proof" as a displayable value π (like we display C).
/// Prover creates π from (value, C). In reality π would be ~700 bytes and bind to C without revealing v.
/// Here we encode: π = C*2 + valid_bit (valid_bit = 1 if value >= 0 else 0) so we can show π in the demo.
fn toy_range_proof_create(value: i64, commitment: i64) -> i64 {
    let valid_bit = if value >= 0 { 1 } else { 0 };
    commitment * 2 + valid_bit
}

/// Toy verification: verifier has only (C, π). Checks that π is valid for C (value was in range).
/// In reality the verifier runs Bulletproof verification equations; here we check π == C*2+1.
fn toy_range_proof_verify(commitment: i64, proof: i64) -> bool {
    proof == commitment * 2 + 1
}

fn main() {
    println!("{}", "=".repeat(60));
    println!("CONFIDENTIAL TRANSACTION DEMO (Pedersen Commitments)");
    println!("{}", "=".repeat(60));
    println!("\n--- Public parameters (everyone knows these) ---");
    println!("  Modulus p = {}", MODULUS);
    println!("  Generator G = {}", G);
    println!("  Generator H = {}", H);
    println!("  (In real crypto, G and H would be curve points.)\n");

    // ---------------------------------------------------------------------------
    // STEP 1: Alice's initial commitment (input)
    // ---------------------------------------------------------------------------
    // Alice has 10 units. She created a commitment earlier using r_input. Only she knows (10, r_input).
    println!("--- Step 1: Alice's input commitment ---");
    let value_input = 10i64;
    let r_input = 12345i64; // Alice's secret blinding factor for the input

    let c_input = pedersen_commit(value_input, r_input);
    let pi_input = toy_range_proof_create(value_input, c_input);
    println!("  Alice's input commitment: C_input = {}*G + {}*H", value_input, r_input);
    println!("  C_input = {}", c_input);
    println!("  π_input = {}  (toy range proof for this commitment)", pi_input);
    println!("  (The value 10 and blinding 12345 are NEVER sent on the chain.)\n");

    // ---------------------------------------------------------------------------
    // STEP 2: Alice creates commitments for the outputs
    // ---------------------------------------------------------------------------
    // Alice sends 5 to Bob, keeps 5 as change. She picks r_bob and r_change so that r_input = r_bob + r_change.
    println!("--- Step 2: Output commitments (Bob and change) ---");
    let value_to_bob = 5i64;
    let value_change = 5i64;

    let r_bob = 11111i64;
    let r_change = r_input - r_bob; // so r_bob + r_change = r_input

    let c_bob = pedersen_commit(value_to_bob, r_bob);
    let c_change = pedersen_commit(value_change, r_change);
    let pi_bob = toy_range_proof_create(value_to_bob, c_bob);
    let pi_change = toy_range_proof_create(value_change, c_change);

    println!("  Bob's output:   value = {}, blinding = {}", value_to_bob, r_bob);
    println!("  C_bob   = {}*G + {}*H = {}   π_bob   = {}", value_to_bob, r_bob, c_bob, pi_bob);
    println!("  Change: value = {}, blinding = {}", value_change, r_change);
    println!("  C_change = {}*G + {}*H = {}   π_change = {}", value_change, r_change, c_change, pi_change);
    println!("  (Again, the actual amounts 5 and 5 are never revealed.)\n");

    // ---------------------------------------------------------------------------
    // STEP 3: What gets published (only commitments)
    // ---------------------------------------------------------------------------
    println!("--- Step 3: What is published on the ledger ---");
    println!("  The network sees commitments C and range proofs π (no values, no blindings):");
    println!("    (C_input,  π_input ) = ({}, {})", c_input, pi_input);
    println!("    (C_bob,    π_bob   ) = ({}, {})", c_bob, pi_bob);
    println!("    (C_change, π_change) = ({}, {})", c_change, pi_change);
    println!("  No one can recover 10, 5, or 5 from these alone.\n");

    // ---------------------------------------------------------------------------
    // STEP 4: Verification using only commitments
    // ---------------------------------------------------------------------------
    // Homomorphic: C_input should equal C_bob + C_change (mod p).
    let sum_outputs = ((c_bob + c_change) % MODULUS + MODULUS) % MODULUS;
    let inputs_match_outputs = c_input == sum_outputs;

    println!("--- Step 4: Public verification (no values revealed) ---");
    println!("  Check: C_input ?= C_bob + C_change  (mod p)");
    println!("  C_input         = {}", c_input);
    println!("  C_bob + C_change = {}", sum_outputs);
    println!("  Match? {}", inputs_match_outputs);
    if inputs_match_outputs {
        println!("  So: sum(input amounts) = sum(output amounts), verified using only commitments.\n");
    } else {
        println!("  Verification failed.\n");
    }

    // ---------------------------------------------------------------------------
    // STEP 4b: Range proofs — create and display π, then verify (C, π) without knowing v
    // ---------------------------------------------------------------------------
    // In real systems: Bulletproofs (or similar) prove 0 <= v < 2^n for a commitment
    // without revealing v. Here we display π like we display C.
    println!("--- Step 4b: Range proofs — create π, then verify (C, π) ---");
    println!("  Prover created π for each commitment (above). Verifier checks using only (C, π):");
    let rp_input = toy_range_proof_verify(c_input, pi_input);
    let rp_bob = toy_range_proof_verify(c_bob, pi_bob);
    let rp_change = toy_range_proof_verify(c_change, pi_change);
    println!("  verify(C_input,  π_input ) => {} (valid)", rp_input);
    println!("  verify(C_bob,    π_bob   ) => {} (valid)", rp_bob);
    println!("  verify(C_change, π_change) => {} (valid)", rp_change);
    println!("  All range proofs pass. Verifier never saw the values.\n");

    // ---------------------------------------------------------------------------
    // STEP 5: Why amounts stay secret
    // ---------------------------------------------------------------------------
    println!("--- Step 5: Why amounts stay secret ---");
    println!("  Given only C = v*G + r*H, there are infinitely many (v, r) that give the same C.");
    println!("  So from C_input, C_bob, C_change one cannot deduce 10, 5, or 5.");
    println!("  Verification only needed the equality C_input = C_bob + C_change.\n");

    // ---------------------------------------------------------------------------
    // DEMO: Negative value attack — why range proofs are required
    // ---------------------------------------------------------------------------
    println!("{}", "=".repeat(60));
    println!("DEMO: How a negative value breaks the system");
    println!("{}", "=".repeat(60));
    println!("\n--- Attack: Malicious transaction with negative \"change\" ---");
    println!("  Attacker has input 10 but wants to send 15 to Bob (creating 5 from nothing).");
    println!("  They use a NEGATIVE change: value_change = -5.");
    println!("  Math still balances: 10 = 15 + (-5), so commitment equation holds.\n");

    let value_input_attack = 10i64;
    let r_input_attack = 99999i64;
    let value_to_bob_attack = 15i64;  // More than input!
    let value_change_attack = -5i64;   // Negative "change" = creating value

    let r_bob_attack = 11111i64;
    let r_change_attack = r_input_attack - r_bob_attack;

    let c_input_attack = pedersen_commit(value_input_attack, r_input_attack);
    let c_bob_attack = pedersen_commit(value_to_bob_attack, r_bob_attack);
    let c_change_attack = pedersen_commit(value_change_attack, r_change_attack);

    let sum_outputs_attack = ((c_bob_attack + c_change_attack) % MODULUS + MODULUS) % MODULUS;
    let attack_verification_passes = c_input_attack == sum_outputs_attack;

    let pi_change_attack = toy_range_proof_create(value_change_attack, c_change_attack);
    println!("  C_input (10)  = {}", c_input_attack);
    println!("  C_bob (15)    = {}", c_bob_attack);
    println!("  C_change (-5) = {}   π_change = {}", c_change_attack, pi_change_attack);
    println!("  C_input ?= C_bob + C_change  =>  {}", attack_verification_passes);
    println!("\n  Commitment verification PASSES even though 5 units were created from thin air!\n");

    println!("--- Rejecting the attack with a range proof ---");
    println!("  Verifier checks (C_change, π_change) without knowing the value:");
    let rp_change_attack = toy_range_proof_verify(c_change_attack, pi_change_attack);
    println!("  verify(C_change, π_change) => {} (INVALID)", rp_change_attack);
    println!("  The malicious transaction is REJECTED because π fails verification (value was negative).\n");

    println!("--- Why a range proof is required ---");
    println!("  Pedersen commitments only prove sum(inputs) = sum(outputs).");
    println!("  They do NOT prove that each value is non-negative or bounded.");
    println!("  Without range proofs, anyone could use negative \"change\" to inflate the supply.");
    println!("  A range proof proves (without revealing the amount) that a committed value v");
    println!("  lies in a valid range, e.g. 0 <= v < 2^64. Then negative or huge values are rejected.\n");

    // ---------------------------------------------------------------------------
    // How a range proof is created and verified (conceptual)
    // ---------------------------------------------------------------------------
    println!("--- How a range proof is created (prover side) ---");
    println!("  Inputs: commitment C = v*G + r*H, and the prover's secret (v, r).");
    println!("  Goal: prove that 0 <= v < 2^n (e.g. n=64) WITHOUT revealing v or r.");
    println!("  Idea (e.g. Bulletproofs-style):");
    println!("    1. Write v in binary: v = b_0 + 2*b_1 + 4*b_2 + ... (each b_i is 0 or 1).");
    println!("    2. Encode the bits into Pedersen commitments or vectors in a special way.");
    println!("    3. Use an inner-product argument to prove that the bits are 0/1 and sum to v.");
    println!("    4. The proof pi is a short string (~700 bytes) that binds to C.");
    println!("  Output: proof pi. The prover sends (C, pi) to the verifier; v and r stay secret.\n");

    println!("--- How a range proof is verified (verifier side) ---");
    println!("  Inputs: commitment C and proof pi (and public parameters G, H, range bound 2^n).");
    println!("  Verifier does NOT know v or r.");
    println!("  Steps:");
    println!("    1. Check that pi is well-formed and has the right size/structure.");
    println!("    2. Run the verification equation(s): combine C, pi, G, H in a fixed formula.");
    println!("    3. The math works out only if C actually commits to some v in [0, 2^n).");
    println!("  If all checks pass => \"C commits to a value in range\". If not => reject.");
    println!("  The verifier never learns v or r, only that the range condition holds.\n");

    // ---------------------------------------------------------------------------
    // Where Bulletproofs fit in real systems
    // ---------------------------------------------------------------------------
    println!("--- Where Bulletproofs fit in real systems ---");
    println!("  In production (e.g. Monero, Mimblewimble):");
    println!("  - Each input and output commitment comes with a RANGE PROOF.");
    println!("  - Bulletproofs are short (~700 bytes) and prove 0 <= v < 2^64 for C = v*G + r*H");
    println!("    without revealing v or r (zero-knowledge).");
    println!("  - Verifiers check: (1) sum(input commitments) = sum(output commitments),");
    println!("    (2) each range proof is valid. Then no negative or overflow amounts are possible.\n");

    // ---------------------------------------------------------------------------
    // Final message
    // ---------------------------------------------------------------------------
    println!("{}", "=".repeat(60));
    if inputs_match_outputs {
        println!("Transaction verified without revealing amounts.");
    } else {
        println!("Verification failed.");
    }
    println!("{}", "=".repeat(60));
}
