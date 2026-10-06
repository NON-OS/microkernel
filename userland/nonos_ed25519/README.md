# nonos_ed25519

Ed25519 in userland. The Nym client signs its gateway handshake, derives its
identity key and verifies the directory's signatures with it, and each of
those used to be a syscall. The arithmetic is the kernel's, moved rather than
rewritten. What stays in the kernel is verification for the boot chain, the
check that runs before a capsule loads.

It is a `no_std` library with no capsule, service or capability word. Its
users are `capsule_net_nym`, `capsule_net_anon`, `capsule_market` (the
catalogue's operator and publisher signatures), `capsule_model_fetch` (the
model catalogue's signature), `capsule_linux`, `openpgp`, and the proof crates
`crypto_proofs` and `model_fetch_proofs`.

## What is in it

`field`, `scalar` and `point` for the curve; `sign`, `verify`, `KeyPair` and
`Signature` in `signature.rs`; `pubkey_from_secret`. `KeyPair::generate` was
not ported: the seed is the caller's to supply, so the library has no ambient
access to randomness. The market's use is described in
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

## Tests

6 host tests in `src/tests.rs`.

```sh
cd userland/nonos_ed25519
cargo test --release
```

CI runs them through `nix flake check`: `tools/nix/checks.nix` lists this
crate beside the `*_proofs` crates, with clippy over all targets.
