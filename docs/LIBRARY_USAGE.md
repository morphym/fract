# Using Fract as a Library

The `fract` crate provides a minimal, high-performance cryptographic hash function based on hyperchaotic dynamical systems. This guide covers; to integrate it into your Rust projects.

## Installation

Add `fract` to your `Cargo.toml`:

```toml
[dependencies]
fract = { path = "path/to/fract" }
# or if published to crates.io
fract = "0.1.0"
```

## Basic Usage

### Single-Shot Hashing

For simple use cases where you have all data available at once:

```rust
use fract::Fract;

fn main() {
    let data = b"hello world";

    // 256-bit hash (32 bytes)
    let hash_256 = Fract::hash(data);
    println!("256-bit: {:02x?}", hash_256);

    // 512-bit hash for enhanced quantum resistance (64 bytes)
    let hash_512 = Fract::hash512(data);
    println!("512-bit: {:02x?}", hash_512);
}
```

Output:
```
256-bit: [36, b1, ea, 9a, 0b, a3, a4, d3, 64, 58, e6, 23, 2e, d5, 76, 19, 4f, 9a, 34, c1, 79, 66, 6a, f2, e2, 5a, 4b, fd, 5a, 8d, 3c, 1f]
512-bit: [08, c9, bc, f3, 67, e6, 09, 6a, 3b, a7, ca, 84, 85, ae, 67, bb, db, ff, e0, f5, d4, c7, 9c, bd, 52, 03, 71, d4, de, 23, 9e, 5c, 91, a7, 48, 8b, fb, 2a, f5, 28, cb, 47, 56, 2e, dd, 32, 3d, 52, 0b, 89, dd, b3, 00, 0a, 82, 36, 69, 32, c5, 3a, fd, 3a, a5, 6e]
```

### Hex String Output

For easier display and comparison:

```rust
use fract::{hash_to_hex, hash512_to_hex};

fn main() {
    let data = b"hello world";

    // Get hash as hex string
    let hex_256 = hash_to_hex(data);
    println!("256-bit hex: {}", hex_256);
    // Output: 36b1ea9a0ba3a4d36458e6232ed576194f9a34c179666af2e25a4bfd5a8d3c1f

    let hex_512 = hash512_to_hex(data);
    println!("512-bit hex: {}", hex_512);
    // Output: 08c9bcf367e6096a3ba7ca8485ae67bbdbffe0f5d4c79cbd520371d4de239e5c91a7488bfb2af528cb47562edd323d520b89ddb3000a82366932c53afd3aa56e
}
```

## Incremental Hashing

For streaming data or large files that don't fit in memory:

```rust
use fract::Fract;
use std::fs::File;
use std::io::{self, Read};

fn hash_reader<R: Read>(mut reader: R) -> io::Result<[u8; 32]> {
    let mut hasher = Fract::new();
    let mut buffer = vec![0; 8192]; // 8KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

fn main() -> io::Result<()> {
    // Hash a file incrementally
    let file = File::open("/path/to/large/file")?;
    let hash = hash_reader(file)?;
    println!("File hash: {}", hex::encode(hash));

    Ok(())
}
```

### Chunked Updates

You can update the hasher with data as it becomes available:

```rust
use fract::Fract;

fn main() {
    let mut hasher = Fract::new();

    // Process data in chunks
    hasher.update(b"The quick brown ");
    hasher.update(b"fox jumps over ");
    hasher.update(b"the lazy dog");

    let hash = hasher.finalize();
    println!("Hash: {}", hex::encode(hash));

    // Result is identical to hashing all at once
    let single_pass = Fract::hash(b"The quick brown fox jumps over the lazy dog");
    assert_eq!(hash, single_pass);
}
```

## API Reference

### `Fract`

The main hasher type that implements the sponge construction.

#### Methods

**`new() -> Self`**
Creates a new hasher instance with the initialization vector.

```rust
let hasher = Fract::new();
```

**`update(&mut self, data: &[u8])`**
Absorbs more data into the hash state. Can be called multiple times.

