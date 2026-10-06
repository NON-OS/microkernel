# nonos-data

The NONOS trust keystore: the committed, public half of the trust
chain. Everything a verifier needs lives here; nothing a signer needs
does. Seeds, private keys, and device slot material never enter this
repository.

## Layout

| Path | Holds |
|------|-------|
| `trust/policy/` | the sealed trust-anchor policy and the capsule policy root |
| `trust/keys/` | every publisher's Ed25519 and ML-DSA-65 public keys |
| `trust/capsules/` | per capsule: NONOS-ID certificate, signed manifest, STARK membership trailer |
| `trust/MANIFEST.sha256` | the ledger: one hash per artifact, pinned and CI-checked |

## The ledger

`MANIFEST.sha256` is stamped over the whole tree with a fixed recipe
and checked on every change and every release: a clean checkout in
which any artifact disagrees with the ledger fails CI. Re-enrolling
the keystore and forgetting to restamp the ledger is the historical
failure mode this exists to catch, and it does.

## The ceremony

`CUSTODY.md` documents mint, enroll, verify, and rotation. The build
half of enrollment runs on a reproducible builder so that what gets
signed is exactly what any pipeline on the pinned toolchain can
rebuild and match; the signing half runs where the seeds live and
nowhere else.

## License

AGPL-3.0, like the rest of NONOS.
