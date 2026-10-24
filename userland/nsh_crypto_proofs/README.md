# nsh_crypto_proofs

Host proofs for `nsh_crypto`. There are 17 tests.

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
- **Sealed frames.** The tests check:
  - frames open in order;
  - the same bytes seal differently each time;
  - a frame out of order is refused and ends the opener;
  - a changed byte anywhere is refused, the length field included;
  - a frame sealed one way does not open the other way;
  - a length that disagrees with the bytes is refused, even when its tag
    is good.
- **Limits.** The last frame and the last byte of a direction are sealed
  and opened, and the next is refused. A frame refused for the limit is
  not counted.
- **The handshake end to end, through `nsh_wire`'s encoders and parsers.**
  Both sides reach the same keys, each direction has its own, and the
  session keys are new keys. A server hello changed in flight leaves the
  keys apart, so the first sealed frame does not open.

Each rule was also broken on purpose, one at a time: 16 mutants, all
caught. They cover:

- the entry length, the type byte, the start label and the chaining;
- one key used for both directions;
- extract's salt and secret swapped;
- session keys not bound to the transcript;
- the challenge leaving out the host keys;
- a reused label;
- the sequence and byte counts, and both limits;
- the length field not bound, or not checked;
- the opener surviving a refusal.

Run with `cargo test` from this directory.
