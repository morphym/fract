# FRACT-256

A Hyperchaotic, Quantum-Resistant, Minimalist Cryptographic Hash implementation in Rust.

## Overview

FRACT is a cryptographic hash function that leverages hyperchaotic dynamical systems on finite modular lattices to achieve provable diffusion, natural quantum resistance, and exceptional performance. By eschewing traditional S-boxes and large constant arrays in favor of coupled chaotic maps with positive Lyapunov exponents, the design achieves cryptographically secure avalanche effects through deterministic chaos.

## Features

- **Minimal Design**: Only 8 arithmetic operations per round, zero lookup tables
- **High Performance**: Targeting ~4 cycles/byte on commodity hardware
- **Quantum Resistant**: Non-algebraic structure resists quantum algorithms
- **Sponge Construction**: 256-bit state with 128-bit rate and capacity
- **Hybrid Logistic-Tent Map**: Chaotic primitive on ℤ₂₆₄
- **Hyperchaotic Lattice**: Four coupled chaotic maps for enhanced diffusion

## Foundation


READ WHITEPAPER -> https://www.pawit.co/whitepapers/fract-whitepaper.pdf


## Metrics

<img width="1077" height="388" alt="image" src="https://github.com/user-attachments/assets/24918d0a-a666-41c0-922e-07eeccc9b114" />


## Core

These are core mathematical foundation; not all are stated here; read whitepaper for comprehensive mathematical specification.

## Hybrid Logistic-Tent Map (HLTM)

The core chaotic primitive is defined on ℤ₂₆₄:

```rust
f(x) = { 4x(1 - x) mod 2^64          if x < 2^63
       { 4(2^64 - x)(x - 2^63) mod 2^64  if x ≥ 2^63
```

This exhibits a Lyapunov exponent λ ≈ 0.693, guaranteeing exponential divergence.

### Coupled Hyperchaotic Lattice Φ

For state S = (s₀, s₁, s₂, s₃) ∈ (ℤ₂₆₄)⁴:

```rust
Φ(S) = {
  s₀' = f(s₀) ⊕ (s₁ ≫ 31) ⊕ (s₃ ≪ 17)
  s₁' = f(s₁) ⊕ (s₂ ≫ 23) ⊕ (s₀ ≪ 11)
  s₂' = f(s₂) ⊕ (s₃ ≫ 47) ⊕ (s₁ ≪ 29)
  s₃' = f(s₃) ⊕ (s₀ ≫ 13) ⊕ (s₂ ≪ 5)
}
```

All operations use modular arithmetic with constant-time behavior.

note: whitepaper contain more information on all mathematical impl.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fract = { path = "path/to/fract" }
```

### Usage

```rust
use fract::{Fract, hash_to_hex};

// Single-shot hashing (256-bit output)
let hash = Fract::hash(b"hello cat");
println!("Hash: {:02x?}", hash);

// Or use the convenience function
let hex_hash = hash_to_hex(b"hello cat");
println!("Hex: {}", hex_hash);

// 512-bit output for enhanced quantum resistance
let hash512 = Fract::hash512(b"hello cat");
println!("512-bit: {:02x?}", hash512);
```

### Incremental Hashing

```rust
use fract::Fract;

let mut hasher = Fract::new();

// Update with multiple chunks
hasher.update(b"chunk1");
hasher.update(b"chunk2");
hasher.update(b"chunk3");

// Finalize and get the hash
let hash = hasher.finalize();
println!("Hash: {:02x?}", hash);
```

## API Reference

### `Fract`

The main hasher struct that implements the sponge construction.

#### Methods

- `new() -> Self` - Creates a new hasher instance
- `update(&mut self, data: &[u8])` - Absorbs data into the state
- `finalize(self) -> [u8; 32]` - Finalizes and returns 256-bit hash
- `hash(data: &[u8]) -> [u8; 32]` - One-shot hashing (256-bit)
- `hash512(data: &[u8]) -> [u8; 64]` - One-shot hashing (512-bit)

#### Convenience Functions

- `hash_to_hex(data: &[u8]) -> String` - Returns 256-bit hash as hex string
- `hash512_to_hex(data: &[u8]) -> String` - Returns 512-bit hash as hex string

## Security Considerations

**WARNING**: This is an experimental implementation of a novel cryptographic design. The security claims in the whitepaper have not been independently verified through third-party cryptanalysis.

### Claims

- **Classical Preimage Resistance**: 2²⁵⁶
- **Classical Collision Resistance**: 2¹²⁸ (birthday bound on 128-bit capacity)
- **Quantum Preimage Resistance**: 2²⁵⁶ (with 512-bit output)

### Future Works.

1. No third-party cryptanalysis has *yet* been performed
2. The aggressive round count (R=8) may need increase for conservative deployments
3. Algebraic attacks using modular arithmetic decomposition have not *yet* been thoroughly analyzed

## Implementation information.

- **Language**: Pure Rust, `#![no_std]` compatible
- **Constants**: Only 4 IV words (256 bits of √2)
- **Memory**: Zero lookup tables, entirely ALU-bound
- **Timing**: Constant-time operations using `wrapping_*` intrinsics
- **Dependencies**: Only `hex` crate for hex encoding functions

## Performance

Target performance characteristics:

- **Throughput**: ~4 cycles/byte
- **Latency**: 48 cycles for 16-byte input
- **Code Size**: <1 KB
- **Vectorization**: Four u64 lanes enable SIMD execution

## Testing

Run the test suite:

```bash
cargo test
```

Run the demo:

```bash
cargo run --example demo
```

## References

- Whitepaper: `fract.pdf` - Comprehensive mathematical specification
- Based on chaos theory and hyperchaotic dynamical systems
- Sponge construction as described in the Keccak/SHA-3 standard

## License

MIT License

## Author

@morphym- Morphy Moretti {Pawit Sahare}.

## Disclaimer

This software is provided "as is", without warranty of any kind. Use at your own risk for security-sensitive applications. Always consult with a cryptographer before using novel cryptographic primitives in production.