```rust
hasher.update(b"first chunk");
hasher.update(b"second chunk");
```

**`finalize(self) -> [u8; 32]`**
Finalizes the hash and returns the 256-bit (32-byte) digest. Consumes the hasher.

```rust
let hash = hasher.finalize();
```

**`hash(data: &[u8]) -> [u8; 32]`**
One-shot hashing function for 256-bit output. Convenience method that creates a hasher, updates it with data, and finalizes it.

```rust
let hash = Fract::hash(b"data");
```

**`hash512(data: &[u8]) -> [u8; 64]`**
One-shot hashing function for 512-bit output. Provides enhanced quantum resistance.

```rust
let hash = Fract::hash512(b"data");
```

### Convenience Functions

**`hash_to_hex(data: &[u8]) -> String`**
Hashes data and returns a hex-encoded string (256-bit).

```rust
let hex_hash = hash_to_hex(b"data");
// Returns: "e2d31388016cc397e98528c734ca8f7c..."
```

**`hash512_to_hex(data: &[u8]) -> String`**
Hashes data and returns a hex-encoded string (512-bit).

```rust
let hex_hash = hash512_to_hex(b"data");
// Returns: "08c9bcf367e6096a3ba7ca8485ae67bb..."
```

## Advanced Usage

### Custom Initialization

You can create multiple independent hashers:

```rust
use fract::Fract;

fn main() {
    // Different contexts
    let mut hasher1 = Fract::new();
    let mut hasher2 = Fract::new();

    // Hash different data
    hasher1.update(b"context1");
    hasher2.update(b"context2");

    let hash1 = hasher1.finalize();
    let hash2 = hasher2.finalize();

    // Results are independent
    assert_ne!(hash1, hash2);
}
```

### Streaming from Network

```rust
use fract::Fract;
use std::net::TcpStream;

fn hash_from_stream(mut stream: TcpStream) -> io::Result<[u8; 32]> {
    let mut hasher = Fract::new();
    let mut buffer = vec![0; 4096];

    loop {
        let bytes = stream.read(&mut buffer)?;
        if bytes == 0 {
            break;
        }
        hasher.update(&buffer[..bytes]);
    }

    Ok(hasher.finalize())
}
```

### Hashing with Prefix/Suffix

```rust
use fract::Fract;

fn hash_with_prefix_suffix(data: &[u8], prefix: &[u8], suffix: &[u8]) -> [u8; 32] {
    let mut hasher = Fract::new();
    hasher.update(prefix);
    hasher.update(data);
    hasher.update(suffix);
    hasher.finalize()
}
```

## Performance Characteristics

### Memory Usage

- **State size**: 256 bits (32 bytes)
- **Buffer size**: 128 bits (16 bytes) for rate portion
- **Total memory**: ~48 bytes per hasher instance
- **Lookup tables**: None (zero memory overhead)

### Speed

Measured on a typical x86_64 system:

- **Small inputs (≤ 16 bytes)**: ~50-100 ns
- **1 KiB**: ~14 μs
- **1 MiB**: ~15 ms (67-75 MiB/s throughput)
- **Cycles/byte**: ~38-42 cycles (estimated at 3GHz)

These are reference measurements; actual performance depends on CPU architecture, compiler optimizations, and data patterns.

## Security Considerations

### Claims (per whitepaper)
:
- **Classical preimage resistance**: 2²⁵⁶
- **Classical collision resistance**: 2¹²⁸ (birthday bound on 128-bit capacity)
- **Quantum preimage resistance**: 2²⁵⁶ (with 512-bit output)
- **Design principle**: Hyperchaotic dynamics with positive Lyapunov exponents

### Important Warnings

⚠️ **Experimental Cryptography**

This is a novel design that has NOT undergone:
- Third-party cryptanalysis
- Peer review by cryptographic community
- Standardization process (NIST, IETF, etc.)
- Long-term security evaluation

Use only for:
- Research and education
- Non-critical applications
- Situations where hyperchaotic properties are specifically desired

