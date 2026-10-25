# nonos_nox_receipt

`nonos_nox_receipt` decides whether an Ethereum transaction receipt proves a
NOX payment. `verify_payment` accepts a receipt only when the transaction
succeeded and its logs carry an ERC-20 `Transfer` emitted by the given token
from the buyer to the given payee for at least the price. Neither address is
in this crate: the payee comes from the signed listing and the token from
`nonos_nox_config`. NOX takes a fee on transfer, so the amount checked is the
one logged as arriving. The scan reads only the fields that decision needs and never
allocates, so a truncated or hostile receipt fails closed.

It is a `no_std` library with no capsule, service or capability word. Its one
user in this tree is `nonos_nox_broker`, and no capsule links either crate
yet. The wallet and NOX are described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Tests

11 host tests in `src/tests/` and three Kani harnesses in
`src/kani_proofs.rs`.

```sh
cd userland/nonos_nox_receipt
cargo test --release
```

No CI job names this crate.
