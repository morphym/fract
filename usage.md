whitepaper: https://pawit.co/whitepapers/fract


## Usage

Run ``cargo add fract`` to have latest version added

OR

Manually add this to your `Cargo.toml`:


```toml
[dependencies]
fract = "0.1.0"
```


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
