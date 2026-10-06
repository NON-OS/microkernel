# The field

Everything in the crate computes over Goldilocks, the prime field with

```
p = 2^64 - 2^32 + 1 = 0xFFFF_FFFF_0000_0001
```

`src/field/` holds the base field, its quadratic extension, and a generic
tower form of the same extension. The layout follows the crate's house style
of one concern per file: `element.rs` the type and constants, `ops.rs` the
ring operations, `exp.rs` exponentiation, `ext.rs` the concrete `Fp2`,
`tower.rs` the generic `Ext2<F>`, `felt.rs` the trait that lets code be
generic over all of them.

## Why Goldilocks

The prime has the special form `2^64 - 2^32 + 1`, which buys three things at
once. Elements fit exactly in a `u64`, so there is no limb arithmetic.
Reduction after multiplication needs no division, only shifts, masks and two
conditional corrections, because `2^64 ≡ 2^32 - 1 (mod p)`. And the
multiplicative group has order `p - 1 = 2^32 · 3 · 5 · 17 · 257 · 65537`,
whose `2^32` factor supplies power-of-two roots of unity deep enough for any
NTT domain this crate will ever ask for.

## The operations, and the one invariant

`ops.rs` keeps every element canonical, in `[0, p)`, at every step, and each
operation documents its carry logic inline:

- **Addition**: `a + b` lies in `[0, 2p)`. If the `u64` overflowed, the wrap
  discarded `2^64`, which is worth `EPSILON = 2^32 - 1` modulo `p`, so adding
  `EPSILON` back canonicalizes; otherwise one conditional subtraction of `p`
  does.
- **Subtraction**: a borrow means the wrap added `2^64`; subtracting `EPSILON`
  corrects it.
- **Multiplication**: the full 128-bit product is reduced by `reduce128`,
  which splits `x = x_lo + 2^64·(x_hi_lo + 2^32·x_hi_hi)` and uses
  `2^64 ≡ EPSILON` and `2^96 ≡ -1 (mod p)`: subtract `x_hi_hi`, add
  `x_hi_lo·EPSILON`, correct at most one borrow and one carry, and finish with
  a single conditional subtraction. No division anywhere.

The fast reduction is pinned by an in-file property test to the reference
specification, the plain `u128` modulo. That test is the contract: any future
"optimization" of `reduce128` that disagrees with the naive reduction on any
sampled input fails loudly. Fast paths in field arithmetic are exactly where a
subtle bug becomes a soundness bug, so the specification lives next to the
implementation and runs on every test invocation.

## The quadratic extension

STARK soundness over a 64-bit field is not enough for the challenge space:
an adversary who can grind `2^64` options defeats it. Challenges are
therefore drawn from the quadratic extension `Fp2 = Fp[X]/(X^2 - 7)`,
giving a 128-bit challenge space. Seven is a quadratic non-residue in
Goldilocks, so the quotient is a field; it is the same non-residue Plonky2
chose for its Goldilocks extension, which makes cross-checking against an
independent implementation straightforward.

`ext.rs` implements the concrete `Fp2` as a coefficient pair `c0 + c1·X`
with schoolbook multiplication reduced by `X^2 = 7`, inversion via the
conjugate and the norm `N = c0^2 - 7·c1^2` (zero only at zero), and the
Frobenius as conjugation.

## The tower, and the duplicate-truth gate

`tower.rs` provides `Ext2<F: Felt>`, the same construction generic over its
base: `Ext2<Fp>` is `Fp2` again, and `Ext2<Fp2>` is a degree-4 tower when a
consumer needs one. The generic form exists for consumers that build proof
systems over larger extensions; the crate's own attestation path uses `Fp2`.

Two types encoding the same field is a duplicate-truth risk: if their
arithmetic ever disagreed, code could be sound under one and broken under the
other. The tower's tests assert element-for-element agreement between
`Ext2<Fp>` and `Fp2` across the operations, and that test is a permanent
gate, not a development aid: any change to either file that breaks the
agreement fails the suite.

## The `Felt` trait

`felt.rs` abstracts what generic AIR code needs from a field element: the
ring operations, embedding from the base (`from_base`), and constants. It is
what lets a composition check be written once (`air/compose_check_gen.rs`)
and instantiated at `Fp`, `Fp2`, or a tower point without duplication.
