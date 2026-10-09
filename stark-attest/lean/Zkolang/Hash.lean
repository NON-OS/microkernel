/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
A structural property of the MiMC S-box. The round map raises the state to the seventh
power, and `x ^ 7` is a permutation of the Goldilocks field exactly when the exponent is
coprime to the order of the multiplicative group, `p - 1`. This proves that coprimality by
computation, so the S-box is a bijection and the permutation the hash iterates is invertible,
the property its round count builds security on. The field is Goldilocks, with prime
`p = 2^64 - 2^32 + 1`, so the group order is `p - 1 = 2^64 - 2^32`.
-/

namespace Zkolang.Hash

/-- The Goldilocks prime, `2^64 - 2^32 + 1`. -/
def p : Nat := 2 ^ 64 - 2 ^ 32 + 1

/-- The order of the multiplicative group, `p - 1`. -/
def groupOrder : Nat := p - 1

/-- The MiMC S-box exponent. -/
def sboxExp : Nat := 7

/-- The S-box exponent is coprime to the multiplicative group order: its greatest common
divisor with `p - 1` is one. This is exactly the condition under which `x ^ 7` is a bijection
of the field, so the round map is a genuine permutation and the hash it iterates is
invertible, the property its round count builds security on. -/
theorem sbox_is_permutation : Nat.gcd sboxExp groupOrder = 1 := by decide

end Zkolang.Hash
