# nonos_secp256k1

secp256k1 in userland. The curve served only the wallet, yet it sat in ring 0
behind the `CryptoSecp256k1Sign` and `CryptoSecp256k1Pubkey` syscalls. It now
runs in `capsule_keyring`'s process, the same arithmetic moved rather than
rewritten. It is a `no_std` library with no capsule, service or capability
word; the keyring is its one user.

## What is in it

- `field`, `scalar`, `point`: field and scalar arithmetic and the curve group.
- `sign`: ECDSA with the nonce from RFC 6979 (`rfc6979.rs`, `hmac_drbg.rs`),
  normalised to low s.
- `verify`, `recover_public_key`, `public_key_from_secret`.

It carries no RNG and does not derive Ethereum addresses: a signing library
has no ambient randomness, and what a public key means is the caller's
business. The keyring's wrapper returns `r || s || v` with `v` biased by 27.
How the wallet signs is described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Tests

12 host tests in `src/tests.rs`.

```sh
cd userland/nonos_secp256k1
cargo test --release
```

CI runs them through `nix flake check`: `tools/nix/checks.nix` lists this
crate beside the `*_proofs` crates, with clippy over all targets.
