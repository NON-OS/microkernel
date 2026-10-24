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
- **No allocator is needed.** The C's only allocation is one 208-byte SHAKE
  state. It is answered from four fixed slots.
- **The C's outside calls are renamed.** `malloc`, `free`, `exit` and
  `randombytes` become this crate's own functions, so the crate links
  beside any libc a capsule or host already has.
- **Every call into the C takes a turn.** Threads queue rather than run the
  C side by side. A randomness source must not call back into this crate.
- **The capsule build.** For a NONOS capsule target, the C is built
  freestanding for the bare architecture, as `nonos_qjs` does.
- **Secret handling.** A secret key is wiped when dropped, and a failed
  signature is wiped before the error is returned.

The proofs are in [userland/mldsa65_proofs](../userland/mldsa65_proofs).
