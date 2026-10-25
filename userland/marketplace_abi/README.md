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
  Schema 3 adds `Commerce` to an entry (payee, bond id, unit, unit price,
  minimum charge, `SettleMode`, proof requirements, the payee's binding
  signature) and `ReleaseExt` to a release (commerce hash, guest policy,
  chunk root, content length, proof anchors), both per plan/23.
- `codec/`: `decode_index` reads schemas 2 and 3. It refuses a blob over
  `MAX_INDEX_BLOB` (2 MiB) before parsing, an unknown price kind, settle mode
  or proof kind, a guest policy that is not 0 or 256 bytes, an escrow listing
  with an unpinned release, and any byte after the signature.
  `release_signing_bytes` gives the bytes a publisher signs: domain
  `NONOS.marketplace.release.v3` for a release with schema 3 fields, v2 for
  one without. `commerce_hash_input` is what a release's commerce hash is the
  BLAKE3 of. The `canonical-encode` feature adds `encode_index` and
  `encode_and_sign`, which write schema 3 only and refuse anything the
  decoder would.
- `ids/`: `listing_id`, `release_id`, `bond_id` and `release_hash`, the
  on-chain ids of plan/23, with the tags from `nonos_nox_config`.
- `verdict/`: `commerce_verdict` (did the release sign the listed commerce
  block) and `binding_verdict` (did the payee sign the PublisherBinding under
  the bond contract's domain).
- The crate holds no hash: `ids` and `verdict` take keccak, BLAKE3 and
  secp256k1 recovery through the `Keccak`, `Blake3` and `Recover` traits, so
  it is the same bytes on the host and in a capsule.
- `limits.rs`: at most 1024 entries, 64 releases each, and fixed bounds on
  every string, arch list and capability list.

## Users

`capsule_market` decodes and verifies the catalogue with it. `market_proofs`
uses the encoder to build test blobs. `nonos-mk` links it with the encoder for
the host tool that builds and signs the catalogue.

## Tests

The crate has no tests of its own. `userland/market_proofs` drives
`decode_index` with encoded, damaged and boundary blobs, and `abi_v3/` there
covers schema 3: round trips, every refusal, the shipped schema 2 catalogue,
the signing domains, the ids against the plan's formulas, and a real
secp256k1 binding signature against an EIP-712 digest built by hand.
