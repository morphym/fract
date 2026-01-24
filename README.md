<img width="5464" height="3072" alt="image" src="https://github.com/user-attachments/assets/282e2891-7a09-4fd8-86d6-7bcba3b704d7" /># FRACT-256

A Hyperchaotic, Quantum-Resistant, Minimalist Cryptographic Hash implementation in Rust.

## Overview

FRACT is a cryptographic hash function that leverages hyperchaotic dynamical systems on finite modular lattices to achieve provable diffusion, natural quantum resistance, and exceptional performance. By eschewing traditional S-boxes and large constant arrays in favor of coupled chaotic maps with positive Lyapunov exponents, the design achieves cryptographically secure avalanche effects through deterministic chaos.

## Features

- **Minimal Design**: Only 8 arithmetic operations per round, zero lookup tables
- **High Performance**: Targeting ~4 cycles/byte on standard hardware
- **Quantum Resistant**: Non-algebraic structure resists quantum algorithms
- **Sponge Construction**: 256-bit state with 128-bit rate and capacity
- **Hybrid Logistic-Tent Map**: Chaotic primitive on ℤ₂₆₄
- **Hyperchaotic Lattice**: Four coupled chaotic maps for enhanced diffusion
- *Deterministic*: All operations are fixed-point integer arithmetic and rust wrapping arithmetic
                   enforced hash stay determinisitc accross all machines.

## Foundation


READ WHITEPAPER -> https://www.pawit.co/whitepapers/fract-whitepaper.pdf

*:*: license is ``creative commons attribution 4.0 international``
<br> </br>
Author: Pawit Sahare ( @morphym ).


## Metrics

<img width="1077" height="388" alt="image" src="https://github.com/user-attachments/assets/24918d0a-a666-41c0-922e-07eeccc9b114" />


### >

Install binary

```cargo install fract```

Then, Enjoy a, Fast. Minimal. Hyperchaotic, Quantum-Resistant, Hash.

```bash
princee@princee:~$ fract

    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║    ░██████╗░█████╗░░█████╗░██╗░░██╗░█████╗░████████╗░█████╗░  ║
    ║    ██╔════╝██╔══██╗██╔══██╗██║░░██║██╔══██╗╚══██╔══╝██╔══██╗  ║
    ║    █████╗░░███████║██║░░╚═╝███████║███████║░░░██║░░░██║░░╚═╝  ║
    ║    ██╔══╝░░██╔══██║██║░░██╗██╔══██║██╔══██║░░░██║░░░██║░░██╗  ║
    ║    ██║░░░░░██║░░██║╚█████╔╝██║░░██║██║░░██║░░░██║░░░╚█████╔╝  ║
    ║    ╚═╝░░░░░╚═╝░░╚═╝░╚════╝░╚═╝░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░░╚════╝░  ║
    ║                                                              ║
    ║    Hyperchaotic · Quantum-Resistant · Fast Cryptographic Hash ║
    ║                                                              ║
    ║    Author: @morphym                                          ║
    ║    Version: 0.1.0                                            ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝

Usage: fract [OPTIONS] [FILE]...
       fract bench [OPTIONS]

Run 'fract --help' for detailed usage information.
princee@princee:~$ fract cat 
c3405751cd163e953f04744da9eb4bd411930a2b3de066c3c9e2ca905b33aa99  cat
princee@princee:~$ 

```

#### Benchmark

To run local benchmark in your machine

```fract bench```


Result: At a ARM 4vCPU aprx 3GHZ result are very good.


https://github.com/user-attachments/assets/327b1417-5fc3-4ba8-8e94-c1f436267648





```bash
ubuntu@ip-172-31-21-181:/tmp$ fract bench
=== Fract Benchmark ===
Data size: 1048576 bytes
Iterations: 100
Mode: 256-bit
Method: single-pass

Total time: 612.685244ms
Throughput: 163.22 MiB/s
Last hash: 60e1a1235112e7d3

=== Additional Stats ===
Bytes processed: 104857600
Nanoseconds per byte: 5.84
Cycles/byte (est. at 3GHz): 17.53
ubuntu@ip-172-31-21-181:/tmp$ 

```

Result: On a 4 2.25GHZ vCPU machine: 

```bash
princee@princee:~$ fract bench
=== Fract Benchmark ===
Data size: 1048576 bytes
Iterations: 100
Mode: 256-bit
Method: single-pass

Total time: 1.439969279s
Throughput: 69.45 MiB/s
Last hash: 60e1a1235112e7d3

=== Additional Stats ===
Bytes processed: 104857600
Nanoseconds per byte: 13.73
Cycles/byte (est. at 3GHz): 41.20
princee@princee:~$ 

```



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

Read: https://github.com/morphym/fract/blob/master/usage.md

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

**NOTE**: This is an experimental implementation of a novel cryptographic design. The security claims in the whitepaper have not yet been independently verified through third-party cryptanalysis.

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

### Citation
```
Pawit, S. (2025). FRACT- A Hyperchaotic, Quantum Resistant, Fast Cryptographic Hash.
  Pawit Sahare. 
https://doi.org/10.5281/zenodo.17983496

https://pawit.co/works/fract
.
```

