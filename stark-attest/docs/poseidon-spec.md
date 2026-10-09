# A reproducible Poseidon instance over Goldilocks

This page fully specifies the width-8 Poseidon permutation this crate proves
inside its AIR, in enough detail that a reader can regenerate every constant
from scratch and check the implementation against their own. It is written to
be useful outside this project, because the property it demonstrates is one
most deployed algebraic hashes lack: **every parameter is derived by a rule you
can rerun, not copied from a table you have to trust.**

The width-12 permutation used outside the circuit (Merkle commitments, the
Poseidon transcript) is a different instance and deliberately not this one: it
uses the published Plonky2 and hadeshash constants for Goldilocks, checked
against reference vectors. Reusing a published instance where one exists is the
right default. This page is about the case where none did.

## The problem this addresses

A Poseidon instantiation is a field, a width, an S-box, a round count, a round
constant schedule and a diffusion matrix. Papers specify the first four
precisely and then, for the last two, most deployments ship a table of several
hundred field elements produced by a script that is either unpublished, no
longer runs, or subtly different from the one described. A reviewer who wants
to know whether a constant is adversarially chosen has no way to check without
reimplementing an undocumented generator.

That is a bad position for a primitive whose security argument depends on the
constants having no structure. The fix is not more constants; it is a rule.

## The instance

| Parameter | Value |
|---|---|
| Field | Goldilocks, `p = 2^64 - 2^32 + 1` |
| Width `t` | 8 |
| Rate / capacity | 4 / 4 (256-bit capacity, targeting 128-bit security) |
| S-box | `x^7`, a permutation since `gcd(7, p-1) = 1` |
| Rounds | 32, **all full** (no partial rounds) |
| Diffusion | Cauchy matrix, provably MDS |
| Round constants | `BLAKE3("NONOS-POSEIDON-GOLDILOCKS-RC" ‖ r ‖ j)` |

### Why all-full rounds

The published schedule for this shape is 8 full plus 22 partial rounds. A
partial round applies the S-box to one lane instead of all `t`, so it is
cheaper in a circuit but contributes less to the algebraic degree that
resists Groebner-basis and interpolation attacks. Thirty-two full rounds
strictly dominate 30 mixed ones on every published attack model: every round
here is at least as strong as the strongest round in the reference schedule,
and there are more of them.

The cost is paid in trace rows rather than in security analysis, which is the
right trade for an attestation hash that is proven once per artifact and
verified in milliseconds. A throughput-oriented deployment would choose
differently, and should.

### The diffusion matrix, and why Cauchy

Let `x_i = i` for `i` in `0..t` and `y_j = t + j` for `j` in `0..t`. The
matrix is

```
M[i][j] = 1 / (x_i - y_j)   over Fp
```

Two node sets, `{0..7}` and `{8..15}`, disjoint by construction, so every
difference is nonzero and every entry is defined. A Cauchy matrix over any
field with distinct `x_i` and distinct `y_j` and the two sets disjoint is
**MDS: every square submatrix is invertible**. This is a classical theorem, not
a computational check, which means the branch number is maximal by proof rather
than by search.

Compare the usual practice: pick a candidate matrix, run an MDS test on it,
ship the matrix. That works, but the reviewer must either rerun the test or
trust it. Here the reviewer checks one line of code against a theorem.

The regeneration is eleven lines and needs nothing but field inversion:

```rust
for i in 0..WIDTH {
    for j in 0..WIDTH {
        m[i][j] = (Fp::from(i as u64) - Fp::from((WIDTH + j) as u64)).inv();
    }
}
```

### The round constants, and why a hash

For round `r` and lane `j`:

```
h = BLAKE3("NONOS-POSEIDON-GOLDILOCKS-RC" ‖ LE64(r) ‖ LE64(j))
c[r][j] = Fp::from_u64(LE64(h[0..8]))
```

The domain string is fixed and meaningful, the indices are little-endian
64-bit, and the field element is the first eight bytes of the digest read
little-endian and reduced. Reduction matters and is exact: BLAKE3 outputs a
uniform 64-bit value, Goldilocks `p` is within `2^32` of `2^64`, so a single
conditional subtraction (`if x >= p { x - p }`) canonicalises, because
`2^64 - p < p`. The resulting distribution is very slightly non-uniform, with
the `2^32 - 1` smallest residues twice as likely as the rest; for round
constants, whose requirement is absence of structure rather than uniformity,
that bias is irrelevant and is stated here rather than hidden.

Anyone can regenerate the full schedule with a BLAKE3 implementation and a
loop. There is no table to trust, no script to find, and no way for the author
to have searched for constants with a hidden property: the domain string is
public and the output is a hash of it.

## What this does and does not claim

**Does:** every parameter is reproducible from this page; the diffusion matrix
is MDS by theorem; the S-box is a permutation by a computed coprimality fact
that is machine-checked in `lean/Zkolang/Hash.lean`; the round count strictly
dominates the published schedule for this shape.

**Does not:** this instance has not been independently cryptanalysed. Deriving
constants honestly removes the question "were these chosen adversarially" and
leaves the question "is this instance strong", which only analysis answers.
The margin argument above is a comparison to a published schedule, not a proof
of resistance. Anyone deploying this in an adversarial setting should either
commission that analysis or use a published instance, and the crate itself uses
a published instance everywhere the circuit does not force a smaller width.

## Reproducing it

```
cargo test -p nonos-stark --lib
cargo run --release --bin soundness -- /tmp/s.json
```

The first checks the permutation against the crate's vectors; the second prints
the round count and the parameters actually compiled in, so a reader can
confirm the numbers on this page match the code rather than the intention.

## For implementers of other systems

If you take one thing from this page, take the rule rather than the constants:
derive your round constants from a hash of a public domain string, and choose a
diffusion matrix whose MDS property is a theorem rather than a test result.
Both cost nothing at runtime, both remove a class of "trust the author"
objections permanently, and neither requires anyone to adopt this crate.
