/-
 stark-attest by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Field

/-!
The Goldilocks reduction the crate runs, proven congruent to the modulus it
claims to compute.

`field/ops.rs::reduce128` reduces a 128-bit product to the field without a
division, using two special-form identities of `p = 2^64 - 2^32 + 1`:

  2^64 ≡ 2^32 - 1   (mod p)      -- the constant EPSILON = 2^32 - 1
  2^96 ≡ -1         (mod p)

The Rust splits the input `x = x_lo + 2^64 * x_hi`, further `x_hi = x_hi_lo +
2^32 * x_hi_hi`, and computes `x_lo - x_hi_hi + x_hi_lo * EPSILON` with carry
corrections. The property test in `ops.rs` samples this against the plain
`u128` modulo; this module proves the identity behind it for every input, so
the sampling is a backstop rather than the guarantee.

The proofs are elementary on purpose: the difference between the original
value and the limb form is exhibited as an explicit integer multiple of `p`,
`omega` certifies the linear identity, and one core-library lemma about
remainders closes each congruence. Core library only; no axioms beyond what
`Field` already uses. The carry corrections in the Rust manage ranges, not
residues; the residue identity proven here is what makes that management
correct rather than lucky.
-/

namespace Zkolang.Reduce

open Zkolang.Field

/-- `EPSILON = 2^32 - 1`, the residue of `2^64` modulo `p`. -/
def epsilon : Int := 2 ^ 32 - 1

/-- `2^64` and `EPSILON` are congruent mod `p`: their difference is exactly `p`. -/
theorem two64_cong_epsilon : cong (2 ^ 64) epsilon := by
  unfold cong epsilon p
  omega

/-- `2^96 ≡ -1 (mod p)`: the difference `2^96 + 1` is `2^32 * p`. -/
theorem two96_cong_neg_one : cong (2 ^ 96) (-1) := by
  unfold cong p
  omega

/-- The value the fast path forms from the three limbs:
`x_lo - x_hi_hi + x_hi_lo * EPSILON`. -/
def reduced (x_lo x_hi_lo x_hi_hi : Int) : Int :=
  x_lo - x_hi_hi + x_hi_lo * epsilon

/-- The reconstruction of the 128-bit value from its limbs:
`x = x_lo + 2^64 * (x_hi_lo + 2^32 * x_hi_hi)`. -/
def original (x_lo x_hi_lo x_hi_hi : Int) : Int :=
  x_lo + 2 ^ 64 * (x_hi_lo + 2 ^ 32 * x_hi_hi)

/-- The original value exceeds the limb form by exactly
`p * (x_hi_lo + (2^32 + 1) * x_hi_hi)`: the whole fast path in one linear
identity. The second coefficient is `2^32 + 1` because `2^96 + 1 = p * (2^32 + 1)`,
which is the `2^96 = -1` identity in explicit multiple form. -/
theorem difference_is_multiple (x_lo x_hi_lo x_hi_hi : Int) :
    original x_lo x_hi_lo x_hi_hi =
      reduced x_lo x_hi_lo x_hi_hi + p * (x_hi_lo + (2 ^ 32 + 1) * x_hi_hi) := by
  unfold original reduced epsilon p
  omega

/-- The congruence: the limb form the fast path reduces is congruent, mod `p`,
to the 128-bit value it came from, for every choice of limbs. -/
theorem reduce_cong (x_lo x_hi_lo x_hi_hi : Int) :
    cong (original x_lo x_hi_lo x_hi_hi) (reduced x_lo x_hi_lo x_hi_hi) := by
  unfold cong original reduced epsilon p
  omega

/-- The shape `Fp::mul` relies on: splitting an arbitrary product into the
exact limbs the Rust extracts (low 64 bits, then the high part's low 32 and
high 32), the fast reduction is congruent to the product itself. -/
theorem mul_reduce_cong (a b : Int) :
    cong (a * b)
      (reduced (a * b % 2 ^ 64) (a * b / 2 ^ 64 % 2 ^ 32) (a * b / 2 ^ 64 / 2 ^ 32)) := by
  have hx :
      a * b =
        original (a * b % 2 ^ 64) (a * b / 2 ^ 64 % 2 ^ 32) (a * b / 2 ^ 64 / 2 ^ 32) := by
    unfold original
    omega
  exact cong_trans (transfer hx) (reduce_cong _ _ _)

end Zkolang.Reduce
