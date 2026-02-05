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

/// Toy "range proof": only checks that value is non-negative.
/// In a real system you would use a zero-knowledge range proof (e.g. Bulletproofs)
/// that proves 0 <= v < 2^n for a commitment C = v*G + r*H WITHOUT revealing v or r.
/// Here we fake it by just checking v >= 0 (the prover would have to reveal v to us).
fn range_proof(value: i64) -> bool {
    value >= 0
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
    println!("  Alice's input commitment: C_input = {}*G + {}*H", value_input, r_input);
    println!("  C_input = {}", c_input);
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

    println!("  Bob's output:   value = {}, blinding = {}", value_to_bob, r_bob);
    println!("  C_bob   = {}*G + {}*H = {}", value_to_bob, r_bob, c_bob);
    println!("  Change: value = {}, blinding = {}", value_change, r_change);
    println!("  C_change = {}*G + {}*H = {}", value_change, r_change, c_change);
    println!("  (Again, the actual amounts 5 and 5 are never revealed.)\n");

    // ---------------------------------------------------------------------------
    // STEP 3: What gets published (only commitments)
    // ---------------------------------------------------------------------------
    println!("--- Step 3: What is published on the ledger ---");
    println!("  The network only sees these three numbers:");
    println!("    C_input  = {}", c_input);
    println!("    C_bob    = {}", c_bob);
    println!("    C_change = {}", c_change);
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
    // STEP 4b: Range proofs (toy: fake range_proof only checks v >= 0)
    // ---------------------------------------------------------------------------
    // In real systems: Bulletproofs (or similar) prove 0 <= v < 2^n for a commitment
    // without revealing v. See comment block below for where Bulletproofs fit in.
    println!("--- Step 4b: Range proofs (toy: check each value >= 0) ---");
    let rp_input = range_proof(value_input);
    let rp_bob = range_proof(value_to_bob);
    let rp_change = range_proof(value_change);
    println!("  range_proof(input=10)  => {} (valid)", rp_input);
    println!("  range_proof(bob=5)    => {} (valid)", rp_bob);
    println!("  range_proof(change=5) => {} (valid)", rp_change);
    println!("  All range proofs pass. (In reality, prover never reveals the values.)\n");

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

    println!("  C_input (10)  = {}", c_input_attack);
    println!("  C_bob (15)    = {}", c_bob_attack);
    println!("  C_change (-5) = {}", c_change_attack);
    println!("  C_input ?= C_bob + C_change  =>  {}", attack_verification_passes);
    println!("\n  Commitment verification PASSES even though 5 units were created from thin air!\n");

    println!("--- Rejecting the attack with a range proof ---");
    let rp_change_attack = range_proof(value_change_attack);
    println!("  range_proof(change=-5) => {} (INVALID)", rp_change_attack);
    println!("  The malicious transaction would be REJECTED because change is negative.\n");

    println!("--- Why a range proof is required ---");
    println!("  Pedersen commitments only prove sum(inputs) = sum(outputs).");
    println!("  They do NOT prove that each value is non-negative or bounded.");
    println!("  Without range proofs, anyone could use negative \"change\" to inflate the supply.");
    println!("  A range proof proves (without revealing the amount) that a committed value v");
    println!("  lies in a valid range, e.g. 0 <= v < 2^64. Then negative or huge values are rejected.\n");

    // ---------------------------------------------------------------------------
    // Where Bulletproofs fit in real systems
    // ---------------------------------------------------------------------------
    println!("--- Where Bulletproofs would fit in real systems ---");
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
