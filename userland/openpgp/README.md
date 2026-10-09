# openpgp

`nonos_openpgp` verifies OpenPGP v4 detached signatures (RFC 4880, RFC 9580)
against a pinned keyring. It is how the Linux personality checks the signatures
that apt and pacman repositories put on their files. `no_std` with `alloc`. It
depends on `nonos_hash` for SHA-256 and SHA-512 and on the `sha1` crate for v4
key fingerprints.

The crate parses packets and computes the digest. The public key arithmetic is
the caller's, through the `Verifier` trait, so a machine keeps one RSA and one
Ed25519 implementation.

## Public surface

- `keys(ring) -> Option<Vec<Key>>`: every v4 public key and subkey packet in a
  binary transferable public key, as `gpg --export` writes it. RSA (algorithms
  1 and 3), EdDSA over Ed25519 (22) and native Ed25519 (27) are kept; other
  algorithms are skipped. `None` if the file does not frame or holds no usable
  key.
- `Key { fingerprint, material }` and `Material::{Rsa { n, e }, Ed25519(..)}`.
  The fingerprint is the v4 SHA-1 fingerprint.
- `dearmor(text) -> Option<Vec<u8>>`: the binary in the first ASCII-armored
  block. Armor headers are skipped, and the CRC-24 line is checked when present.
- `trait Verifier { fn rsa(..) -> bool; fn ed25519(..) -> bool; }`: for RSA the
  digest goes inside a PKCS#1 v1.5 DigestInfo for the given hash.
- `verify(v, ring, sig, data) -> Result<Verified, Refusal>`: one signature
  packet over `data`. On success, `Verified { fingerprint, hash }` names the key
  that signed.
- `Refusal`, with `why()` for a log line: `Malformed`, `Version`, `NotBinary`,
  `WeakHash` (MD5, SHA-1), `UnknownHash`, `Algorithm`, `Critical`,
  `UnknownKey`, `QuickCheck`, `BadSignature`.

The issuer is found by the issuer fingerprint subpacket, or else by the issuer
key ID matched against a key's low eight fingerprint bytes. A critical
subpacket in the hashed area of a type the crate does not know refuses the
signature.

## What it does not do

It does not sign, encrypt or decrypt. It accepts only v4 signatures over binary
documents (type 0x00) made with SHA-256 or SHA-512. It does not check
signature creation or expiry times, key expiry, key flags, revocations or
subkey binding signatures: every key and subkey in the pinned file is trusted
because it is in the file. Partial and indeterminate packet lengths are
refused. The RSA and Ed25519 math is not here.

## Users

`capsule_linux` (`src/linux/install/deb/`, `pacman/` and `pgp/`). Its
`Verifier` sends RSA to the crypto service and runs Ed25519 through
`nonos_ed25519` (`src/linux/install/pgp/verifier.rs`). `capsule_linux_proofs`
tests the Debian chain, the Kali anchor and pacman RSA signatures through it.

## Tests

Host tests in `tests/`, with a test `Verifier` in `tests/support/host.rs`
(Ed25519 through `nonos_ed25519`, RSA by modular exponentiation and a compare
of the whole PKCS#1 v1.5 block):

- `verify_good.rs`: fingerprints match what GnuPG reported, every GnuPG
  signature verifies, and a subkey signature names the subkey.
- `verify_refused.rs`: a changed file byte, a damaged signature value and the
  wrong keyring are each refused.
- `armor.rs`: armored keys and signatures read and verify; a damaged checksum
  line is refused.
- `mutate.rs`, `mutate_armor.rs`: seeded mutation of keyrings, signatures and
  armor never panics or overflows in a debug build.

The vectors in `tests/vectors/` were written by `tools/nonos-openpgp-vectors`,
which makes throwaway keys with GnuPG and keeps only the public keys,
signatures and fingerprints. The crate is not a `*_proofs` crate, so
`nix flake check` does not run these tests. It runs the `capsule_linux_proofs`
tests that use the crate. No Kani proofs. See
[Linux personality](../../docs/handbook/linux/personality.md).
