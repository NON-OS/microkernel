# nonos_p256_proofs

Host tests for `nonos_p256`, through its public API. Everything in `src/`
is `#[cfg(test)]`.

NIST's CAVP vector files could not be fetched where this was written
(csrc.nist.gov is refused there). The known answers are instead:
- the generator, as FIPS 186-4, D.1.2.3 prints it (recalled). It is
  confirmed by the curve equation and by having order r, which a mistyped
  digit could not survive;
- 2G, kG and one ECDH shared x, computed by a separate affine big-integer
  implementation written for the check;
- the debug key pair of the Bluetooth Core Specification 5.4, Vol 3,
  Part H, 2.3.5.6.1.

6 tests:

| File | Tests | What it checks |
|---|---|---|
| `curve_tests.rs` | 2 | p + (2^256 mod p) is 2^256; G is the printed point and lies on the curve; 2G and kG; (r - 1)G is -G; rG is the identity |
| `ecdh_tests.rs` | 2 | the debug secret gives the debug public key; two secrets reach the same shared x |
| `refuse_tests.rs` | 2 | secrets of 0, r and 2^256 - 1 refused, 1 and r - 1 taken; a point off the curve, a coordinate at or above p, (0, 0) and the identity refused |

Three breaks were tried, each on its own, and each failed a test: a zero
secret accepted, the shared y given for x, and an off-curve point replaced
by the generator instead of refused.

Run with `RUST_TEST_THREADS=1 cargo test --release` from this directory.
