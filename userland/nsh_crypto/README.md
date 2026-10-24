# nsh_crypto

The cryptography of nsh/1, as specified in
[docs/design/nsh-v1-wire.md](../../docs/design/nsh-v1-wire.md):

- **The X-Wing exchange of the hellos.** `ClientKem` makes a key pair from a
  32-byte seed and is used once, because decapsulating consumes it.
  `encapsulate` is the server's half, taking 64 bytes of randomness.
- **The transcript chain.** `Transcript` adds one link per frame and can
  give a signed frame's prefix without changing the chain.
- **The key schedule.** `handshake_keys` gives the handshake keys and the
  handshake secret. `session_keys` consumes that secret, so it is used
  once and then wiped.
- **The labelled digests the hybrid keys sign.** `labelled`, with one
  label per purpose. Alongside them come `attest_challenge`, which is the
  32 bytes `HOST_PROOF`'s document must carry, and `key_digest`.
- **Sealed frames.** `Sealer` and `Opener` keep one count per direction,
  with the length field as associated data. They stop at 2^32 frames or
  2^32 bytes. A spent direction cannot seal even an `ABORT`, so the session
  then closes without a frame. An `Opener` that refuses one frame, for any
  reason but a short buffer of the caller's, refuses every frame after
  it.

Every primitive is one the tree already carries:

- X-Wing comes from the `x-wing =0.1.0` crate that `shield_core` pins;
- SHA-512, HMAC and HKDF come from `nonos_hash`;
- ChaCha20-Poly1305 comes from `nonos_seal`.

The crate is `no_std` and reads no entropy of its own: every random byte
is the caller's. Secrets the crate makes are moved in with `Secret::take`,
which wipes the copy they came from. Copies inside the X-Wing crate are
that crate's to wipe, and the client seed in particular is handed over
by value. It builds for `x86_64-nonos-user` and for the host.
Signing is not here; the hybrid signatures come from `nonos_ed25519` and
`nonos-mldsa65`.

The `proofs` feature exists only for the proof crate. It lets a count
start partway, so the limits can be shown. A real session always starts
at zero.

The proofs are in [nsh_crypto_proofs](../nsh_crypto_proofs).
