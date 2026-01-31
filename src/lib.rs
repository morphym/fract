//! FRACT-256: A Hyperchaotic, Quantum-Resistant, Minimalist Cryptographic Hash
//!
//! This implementation follows the specification in the whitepaper for FRACT,
//! a cryptographic hash function that leverages hyperchaotic dynamical systems
//! on finite modular lattices.

/// Rate in bytes: 128 bits (2 × u64)
const RATE: usize = 16;

/// Number of permutation rounds
const ROUNDS: usize = 8;

/// Initialization Vector (first 256 bits of √2)
const IV: [u64; 4] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
];

/// FRACT-256 hasher state
#[derive(Clone, Debug)]
pub struct Fract {
    /// Internal state vector (4 × u64)
    pub state: [u64; 4], // state public for zk-disorder
    /// Buffer for absorbing data
    buffer: [u8; RATE],
    /// Number of bytes currently in buffer
    buffer_len: usize,
    /// Total number of bytes absorbed
    total_len: usize,
    /// Whether the final block has been processed
    finalized: bool,
}

impl Fract {
    /// Creates a new Fract-256 hasher
    pub fn new() -> Self {
        Self {
            state: IV,
            buffer: [0; RATE],
            buffer_len: 0,
            total_len: 0,
            finalized: false,
        }
    }

    /// Absorbs data into the state
    pub fn update(&mut self, data: &[u8]) {
        assert!(!self.finalized, "Cannot update after finalization");

        let mut bytes = data;
        self.total_len += bytes.len();

        // Fill buffer if not empty
        if self.buffer_len > 0 {
            let capacity = RATE - self.buffer_len;
            let take = bytes.len().min(capacity);

            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&bytes[..take]);
            self.buffer_len += take;
            bytes = &bytes[take..];

            // If buffer is full, absorb it
            if self.buffer_len == RATE {
                self.absorb_block();
                self.buffer_len = 0;
            }
        }

        // Process full blocks
        while bytes.len() >= RATE {
            self.buffer[..RATE].copy_from_slice(&bytes[..RATE]);
            self.absorb_block();
            bytes = &bytes[RATE..];
        }

        // Store remaining bytes
        if !bytes.is_empty() {
            self.buffer[..bytes.len()].copy_from_slice(bytes);
            self.buffer_len = bytes.len();
        }
    }

    pub fn from_state(state: [u64; 4]) -> Self {
        // This is only for zk-disorder.
        Self {
            state,
            buffer: [0; RATE],
            buffer_len: 0,
            total_len: 0,
            finalized: false,
        }
    }

    /// Finalizes and returns the hash (256-bit output)
    pub fn finalize(mut self) -> [u8; 32] {
        if !self.finalized {
            self.pad_and_absorb();
            self.finalized = true;
        }

        self.squeeze_256()
    }

    /// Convenience method: hash data in one shot (256-bit output)
    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }

    /// Convenience method: hash data in one shot (512-bit output for enhanced quantum resistance)
    pub fn hash512(data: &[u8]) -> [u8; 64] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.squeeze_512()
    }

    /// Absorbs the current buffer block
    fn absorb_block(&mut self) {
        // XOR block into rate portion of state
        self.state[0] ^= u64::from_le_bytes([
            self.buffer[0],
            self.buffer[1],
            self.buffer[2],
            self.buffer[3],
            self.buffer[4],
            self.buffer[5],
            self.buffer[6],
            self.buffer[7],
        ]);
        self.state[1] ^= u64::from_le_bytes([
            self.buffer[8],
            self.buffer[9],
            self.buffer[10],
            self.buffer[11],
            self.buffer[12],
            self.buffer[13],
            self.buffer[14],
            self.buffer[15],
        ]);

        // Apply permutation
        self.permute();
    }

    /// Applies padding and absorbs final block
    fn pad_and_absorb(&mut self) {
        // 10*1 padding on the rate portion
        self.buffer[self.buffer_len] = 0x01;
        for i in self.buffer_len + 1..RATE {
            self.buffer[i] = 0x00;
        }
        self.buffer[RATE - 1] = 0x80;

        self.absorb_block();
    }

    /// Applies the permutation (8 rounds of Φ)
    fn permute(&mut self) {
        for _ in 0..ROUNDS {
            self.apply_phi();
        }
    }

    /// Applies one round of the hyperchaotic lattice transformation Φ
    #[inline(always)]
    pub fn apply_phi(&mut self) {
        // make thi sstate public for zk-disorder.
        let [s0, s1, s2, s3] = self.state;

        // Hybrid Logistic-Tent Map f(x) on Z_2^64
        let f0 = hltm(s0);
        let f1 = hltm(s1);
        let f2 = hltm(s2);
        let f3 = hltm(s3);

        // Coupled hyperchaotic lattice Φ with wrapping operations for cross-platform determinism
        self.state = [
            f0.wrapping_add((s1 >> 31) ^ (s3 << 17)),
            f1.wrapping_add((s2 >> 23) ^ (s0 << 11)),
            f2.wrapping_add((s3 >> 47) ^ (s1 << 29)),
            f3.wrapping_add((s0 >> 13) ^ (s2 << 5)),
        ];
    }

    /// Squeezes 256 bits from the state
    fn squeeze_256(&mut self) -> [u8; 32] {
        let mut output = [0u8; 32];

        // First squeeze: output rate portion
        output[0..8].copy_from_slice(&self.state[0].to_le_bytes());
        output[8..16].copy_from_slice(&self.state[1].to_le_bytes());

        // Apply permutation and squeeze again
        self.permute();
        output[16..24].copy_from_slice(&self.state[0].to_le_bytes());
        output[24..32].copy_from_slice(&self.state[1].to_le_bytes());

        output
    }

    /// Squeezes 512 bits from the state (enhanced quantum resistance)
    fn squeeze_512(&mut self) -> [u8; 64] {
        let mut output = [0u8; 64];

        // First squeeze: output rate portion
        output[0..8].copy_from_slice(&self.state[0].to_le_bytes());
        output[8..16].copy_from_slice(&self.state[1].to_le_bytes());

        // Apply permutation and squeeze again
        self.permute();
        output[16..24].copy_from_slice(&self.state[0].to_le_bytes());
        output[24..32].copy_from_slice(&self.state[1].to_le_bytes());

        // Apply permutation and squeeze again
        self.permute();
        output[32..40].copy_from_slice(&self.state[0].to_le_bytes());
        output[40..48].copy_from_slice(&self.state[1].to_le_bytes());

        // Apply permutation and final squeeze
        self.permute();
        output[48..56].copy_from_slice(&self.state[0].to_le_bytes());
        output[56..64].copy_from_slice(&self.state[1].to_le_bytes());

        output
    }
}

