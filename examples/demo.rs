/// Demo example for Fract
///
/// Usage: cargo run --example demo
///
/// This example demonstrates the usage of the Fract hash function
/// with various test cases.
use fract::{hash512_to_hex, hash_to_hex, Fract};

fn main() {
    println!("=== Fract-256 ===\n");

    // Test 1: Empty string
    println!("1. Hash of empty string:");
    let hash = hash_to_hex(b"");
    println!("   {}", hash);
    println!();

    // Test 2: "hello world"
    println!("2. Hash of 'hello world':");
    let hash = hash_to_hex(b"hello world");
    println!("   {}", hash);
    println!();

    // Test 3: 512-bit output
    println!("3. 512-bit hash of 'hello world' (enhanced quantum resistance):");
    let hash = hash512_to_hex(b"hello world");
    println!("   {}", hash);
    println!();

    // Test 4: Large data
    println!("4. Hash of 1MB of 'A's:");
    let data = vec![b'A'; 1024 * 1024];
    let hash = hash_to_hex(&data);
    println!("   {}", hash);
    println!();

    // Test 5: Avalanche effect demonstration
    println!("5. Avalanche effect demonstration:");
    let msg1 = "The quick brown fox jumps over the lazy dog";
    let msg2 = "The quick brown fox jumps over the lazy dof"; // Changed last char

    let hash1 = hash_to_hex(msg1.as_bytes());
    let hash2 = hash_to_hex(msg2.as_bytes());

    println!("   Original: {}", msg1);
    println!("   Modified: {}", msg2);
    println!("   Hash 1: {}", hash1);
    println!("   Hash 2: {}", hash2);

    // Count differing bits
    let bytes1 = Fract::hash(msg1.as_bytes());
    let bytes2 = Fract::hash(msg2.as_bytes());

    let diff_bits = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum::<u32>();

    println!(
        "   Bits differing: {} out of 256 ({:.1}%)",
        diff_bits,
        (diff_bits as f64 / 256.0) * 100.0
    );
    println!();

    // Test 6: Chunked hashing
    println!("6. Chunked hashing (same as single-pass):");
    let msg =
        b"The ChaosFiber-256 hash function demonstrates natural diffusion via topological mixing";

    // Single pass
    let hash1 = hash_to_hex(msg);

    // Chunked
    let mut hasher = Fract::new();
    hasher.update(&msg[0..23]);
    hasher.update(&msg[23..45]);
    hasher.update(&msg[45..]);
    let hash2 = hex::encode(hasher.finalize());

    println!("   Single-pass: {}", hash1);
    println!("   Chunked:     {}", hash2);
    println!("   Identical:   {}", hash1 == hash2);
    println!();

    // Performance note
    println!("=== Performance Characteristics ===");
    println!("- State size: 256 bits (4 × u64)");
    println!("- Rate: 128 bits (2 × u64)");
    println!("- Rounds: 8 permutations per absorb/squeeze");
    println!("- Operations: Only 8 arithmetic ops per round");
    println!("- Memory: Zero lookup tables");
    println!("- Target: ~4 cycles/byte on commodity hardware");
    println!();
    println!("=== Security Claims ===");
    println!("- Classical preimage resistance: 2^256");
    println!("- Classical collision resistance: 2^128");
    println!("- Quantum preimage resistance: 2^256 (with 512-bit output)");
    println!("- Design principle: Hyperchaos with positive Lyapunov exponents");
}
