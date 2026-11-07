# lock_proofs

Host-runnable proofs for the lock screen's passphrase, on the real
`userland/nonos_lock` source (which carries the kernel's Argon2 by path).

- The verifier: Argon2id at the shipped parameters matches the right
  passphrase only; a one-byte change, a prefix, an empty passphrase and the
  right passphrase under another salt are all refused. The kernel Argon2 it
  runs matches the independent reference vector.
- The record: 62 bytes that read back as written; a wrong length, magic or
  parameter set the keyring cannot run is refused.
- The backoff: three free wrong entries, then 2 s doubling to 15 min; a count
  read back after a restart waits its whole wait again from boot; a right
  entry clears it.
- The rules: at least 8 characters, at most 256 bytes, text, not only spaces,
  typed the same twice.
- The keyring wire and the on-disk blob: every request reads back as built,
  a body of the wrong length is refused, and each status maps to its answer.

The keyring's handlers over this crate are proved in `userland/wallet_proofs`
(`keyring_lock_tests.rs`).

Run: `cargo +nightly-2026-01-16 test --release` in this directory.
