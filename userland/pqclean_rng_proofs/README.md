# pqclean_rng_proofs

Host proofs that the kernel's post-quantum C fails closed when its
randomness fails. There are 8 tests.

**What is mounted.** The kernel's own files are mounted with `#[path]`, at
the module paths the kernel gives them:

- `src/crypto/pqclean_support/random.rs`: the randomness hook;
- `src/crypto/pqc/ml_dsa_65/drawn.rs`, with its ffi, types and constants;
- `src/crypto/pqc/kyber.rs` and `kyber_drawn.rs`;
- `src/crypto/util/rng/global/fallback.rs`.

They link against the pinned PQClean ML-DSA-65 and ML-KEM-768 C, with the
kernel's `randombytes.c` in front. Only a few things stand in for the
kernel (`src/stand_in*.rs`):

- the seeded RNG, which a test can make fail from a given draw on, or at
  that draw only;
- the hardware entropy source;
- the time counter;
- the serial line.

**What is tested.**

- With a working RNG, ML-DSA-65 makes a key pair and signs. ML-KEM-768's
  two sides share a secret.
- A failed draw refuses an ML-DSA key pair or signature, and an ML-KEM key
  pair or encapsulation. Nothing made from the failed draw is handed back,
  and the next draw that works is used as normal.
- A signature made with a working RNG verifies, through the kernel's own
  `api.rs`, over the message signed, and not over another.
- The C hook itself returns -1 and hands over zeros when a draw fails.
- The weak fallback writes `[RNG] fallback used: weak bytes (entropy
  unavailable)` to the serial line:
  - never while the hardware entropy source answers;
  - once, at the first weak value;
  - not again in the same boot.

**Concurrency.** A draw that fails on any CPU refuses every call running at
the same moment. That is by design: it may refuse a call that would have
been sound, and never accepts one that was not. The tests that make draws
fail therefore take turns.

**Mutants.** 11 mutants were applied to the kernel's sources, one at a
time, and all were caught:

- fallback bytes accepted;
- a failed draw not counted;
- a failed draw's buffer left as it was;
- the C hook reporting success;
- each of the four wrappers ignoring a failed draw;
- the fallback said every time, or never;
- the hardware entropy source skipped.

**What is reviewed, not proved.**

- The wipes of a refused signature or shared secret are checked by
  reading the code. The `Err` the tests see is what hands nothing back.
- The kernel's real `fill_random_bytes_secure` is stood in for here. That
  it never falls back is checked by reading `generate.rs`.

Run with `cargo test` from this directory.
