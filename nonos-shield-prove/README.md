# nonos-shield-prove

The NOX Shield spend prover the OS runs: the production pool's statement
(37 limbs, the not-before time on a 600 second grid), proved without std on
one core inside a capsule. It is the prove path of STARKs' `nox_prover`,
with the host modules it used (files, `/dev/urandom`, process exits) taken
out, built on the STARKs commit the flake pins.

| Call | What it does |
|---|---|
| `prove(request, seed, entropy, progress)` | the proof, its public limbs, the created notes, and the periodic cache |
| `prove_cached(...)` | the same from a cache an earlier proof left |
| `verify`, `share`, `verify_shared` | the format 5 check, the format 7 form the pool reads, and its check |

The request and seed are the pool's own JSON shapes. A proof is verified
and its zero-knowledge rank condition certified before it is returned.

## The gate

`tests/vectors.rs` proves the four pinned production vectors from the
pinned STARKs `spec/wallet-vectors-not-before` and requires every byte of
the proof and of its format 7 form to match:

    STARKS_SPEC=<starks checkout>/spec cargo test --release --features parallel -- --ignored

On 3 October 2026, on 4 host cores: transfer-eth, withdraw-eth,
transfer-nox and withdraw-nox byte for byte, 203 s for the four.
