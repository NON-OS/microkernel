# The development market catalogue

`index.json` is a schema 3 marketplace catalogue for development images
only. It lets a dev image's Marketplace list priced listings, so the paid
paths (meter vouchers, an escrow order) can be walked by hand. The release
catalogue, `nonos-data/market/index.json`, is schema 2 and lists only free
Qwen tiers.

## What is in it

Four listings, all `devtest.*`:

- `devtest.free`: free, settle mode `free`.
- `devtest.install`: a one-time price, paid by meter voucher, unit `install`.
- `devtest.metered`: charged per `call` by voucher, with a minimum charge.
- `devtest.escrow`: an escrow package, which pins a chunk root and length.

The Qwen tiers are left out. The seal still signs the model catalogue
(`nonos-data/models/catalogue.bin`) the same way for every image, so a dev
image can still fetch a tier; only its Marketplace listings change.

## Placeholders

None of these values is real, and nobody holds a key for any of them:

- Every `payee` is `0xdddd...dd000N`, a made-up pattern, not a derived
  address.
- Every `binding_sig` is 65 bytes of `0xee`. No bond contract is deployed on
  any chain (`nonos_nox_config`), so the binding check stops at "no domain"
  before it would read the signature.
- `bond_id` is left empty. The CLI derives it from the payee and the
  publisher key.
- Hashes are repeated bytes, and every package URL is under
  `market.devnet.invalid`, a domain reserved so that it never resolves.
- `publisher_pubkey` is `OPERATOR`, which is not valid hex. The seal fills in
  the operator key of the copy it runs in, so an unfilled catalogue fails to
  encode instead of being signed.

## Who reads it

Only `tools/nonos_seal/dev_market.py`, and only when the seal is a
development seal (not `cfg.release`) running in nonos-dev-image's copy at
`target/dev/tree`. That copy signs the catalogue with the throwaway operator
key nonos-dev-image made there, and refuses to sign it if that key is the
one the checkout holds. A release market trusts only the operator key the
checkout tracks (`userland/capsule_market/src/bootstrap_trust/keys.rs`), so
it refuses this catalogue even if the bytes were copied into a release
image. `nonos-mk/tests/dev_catalogue_*.rs` proves both cases against the
capsule's own ingest code. `scripts/check_dev_catalogue.py` fails if a
release path names this directory.

The signature is the only refusal. Every build reads schema 2 and schema 3
alike, and the market capsule never checks the schema, so nothing refuses
this catalogue for being schema 3: do not count on a schema gate.
