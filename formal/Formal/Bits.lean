/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Capability words. The kernel keeps a capsule's authority as a `u64` with one
bit per capability (`Capability::bit`), and every check is a mask operation on
that word. The model keeps the same words as `Nat`, so a check in Lean reads
as the Rust it stands for: `excess a b` is `a & !b`, and `Within a b` is
`a & !b == 0`, the test `image_ceiling::gate`, `within_ceiling` and
`grant_within_manifest` all make.
-/

namespace Formal

/-- A capability word. -/
abbrev Caps := Nat

/-- Whether the word holds capability bit `i`: `bits & (1 << i) != 0`. -/
def has (c : Caps) (i : Nat) : Bool := c.testBit i

/-- The bits of `a` outside `b`: `a & !b` (`image_ceiling::rule::over`). -/
def excess (a b : Caps) : Caps := a ^^^ (a &&& b)

/-- `a` grants nothing outside `b`: `a & !b == 0`. -/
def Within (a b : Caps) : Prop := excess a b = 0

instance (a b : Caps) : Decidable (Within a b) :=
  inferInstanceAs (Decidable (excess a b = 0))

theorem has_excess (a b i : Nat) : has (excess a b) i = (has a i && !has b i) := by
  unfold has excess
  rw [Nat.testBit_xor, Nat.testBit_and]
  cases a.testBit i <;> cases b.testBit i <;> rfl

theorem has_or (a b i : Nat) : has (a ||| b) i = (has a i || has b i) := Nat.testBit_or a b i

theorem has_and (a b i : Nat) : has (a &&& b) i = (has a i && has b i) := Nat.testBit_and a b i

theorem has_zero (i : Nat) : has 0 i = false := Nat.zero_testBit i

theorem eq_zero_iff (n : Nat) : n = 0 ↔ ∀ i, has n i = false := by
  constructor
  · intro h i; subst h; exact Nat.zero_testBit i
  · intro h
    apply Nat.eq_of_testBit_eq
    intro i
    rw [Nat.zero_testBit]
    exact h i

/-- The mask test is set inclusion, bit by bit. -/
theorem within_iff (a b : Caps) : Within a b ↔ ∀ i, has a i = true → has b i = true := by
  unfold Within
  rw [eq_zero_iff]
  constructor
  · intro h i ha
    have := h i
    rw [has_excess, ha] at this
    cases hb : has b i
    · rw [hb] at this; exact absurd this (by decide)
    · rfl
  · intro h i
    rw [has_excess]
    cases ha : has a i
    · rfl
    · rw [h i ha]; rfl

theorem within_refl (a : Caps) : Within a a :=
  (within_iff a a).2 fun _ h => h

theorem within_trans {a b c : Caps} (hab : Within a b) (hbc : Within b c) : Within a c :=
  (within_iff a c).2 fun i h => (within_iff b c).1 hbc i ((within_iff a b).1 hab i h)

theorem within_zero (a : Caps) : Within 0 a :=
  (within_iff 0 a).2 fun i h => by rw [has_zero] at h; exact absurd h (by decide)

theorem within_or {a b c : Caps} (ha : Within a c) (hb : Within b c) : Within (a ||| b) c := by
  refine (within_iff _ c).2 fun i h => ?_
  rw [has_or] at h
  cases hai : has a i
  · rw [hai] at h
    exact (within_iff b c).1 hb i (by simpa using h)
  · exact (within_iff a c).1 ha i hai

theorem within_or_left (a b : Caps) : Within a (a ||| b) :=
  (within_iff _ _).2 fun i h => by rw [has_or, h]; rfl

theorem within_or_right (a b : Caps) : Within b (a ||| b) :=
  (within_iff _ _).2 fun i h => by rw [has_or, h]; cases has a i <;> rfl

theorem within_and_left (a b : Caps) : Within (a &&& b) a :=
  (within_iff _ _).2 fun i h => by
    rw [has_and] at h
    cases hai : has a i
    · rw [hai] at h; simp at h
    · rfl

theorem within_and_right (a b : Caps) : Within (a &&& b) b :=
  (within_iff _ _).2 fun i h => by
    rw [has_and] at h
    cases hbi : has b i
    · rw [hbi] at h; simp at h
    · rfl

theorem within_excess (a m : Caps) : Within (excess a m) a :=
  (within_iff _ _).2 fun i h => by
    rw [has_excess] at h
    cases hai : has a i
    · rw [hai] at h; simp at h
    · rfl

theorem within_or_mono {a b c d : Caps} (hac : Within a c) (hbd : Within b d) :
    Within (a ||| b) (c ||| d) :=
  within_or (within_trans hac (within_or_left c d)) (within_trans hbd (within_or_right c d))

/-- A bit held by a word inside `b` is held by `b`. -/
theorem has_of_within {a b : Caps} (h : Within a b) {i : Nat} (hi : has a i = true) :
    has b i = true :=
  (within_iff a b).1 h i hi

/-! The two `Option.map` facts the proofs use, proved here because the core
library renamed its own between the Lean releases the two projects pin; with
these the model builds unchanged on both. -/

theorem option_map_eq_some {α β : Type} {f : α → β} {o : Option α} {b : β} :
    o.map f = some b ↔ ∃ a, o = some a ∧ f a = b := by
  cases o with
  | none => exact ⟨fun h => (nomatch h), fun ⟨_, h, _⟩ => (nomatch h)⟩
  | some a =>
      refine ⟨fun h => ⟨a, rfl, Option.some.inj h⟩, fun ⟨a', h, hf⟩ => ?_⟩
      cases h
      exact congrArg some hf

theorem option_map_some {α β : Type} (f : α → β) (a : α) : (some a).map f = some (f a) := rfl

end Formal
