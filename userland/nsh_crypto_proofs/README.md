# nsh_crypto_proofs

Host proofs for `nsh_crypto`. There are 24 tests.

- **X-Wing against the specification's three published vectors.** The
  vectors are in `vectors/xwing.txt`, converted from the JSON that ships
  with the `x-wing` crate, and kept with that file's digest. The tests
  check:
  - the key pair from the seed;
  - the ciphertext and secret from the encapsulation randomness;
  - decapsulation back to the same secret;
  - that a changed ciphertext gives another secret;
  - that a malformed key is refused.
- **The transcript chain, the key schedule, the seven labels and the
  attestation challenge.** These are checked against
  `vectors/schedule.txt`. Its values were computed with Python's `hashlib`
  and `hmac` from the definitions in the spec, not from the crate.
- **Known-answer sealed frames.** Four frames are checked against
  `vectors/frames.txt`, which was sealed with the `cryptography` package's
  ChaCha20-Poly1305 from the spec's layout. They sit at sequence 0, 1, 300
  and the last one a direction has. This is what pins the nonce layout and
  the associated data to the spec.
- **Sealed frames.** The tests check:
  - frames open in order;
  - the same bytes seal differently each time;
  - a frame out of order is refused and ends the opener;
  - a changed byte anywhere is refused, the length field included;
  - a frame sealed one way does not open the other way;
  - a length that disagrees with the bytes is refused, even when its tag
    is good.
- **Limits and edges.**
  - The last frame and the last byte of a direction are sealed and opened,
    and the next is refused.
  - A frame refused for the limit is not counted, and it ends the opener.
  - The opener counts the bytes it opens.
  - A length mismatch ends the opener.
  - A buffer too small is the caller's error and ends nothing.
  - A seal that fails is not counted.
  - A frame with no type byte is refused by both sides.
- **The handshake end to end, through `nsh_wire`'s encoders and parsers.**
  Both sides reach the same keys, each direction has its own, and the
  session keys are new keys. A server hello changed in flight leaves the
  keys apart, so the first sealed frame does not open.

Each rule was also broken on purpose, one at a time: 24 mutants, all
caught. They cover:

- the entry length, the type byte, the start label and the chaining;
- one key used for both directions;
- extract's salt and secret swapped;
- session keys not bound to the transcript;
- the challenge leaving out the host keys;
- a reused label;
- the sequence and byte counts, and both limits;
- the nonce's layout;
- the length field not bound (sealing or opening) or not checked;
- the opener not counting bytes, surviving a refusal, a limit or a length
  mismatch, or ending on a short buffer;
- an empty frame sealed or opened.

Run with `cargo test` from this directory.