impl Default for Fract {
    fn default() -> Self {
        Self::new()
    }
}

/// Hybrid Logistic-Tent Map  on Z_2^64
/// f(x) = { 4x(1-x) mod 2^64 if x < 2^63
///        { 4(2^64 - x)(x - 2^63) mod 2^64 if x >= 2^63
#[inline(always)]
fn hltm(x: u64) -> u64 {
    if x < (1u64 << 63) {
        // Logistic map variant: 4x(1-x) mod 2^64
        let x_mod = x as u128 * 4;
        let x_sq_mod = ((x as u128 * x as u128) >> 64) * 4;
        (x_mod - x_sq_mod) as u64
    } else {
        // Tent map variant: 4(2^64 - x)(x - 2^63) mod 2^64
        let x_prime = x ^ (1u64 << 63); // x - 2^63
        let x_complement = (!x).wrapping_add(1); // 2^64 - x

        let product = (x_prime as u128) * (x_complement as u128) * 4;
        product as u64
    }
}

// /// Hash data and return 256-bit digest in hexadecimal format
// pub fn hash_to_hex(data: &[u8]) -> String {
//     let hash = Fract::hash(data);
//     hex::encode(hash)
// }

// /// Hash data and return 512-bit digest in hexadecimal format
// pub fn hash512_to_hex(data: &[u8]) -> String {
//     let hash = Fract::hash512(data);
//     hex::encode(hash)
// } -> redudant for this branch.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hltm_basic() {
        // Test HLTM with some known values
        let x1 = 0x0123456789abcdef;
        let _ = hltm(x1); // Just ensure it doesn't panic

        let x2 = 0xfedcba9876543210;
        let _ = hltm(x2); // Just ensure it doesn't panic
    }

    #[test]
    fn test_empty_hash() {
        let hash = Fract::hash(b"");
        // println!("Empty hash: {}", hex::encode(hash));
        assert_eq!(hash.len(), 32);
        // Note: We don't have test vectors yet since this is a new design
    }

    #[test]
    fn test_hello_world() {
        let hash = Fract::hash(b"hello cat");
        // println!("'hello cat' hash: {}", hex::encode(hash));
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hello_world_chunked() {
        let mut hasher = Fract::new();
        hasher.update(b"hello ");
        hasher.update(b"cat");
        let hash = hasher.finalize();

        let expected = Fract::hash(b"hello cat");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_large_data() {
        let data = vec![0x61; 10000]; // 10KB of 'a'
        let hash = Fract::hash(&data);
        // println!("Large data hash: {}", hex::encode(hash));
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash512() {
        let hash = Fract::hash512(b"hello world");
        // println!("'hello world' hash512: {}", hex::encode(hash));
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_avalanche() {
        // Test avalanche effect: small change should produce completely different hash
        let data1 = b"The quick brown fox jumps over the lazy dog";
        let data2 = b"The quick brown fox jumps over the lazy dof"; // Changed last char

        let hash1 = Fract::hash(data1);
        let hash2 = Fract::hash(data2);

        // Check that hashes are different (very high probability they should be completely different)
        assert_ne!(hash1, hash2);

        // Count differing bits
        let diff_bits = hash1
            .iter()
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>();

        println!("Avalanche effect: {} bits differ out of 256", diff_bits);
        // Should be close to 128 bits (50% difference) for good avalanche
        assert!(
            diff_bits > 100,
            "Poor avalanche effect: only {} bits differ",
            diff_bits
        );
    }

    // #[test]
    // fn test_convenience_functions() {
    //     // let hex256 = hash_to_hex(b"test");
    //     assert_eq!(hex256.len(), 64); // 256 bits = 64 hex chars

    //     let hex512 = hash512_to_hex(b"test");
    //     assert_eq!(hex512.len(), 128); // 512 bits = 128 hex chars
    // }
}
