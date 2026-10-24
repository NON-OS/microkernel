# nonos-mldsa65

ML-DSA-65 (FIPS 204) for userland and the host. It provides key pairs,
signatures with an empty context, and verification, all over the pinned C
in `third_party/pqclean/crypto_sign/ml-dsa-65/clean`. That is the same
source the kernel, the loader and `nonos-sign` compile. The tree's pin
check (`nonos-ci/check-pqclean-pin.sh`) covers it unchanged.

- **Randomness comes from the caller.** Each key pair or signature takes a
  source closure, and the crate reads no entropy of its own. The C ignores
  whether a draw succeeded, so the crate notes any failed draw itself and
  fails the whole call. Otherwise a failed draw would yield a key or a
  signature made from zeros.
- **No allocator is needed.** The C allocates only SHAKE states (208 or 200
  bytes), one at a time. They are answered from four fixed slots.
- **The C's outside calls are renamed.** `malloc`, `free`, `exit` and
  `randombytes` become this crate's own functions, and so do the SHAKE and
  SHA-3 names the C exports (`shake_names.txt`). The crate therefore links
  beside any libc, and beside another copy of the SHAKE code. The
  `PQCLEAN_MLDSA65_CLEAN_*` names stay as they are, so a binary must not
  link both this crate and `nonos-sign`.
- **Every call into the C takes a turn.** Threads queue rather than run the
  C side by side. A randomness source must not call back into this crate,
  and must not panic: a panic cannot cross the C, and ends the program.
- **The capsule build.** For a NONOS capsule target, the C is built
  freestanding for the bare architecture, as `nonos_qjs` does.
- **Secret handling.** The `SecretKey` a caller holds is wiped when dropped,
  and a failed signature is wiped before the error is returned. The C's own
  stack and the copies a move leaves behind are not wiped. A caller that
  needs that keeps the key in one place and signs there.

The proofs are in [userland/mldsa65_proofs](../userland/mldsa65_proofs).
