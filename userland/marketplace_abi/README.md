# marketplace_abi

`nonos_marketplace_abi` (directory `marketplace_abi`) is the wire form of the
signed marketplace catalogue: the types, the limits and a length-prefixed
binary codec. It is `no_std` with `unsafe` forbidden, and has no capsule,
service or capability word of its own. Field shape and order follow
`abi/marketplace_index.schema.json`. The market and the catalogue format are
described in
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

## What is in it

- `types/`: `MarketplaceIndex` (schema version, operator id and key, publish
  time, serial, entries, index signature), `MarketplaceEntry`,
  `CapsuleRelease`, prices, tokens, the operator's `ValidationReport`, and
  `InstallReadiness`, the verdict and six gates the market answers with.
- `codec/`: `decode_index`, which accepts schema 2 only and refuses a blob
  over `MAX_INDEX_BLOB` (2 MiB) before parsing, and `release_signing_bytes`,
  the bytes a publisher signs, under the domain `NONOS.marketplace.release.v2`.
  The `canonical-encode` feature adds `encode_index` and `encode_and_sign`.
- `limits.rs`: at most 1024 entries, 64 releases each, and fixed bounds on
  every string, arch list and capability list.

## Users

`capsule_market` decodes and verifies the catalogue with it. `market_proofs`
uses the encoder to build test blobs. `nonos-mk` links it with the encoder for
the host tool that builds and signs the catalogue.

## Tests

The crate has no tests of its own. `userland/market_proofs` drives
`decode_index` with encoded, damaged and boundary blobs.
