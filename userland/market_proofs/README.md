# market_proofs

Host test crate (package `nonos_market_proofs`) for the market capsule. It
compiles the capsule's readiness gate (`install_ready/checks.rs`,
`install_ready/arch.rs`), its release lookup (`find_release.rs`) and its
request header decode (`protocol/`) through `#[path]`, unchanged, and links
`marketplace_abi` with its canonical encoder. The market is described in
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

## What the tests check

13 `#[test]` functions in four modules:

- `readiness_tests.rs`: a `linux.` distribution package is ready without
  shipping a proof, but only on a machine that hosts one; naming the hosted
  arch does not exempt a capsule from its proof; a capsule that ships a proof
  is ready; no index signature means no readiness.
- `release_tests.rs`: an empty release id is the first release, a named one is
  that release, and an unknown listing or release is none.
- `index_decode_tests.rs`: blobs from the encoder decode back, and damaged or
  boundary blobs from a fixed-seed generator decode within the caps or not at
  all.
- `request_tests.rs`: arbitrary request headers and bodies from a fixed-seed
  generator either decode or are refused.

## Running

```sh
cd userland/market_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`), with overflow
checks on and clippy over all targets.

## Not covered

Signature verification, `load_verified`, the boot index read and the reply
encoders are not mounted here.
