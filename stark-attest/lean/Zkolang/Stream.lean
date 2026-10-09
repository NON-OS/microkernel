/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The streamed prover walks the evaluation domain one coset at a time: position
`j = c + blowup * i` of a domain of size `blowup * t` is row `i` of coset `c`.
Its composition window reads `(j + k * blowup) % n`, and the whole design rests
on one arithmetic fact: that read lands at row `(i + k) % t` of the same coset,
so a pass holding a single coset in memory sees every value a window needs.

The Rust comment says this is the only fact the function rests on. A fact that
load-bearing should not be prose, so here it is a theorem, over the core
library. If it were false, streaming would read a different polynomial's value
and the digest gate would have caught the bytes; this catches the reasoning.
-/

namespace Zkolang.Stream

/-- Row `i` of coset `c` is domain position `c + blowup * i`. -/
def pos (c blowup i : Nat) : Nat := c + blowup * i

/-- The window read from position `j`: `k` strides of `blowup`, wrapped to the
domain of size `blowup * t`. -/
def windowRead (j k blowup t : Nat) : Nat := (j + blowup * k) % (blowup * t)

/-- The window never leaves its coset: reading `k` strides ahead of row `i` of
coset `c` lands at row `(i + k) % t` of coset `c`. -/
theorem window_stays_in_coset (c blowup i k t : Nat)
    (hc : c < blowup) (ht : 0 < t) :
    windowRead (pos c blowup i) k blowup t = pos c blowup ((i + k) % t) := by
  unfold windowRead pos
  -- The read is c + blowup * (i + k); split i + k into its residue and its
  -- full periods, and the periods vanish against the modulus.
  have hik : c + blowup * i + blowup * k = c + blowup * (i + k) := by
    rw [Nat.mul_add]; exact Nat.add_assoc c (blowup * i) (blowup * k)
  rw [hik]
  have hr : (i + k) % t < t := Nat.mod_lt _ ht
  conv => lhs; rw [← Nat.mod_add_div (i + k) t]
  have hsplit :
      c + blowup * ((i + k) % t + t * ((i + k) / t))
        = c + blowup * ((i + k) % t) + blowup * t * ((i + k) / t) := by
    rw [Nat.mul_add, ← Nat.mul_assoc, ← Nat.add_assoc]
  rw [hsplit, Nat.add_mul_mod_self_left]
  -- What remains is below the modulus, so the residue is itself.
  apply Nat.mod_eq_of_lt
  have h1 : c + blowup * ((i + k) % t) < blowup * (1 + (i + k) % t) := by
    rw [Nat.mul_add, Nat.mul_one]; omega
  have h2 : blowup * (1 + (i + k) % t) ≤ blowup * t :=
    Nat.mul_le_mul_left _ (by omega)
  omega

/-- Positions of distinct rows of one coset are distinct: a coset write of the
streamed digest layer cannot collide with itself. -/
theorem coset_rows_injective (c blowup : Nat) (hb : 0 < blowup) {i j : Nat}
    (h : pos c blowup i = pos c blowup j) : i = j := by
  unfold pos at h
  have := Nat.add_left_cancel h
  exact Nat.eq_of_mul_eq_mul_left hb this

/-- Positions of distinct cosets are distinct rows apart: two cosets of the
streamed commit never write the same digest slot. -/
theorem cosets_disjoint (blowup : Nat) {c c' i i' : Nat}
    (hc : c < blowup) (hc' : c' < blowup)
    (h : pos c blowup i = pos c' blowup i') : c = c' := by
  unfold pos at h
  have h1 : (c + blowup * i) % blowup = c := by
    rw [Nat.add_mul_mod_self_left]
    exact Nat.mod_eq_of_lt hc
  have h2 : (c' + blowup * i') % blowup = c' := by
    rw [Nat.add_mul_mod_self_left]
    exact Nat.mod_eq_of_lt hc'
  rw [← h1, ← h2, h]

end Zkolang.Stream
