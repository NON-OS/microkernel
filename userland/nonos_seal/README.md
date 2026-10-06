# nonos_seal

ChaCha20-Poly1305 (RFC 8439), from scratch and `no_std`. `seal` encrypts a
plaintext under a 32-byte key, a 12-byte nonce and associated data and appends
the 16-byte tag. `open` checks the tag in constant time before it decrypts and
answers `Err(SealError::AuthFailed)` on any mismatch, so a tampered or
wrong-key blob never decrypts. `SealState` keeps a monotonic counter for
callers that seal many records under one key, so a nonce cannot repeat by
accident.

It is a library with no capsule, service or capability word. `nonos_vault`
builds the machine-sealed records on it (the wallet's vault blob among them),
`nonos_wifi_client` seals remembered Wi-Fi networks with it (and
`capsule_settings_proofs` tests that sealing), and `capsule_wallet_nonos` uses
its length constants. The vault path is described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Tests

7 host tests in `src/tests/mod.rs`.

```sh
cd userland/nonos_seal
cargo test --release
```

No CI job names this crate directly.
