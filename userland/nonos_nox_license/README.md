# nonos_nox_license

Pay-per-use entitlements for NONOS tool capsules, settled in NOX. A tool would
run only with an entitlement the broker signed, and the broker signs only
against a confirmed, unspent NOX payment. This crate holds what both sides
must agree on:

- `price.rs`: the tool catalogue. One tool, `ToolId::Recon` (wire id 1,
  `recon`), at 0.25 NOX a use, priced in base units of 1e18 per NOX.
  `price_of` answers `None` for an unknown id.
- `entitlement.rs`: the record, fixed width and little-endian. Magic `NXL1`,
  tool id, buyer address, device binding, uses, issue time, expiry, the
  funding transaction hash and a nonce, 120 bytes signed by a detached 64-byte
  Ed25519 signature.
- `verify.rs`: `check` parses a record and verifies its signature through an
  injected `Verify`, so the crate stays pure and testable on the host.

It is a `no_std` library with no capsule, service or capability word. Its one
user is `nonos_nox_broker`, and no capsule links either crate yet. The wallet
and NOX are described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

## Limits

`TREASURY` is a placeholder: the bytes `NONOS` followed by zeros and a final
1, not the treasury address in `abi/nox_deployment.json`.

## Tests

14 host tests in `src/tests.rs`, which sign with `ed25519-dalek` as a
dev-dependency, and three Kani harnesses in `src/kani_proofs.rs`.

```sh
cd userland/nonos_nox_license
cargo test --release
```

No CI job names this crate.
