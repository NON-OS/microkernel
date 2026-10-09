# wallet_proofs

Host test crate for the NONOS wallet and the parts of the keyring that hold
its key. It compiles the real files of `capsule_wallet_nonos`,
`capsule_keyring` and `nonos_route_link` through `#[path]`, unchanged, and
checks them with unit tests and Kani harnesses. Its only dependency is
`nonos_policy_proto` (`Cargo.toml`). The wallet is described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## What is under test

- From the wallet (`src/wallet/`, `src/nox/`): amount parsing, formatting and
  scaling, the stakeable amount, the swap curve and its limits, the NOX read
  helpers, the route text the status line shows and the routed reader that
  reads an answer over a mixnet.
- From the keyring: the key store (`src/store/`, all of it but the one file
  that reads the kernel clock) and the vault gate rule
  (`server/vault_gate/rule.rs`).
- From `nonos_route_link`: `pick.rs` and `describe.rs`, the route rule the
  wallet's requests follow.

## What the tests check

77 `#[test]` functions and five Kani harnesses (`src/kani_proofs.rs`):

- amount parsing and typed formatting, `mul_div`, scaling, the stakeable
  amount, the swap curve and its limits;
- the status line names the route the requests took, and the routed reader
  reads an answer that arrives late and in pieces whole, refuses an oversized
  one, and keeps what came before a broken stream (`route_tests.rs`);
- each owner keeps its share of the keyring's 128 places, and the keys of an
  owner that ended are dropped while a living owner's are not
  (`keyring_owner_tests.rs`);
- only an owner of `app.nonos_wallet`, `app.nonos_wallet.1` or
  `app.nonos_wallet.2` may seal or open the vault record, and never sender 0
  (`vault_gate_tests.rs`).

## Running

```sh
cd userland/wallet_proofs
cargo test --release
cargo kani
```

CI runs the tests and clippy through `nix flake check`
(`tools/nix/checks.nix`), and the Kani harnesses in the `proof-crates-kani`
job of `.github/workflows/verify.yml`.

## Not covered

The window, the TLS client, the IPC calls to the keyring and vfs, and the
keyring's signing handlers are not mounted here.
