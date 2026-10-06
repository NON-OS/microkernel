# nonos_nox_broker

The license broker's decision, without the capsule around it. `issue` prices
the tool through `nonos_nox_license`, checks with `nonos_nox_receipt` that the
receipt pays the treasury at least that price in NOX, refuses a funding
transaction already redeemed, and returns the grant the buyer earned, one use
per price paid. It never signs: the capsule that would hold the Ed25519 key
signs the result.

Replay is blocked by a `SpentSet`, a ring of `SPENT_CAPACITY` (4096) funding
hashes the caller is expected to persist. When it fills, the oldest hash is
overwritten, so a transaction 4096 payments old could be redeemed again;
`record` reports the eviction. `issue` records a hash only on success, so a
failed attempt never burns a payment and a repeat never mints a second grant.

It is a `no_std` library with no capsule, service or capability word, and no
capsule links it yet: no broker capsule exists in this tree. The wallet and
NOX are described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Tests

9 host tests in `src/tests.rs` and three Kani harnesses in
`src/kani_proofs.rs`.

```sh
cd userland/nonos_nox_broker
cargo test --release
```

No CI job names this crate.
