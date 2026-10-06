# nonos_aes

`nonos_aes` is a software AES-128 with two counter modes. `no_std`, no
dependencies. Only the forward cipher exists, since counter mode never
decrypts a block.

## Public surface

- `Aes128::new(key)` and `Aes128::encrypt_block(&mut block)`. Dropping an
  `Aes128` zeroes its round keys with volatile writes.
- `Ctr128Be::new(key)`: the counter is the whole 128-bit block, big endian,
  starting at zero. This is what a Tor relay runs.
- `Ctr64Be::new(key, iv)`: the counter is the low 64 bits of `iv`, big endian,
  wrapping within them. The high half stays fixed. This is the Sphinx layout.
- Both have `apply(&mut data)`, which XORs the keystream in and carries on
  from the exact byte where the previous call stopped, so data does not need
  to be block aligned.
- `BLOCK_BYTES` and `KEY_BYTES`, both 16.

The S-box is computed, not looked up: each byte is inverted in GF(2^8) as
x^254 with masked shift-and-add multiplies, then put through the FIPS 197
affine map (`src/sub_byte.rs`). The goal is no table index or branch on key or
state bytes.

## What it does not do

No AES-192 or AES-256, no inverse cipher, no authenticated mode. It does not
track nonces: reusing a key and counter is the caller's mistake to avoid.

## Users

- `capsule_net_anon` uses `Ctr128Be` for each hop's forward and backward
  keystream (`src/circuit/hop.rs`).
- `anon_ntor_proofs` re-exports it through its crypto shim.
- `capsule_net_nym` does not use this crate. It has its own AES with a
  `Ctr64Be` in `src/crypto/aes/`, so `nonos_aes::Ctr64Be` has no caller outside
  `aes_proofs`.

## Tests

No tests in the crate. `aes_proofs` checks it against FIPS 197 and its CTR
modes against SP 800-38A, and compiles `sub_byte.rs` and `xtime.rs` by
`#[path]` to compare the S-box with a table. `nix flake check` runs that as
`proofs-aes_proofs`. There are no Kani proofs. The relay cell encryption that uses it is described
in [Anyone](../../docs/handbook/network/anyone.md). The kernel's own AES is a
separate implementation, in [Kernel crypto](../../docs/handbook/kernel/crypto.md).
