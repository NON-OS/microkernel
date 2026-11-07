/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Words as bit sets. The kernel keeps capability masks, rights and allowlists as
machine words and tests containment with `a & !b == 0`. `excess a b` is that
`a & !b` on `Nat`, and `Within a b` is the test; the lemmas read both bit by
bit so the modules that import this one reason about sets, not arithmetic.
-/

namespace Nonos.WordSet

/-- Bit `i` of `w`. -/
def bit (w i : Nat) : Bool := w.testBit i

/-- `a & !b`. -/
def excess (a b : Nat) : Nat := a ^^^ (a &&& b)

/-- `a & !b == 0`: every bit of `a` is a bit of `b`. -/
def Within (a b : Nat) : Prop := excess a b = 0

instance (a b : Nat) : Decidable (Within a b) := inferInstanceAs (Decidable (excess a b = 0))

theorem bit_excess (a b i : Nat) : bit (excess a b) i = (bit a i && !bit b i) := by
  unfold bit excess
  rw [Nat.testBit_xor, Nat.testBit_and]
  cases a.testBit i <;> cases b.testBit i <;> rfl

theorem bit_or (a b i : Nat) : bit (a ||| b) i = (bit a i || bit b i) := Nat.testBit_or a b i

theorem bit_and (a b i : Nat) : bit (a &&& b) i = (bit a i && bit b i) := Nat.testBit_and a b i

theorem bit_zero (i : Nat) : bit 0 i = false := Nat.zero_testBit i

theorem eq_zero_iff (n : Nat) : n = 0 ↔ ∀ i, bit n i = false := by
  constructor
  · intro h i; subst h; exact Nat.zero_testBit i
  · intro h
    apply Nat.eq_of_testBit_eq
    intro i
    rw [Nat.zero_testBit]
    exact h i

theorem within_iff (a b : Nat) : Within a b ↔ ∀ i, bit a i = true → bit b i = true := by
  unfold Within
  rw [eq_zero_iff]
  constructor
  · intro h i ha
    have := h i
    rw [bit_excess, ha] at this
    cases hb : bit b i
    · rw [hb] at this; simp at this
    · rfl
  · intro h i
    rw [bit_excess]
    cases ha : bit a i
    · rfl
    · rw [h i ha]; rfl

theorem within_refl (a : Nat) : Within a a := (within_iff a a).2 fun _ h => h

theorem within_trans {a b c : Nat} (h1 : Within a b) (h2 : Within b c) : Within a c :=
  (within_iff a c).2 fun i h => (within_iff b c).1 h2 i ((within_iff a b).1 h1 i h)

theorem within_zero (a : Nat) : Within 0 a :=
  (within_iff 0 a).2 fun i h => by rw [bit_zero] at h; cases h

theorem zero_of_within_zero {a : Nat} (h : Within a 0) : a = 0 := by
  rw [eq_zero_iff]
  intro i
  cases ha : bit a i
  · rfl
  · have := (within_iff a 0).1 h i ha
    rw [bit_zero] at this
    cases this

theorem within_or {a b c : Nat} (ha : Within a c) (hb : Within b c) : Within (a ||| b) c := by
  refine (within_iff _ c).2 fun i h => ?_
  rw [bit_or] at h
  cases hai : bit a i
  · rw [hai] at h
    exact (within_iff b c).1 hb i (by simpa using h)
  · exact (within_iff a c).1 ha i hai

theorem within_or_left (a b : Nat) : Within a (a ||| b) :=
  (within_iff _ _).2 fun i h => by rw [bit_or, h]; rfl

theorem within_or_right (a b : Nat) : Within b (a ||| b) :=
  (within_iff _ _).2 fun i h => by rw [bit_or, h]; cases bit a i <;> rfl

theorem within_and_left (a b : Nat) : Within (a &&& b) a :=
  (within_iff _ _).2 fun i h => by
    rw [bit_and] at h
    cases hai : bit a i
    · rw [hai] at h; simp at h
    · rfl

theorem within_and_right (a b : Nat) : Within (a &&& b) b :=
  (within_iff _ _).2 fun i h => by
    rw [bit_and] at h
    cases hbi : bit b i
    · rw [hbi] at h; simp at h
    · rfl

theorem within_and {a b c : Nat} (hb : Within a b) (hc : Within a c) : Within a (b &&& c) :=
  (within_iff _ _).2 fun i h => by
    rw [bit_and, (within_iff a b).1 hb i h, (within_iff a c).1 hc i h]; rfl

theorem within_excess (a m : Nat) : Within (excess a m) a :=
  (within_iff _ _).2 fun i h => by
    rw [bit_excess] at h
    cases hai : bit a i
    · rw [hai] at h; simp at h
    · rfl

/-- What `excess a m` keeps has no bit of `m`. -/
theorem excess_disjoint (a m i : Nat) (hm : bit m i = true) : bit (excess a m) i = false := by
  rw [bit_excess, hm]; cases bit a i <;> rfl

theorem bit_of_within {a b : Nat} (h : Within a b) {i : Nat} (hi : bit a i = true) : bit b i = true :=
  (within_iff a b).1 h i hi

theorem not_within_of_bit {a b i : Nat} (ha : bit a i = true) (hb : bit b i = false) : ¬ Within a b := by
  intro h
  rw [bit_of_within h ha] at hb
  cases hb

end Nonos.WordSet