**DO NOT use** for:
- Password hashing (use argon2, scrypt, or bcrypt)
- Key derivation (use HKDF or similar)
- Digital signatures
- Production security systems

### Implementations Note

The reference implementation uses:
- Fixed-point arithmetic only (no floating-point)
- `wrapping_*` operations for deterministic cross-platform behavior
- No dynamic memory allocation in hot path
- Constant-time operations to resist timing attacks

## Examples

See the `examples/` directory for complete examples:

- `demo.rs` - Comprehensive demonstration of features
- Add your own: `cargo run --example <name>`

## Testing

Run the built-in test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

## Benchmarking

The `fract` binary includes built-in benchmarks:

```bash
# Benchmark 1MB data, 100 iterations
fract bench

# Custom size and iterations
fract bench --size 1048576 --iter 1000

# Test 512-bit mode
fract bench --512

# Test incremental (chunked) hashing
fract bench --chunked

# Combine options
fract bench --size 16384 --iter 1000 --512 --chunked
```

Example output:
```
=== Fract Benchmark ===
Data size: 1048576 bytes
Iterations: 100
Mode: 256-bit
Method: single-pass

Total time: 147.55311ms
Throughput: 67.77 MiB/s
Last hash: 60e1a1235112e7d3

=== Additional Stats ===
Bytes processed: 10485760
Nanoseconds per byte: 14.07
Cycles/byte (est. at 3GHz): 42.22
```

## Integration Examples

### With Standard Types

```rust
use fract::Fract;

// Hash a String
let s = String::from("hello world");
let hash = Fract::hash(s.as_bytes());

// Hash a Vec<u8>
let vec = vec![1, 2, 3, 4, 5];
let hash = Fract::hash(&vec);

// Hash an array
let arr = [1u8; 32];
let hash = Fract::hash(&arr);
```

### In Structs

```rust
use fract::Fract;

struct Document {
    content: Vec<u8>,
    hash: [u8; 32],
}

impl Document {
    fn new(content: Vec<u8>) -> Self {
        let hash = Fract::hash(&content);
        Self { content, hash }
    }

    fn verify(&self) -> bool {
        let computed = Fract::hash(&self.content);
        computed == self.hash
    }
}
```

### Error Handling

```rust
use fract::Fract;
use std::fs::File;
use std::io::{self, Read};

fn hash_file_safe(path: &str) -> Result<[u8; 32], io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Fract::new();
    let mut buffer = vec![0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}
```

## Troubleshooting

### Determinism Issues

If you need identical hashes across different platforms:

- Ensure you're using the same Fract version
- Use little-endian byte order (default)
- Avoid mixing 256-bit and 512-bit modes
- Use the same chunk sizes for incremental hashing

The implementation uses `wrapping_*` operations which are deterministic across all platforms that Rust supports.

### Performance Issues

If hashing is slower than expected:

1. Enable compiler optimizations in release mode
   ```bash
   cargo build --release
   ```

2. Check that you're not in a debug build

3. For maximum performance, use the convenience functions (`Fract::hash()`) or single-pass hashing rather than many small updates

4. Consider increasing buffer sizes when reading from files or network

### Memory Issues

The hasher uses only ~48 bytes of stack space. If you're seeing high memory usage:

- Check that you're not holding onto large buffers unnecessarily
- Use streaming/chunked hashing for large files
- Each hasher is independent - create new instances rather than reusing

## Contributing

When contributing to the library:

1. Maintain the minimalist design principle
2. Ensure cross-platform determinism
3. Add tests for any new functionality
4. Update benchmarks if performance changes
5. Document any security implications

## License

MIT License - See LICENSE file for details

## References

- Original whitepaper: `fract.md`
- Sponge construction: [Keccak/SHA-3 Standard](https://keccak.team/files/Keccak-reference-3.0.pdf)
- Chaos theory in cryptography: [Chaos-based Cryptography Survey](https://arxiv.org/abs/2008.04141)

For questions or issues, see the GitHub repository at: https://github.com/morphym/fract
