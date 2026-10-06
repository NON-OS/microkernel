# nonos_nox_receipt

`nonos_nox_receipt` decides whether an Ethereum transaction receipt proves a
NOX payment. `verify_payment` accepts a receipt only when the transaction
succeeded and its logs carry an ERC-20 `Transfer` emitted by the NOX token
(`NOX_TOKEN`, `0x0a26c80b...9eca`) from the buyer to the treasury for at least
the price. The scan reads only the fields that decision needs and never
allocates, so a truncated or hostile receipt fails closed.

It is a `no_std` library with no capsule, service or capability word. Its one
user in this tree is `nonos_nox_broker`, and no capsule links either crate
yet. The wallet and NOX are described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Tests

10 host tests in `src/tests.rs` and three Kani harnesses in
`src/kani_proofs.rs`.

```sh
cd userland/nonos_nox_receipt
cargo test --release
```

No CI job names this crate.
