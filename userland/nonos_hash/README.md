# nonos_hash

`nonos_hash` holds the hash primitives that the wallet, the signing crates and
the package installer share. `no_std`, no dependencies, no allocation. It was
split out of `nonos_hd` so a crate that needs SHA-512 does not pull in BIP39
and its word list.

## Public surface

- `sha256(data) -> [u8; 32]`: one-shot FIPS 180-4 SHA-256.
- `Sha256`: streaming SHA-256 with `new`, `update` and `finalize`, using the
  same compression function as `sha256`.
- `sha512(data) -> [u8; 64]` and `Sha512`: SHA-512, one-shot and streaming.
  `Sha512` wipes its state after `finalize` and on drop.
- `hmac_sha512(key, message) -> [u8; 64]` and `HmacSha512`: RFC 2104 HMAC
  over SHA-512, streaming. Keys longer than 128 bytes are hashed first. The
  padded key copies are wiped once used, and the outer pad on drop.
- `wipe(buf)`: zeroes a buffer with volatile writes, then a compiler fence.

## What it does not do

No SHA-1, SHA-3, BLAKE or HMAC-SHA-256 (`nonos_secp256k1` builds its own
HMAC-SHA-256 on `sha256`). `Sha256` does not wipe its state. The
implementations are plain Rust with no CPU acceleration.

## Users

- `nonos_hd` re-exports `sha256`, `sha512`, `hmac_sha512`, `HmacSha512`,
  `Sha512` and `wipe`, and builds PBKDF2 and BIP32 on them.
- `nonos_ed25519` (`sha512` in `src/signature.rs`).
- `nonos_secp256k1` (HMAC-SHA-256 for RFC 6979 nonces, on `sha256`).
- `openpgp` (`Sha256` and `Sha512` for signature digests).
- `xz` (the SHA-256 block check).
- `capsule_linux` and `capsule_linux_proofs` (package checksums in
  `src/linux/install/`).

## Tests

`tests/stream.rs` checks `sha256(b"abc")` against FIPS 180-4 and that the
streaming `Sha256` matches the one-shot at every length up to 299 bytes and
every split point. The crate is not a `*_proofs` crate, so `nix flake check`
does not run these. `nonos_hd/tests/vectors.rs` has SHA-256, SHA-512 and
RFC 4231 HMAC-SHA-512 known answers through `nonos_hd`'s re-exports. No Kani
proofs. See [Kernel crypto](../../docs/handbook/kernel/crypto.md) for the
kernel's separate implementations.
