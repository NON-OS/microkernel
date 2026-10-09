/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The Goldilocks field as the integers modulo the prime, and the precise sense in which the
gadget identities hold in it. Field equality is congruence modulo `p = 2^64 - 2^32 + 1`,
defined here directly as equality of remainders so the development stays in the core library.
Every integer reduces to a canonical representative in `[0, p)`, and any integer identity a
gadget proves descends to the field, because equal integers are congruent: this `transfer` is
what lets the integer proofs of the other modules stand as field proofs. Addition, its
inverse, and multiplication are the integer operations read through the congruence, so they
carry their ring laws; the additive inverse is exhibited, and congruence is shown compatible
with the two operations.

What is deliberately not proven: the multiplicative inverse the `field` gadgets name
(`recip`, `divide`, `ratio`) exists for every nonzero element only because `p` is prime, by
Fermat's little theorem. Establishing the primality of a sixty-four-bit number and Fermat's
theorem needs machinery beyond the core library this development is confined to, so those
facts are noted, not assumed: there is no axiom here standing in for them.
-/

namespace Zkolang.Field

/-- The Goldilocks prime, `2^64 - 2^32 + 1`. -/
def p : Int := 2 ^ 64 - 2 ^ 32 + 1

/-- Field equality: two integers are equal in the field when they share a remainder mod p. -/
def cong (a b : Int) : Prop := a % p = b % p

/-- The additive inverse gadget, `0 - x`. -/
def neg (x : Int) : Int := 0 - x

/-- The modulus is positive. -/
theorem p_pos : 0 < p := by decide

/-- The modulus is nonzero. -/
theorem p_ne_zero : p ≠ 0 := by decide

/-- The transfer principle: an integer identity descends to the field, because equal
integers share a remainder. Every gadget theorem proven as an integer equality is, through
this, a statement about the Goldilocks field. -/
theorem transfer {a b : Int} (h : a = b) : cong a b := by unfold cong; rw [h]

/-- Congruence is reflexive, symmetric, and transitive: it is an equality. -/
theorem cong_refl (a : Int) : cong a a := rfl
theorem cong_symm {a b : Int} (h : cong a b) : cong b a := h.symm
theorem cong_trans {a b c : Int} (h1 : cong a b) (h2 : cong b c) : cong a c := h1.trans h2

/-- Every integer is congruent to its canonical representative. -/
theorem canonical (a : Int) : cong a (a % p) := by
  unfold cong; exact (Int.emod_emod_of_dvd a ⟨1, by omega⟩).symm

/-- The canonical representative lies in `[0, p)`: the field has exactly `p` elements. -/
theorem canonical_range (a : Int) : 0 ≤ a % p ∧ a % p < p :=
  ⟨Int.emod_nonneg a p_ne_zero, Int.emod_lt_of_pos a p_pos⟩

/-- The additive inverse gadget really inverts: `x + (0 - x)` is zero in the field. -/
theorem neg_add (x : Int) : cong (x + neg x) 0 := by
  unfold neg; exact transfer (by omega)

/-- Congruence is compatible with addition, so a gadget built by adding respects the field
equality of its parts. -/
theorem add_compat {a b c d : Int} (h1 : cong a b) (h2 : cong c d) : cong (a + c) (b + d) := by
  unfold cong at *
  rw [Int.add_emod, h1, h2, ← Int.add_emod]

/-- Congruence is compatible with multiplication. -/
theorem mul_compat {a b c d : Int} (h1 : cong a b) (h2 : cong c d) : cong (a * c) (b * d) := by
  unfold cong at *
  rw [Int.mul_emod, h1, h2, ← Int.mul_emod]

/-- A worked descent: the quadratic Horner identity, proven over the integers in `Poly`,
holds in the field. Any of the module identities transfers the same way. -/
theorem quad_horner_in_field (a b c x : Int) :
    cong ((a * x + b) * x + c) (a * x * x + b * x + c) :=
  transfer (by rw [Int.add_mul])

end Zkolang.Field
