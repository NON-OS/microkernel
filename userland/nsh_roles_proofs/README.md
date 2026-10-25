# nsh_roles_proofs

Host tests for `nsh_roles`, the nsh/1 role table and grant checks. There
are 94 tests.

- **Real signatures.** Tables, revocation lists, grants, approvals and
  `USER_AUTH` bodies carry real Ed25519 and ML-DSA-65 signatures, made with
  eight keys the tests derive from fixed seeds (`src/build/keys.rs`).
- **Every refusal is a test.** The builders in `src/build` make a valid
  body field by field. Each test changes one thing and shows that exactly
  the check for it refuses, at its boundary where it has one: the grant's
  window, the revocation age, the hour an escalation's window may span, and
  the table serial and list issue time a host has already accepted.
- **Both halves.** Each signature is spoiled in its Ed25519 half and in its
  ML-DSA-65 half, and each is refused.
- **Order.** A login's proof of the key comes before anything about the
  grant, and the second part runs check by check.
- **Kernel names.** The capability names are read back from
  `src/capabilities/types/defs.rs` and must match bit for bit.

Each check was also broken on purpose, one at a time, in `nsh_roles`'
source: 107 mutants, 106 caught. The one left, taking the role from the
escalation grant rather than the base grant, is equivalent: the parent check
has already required the two to be equal.

Run the tests with `cargo test` from this directory. The fuzz targets, one
per parser, are in `fuzz/`: seed with `python3 -I fuzz/seed.py --out <dir>`,
then `cargo +nightly-2026-01-16 fuzz run <target> <dir>/<target>`.
