/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The per-process syscall allowlist (`src/syscall/contract/allowlist/`).

`derive.rs` asks the capability tables about every registry slot twice, once
per launcher answer, and keeps the answers as two bitmaps (`set.rs`).
`decide.rs` then answers a call with one bit test. The model keeps the
bitmaps as words and the table as a function from slot and launcher answer to
a decision. The theorems: the list answers exactly what the tables answered
when it was derived; a number with no slot is refused; the launcher question
is asked only where the answers differ; and an out-of-range slot is never set.
-/

import Nonos.WordSet

namespace Nonos.AllowlistBits

open Nonos.WordSet

/-- Two bitmaps over `count` registry slots. -/
structure Allowlist where
  plain : Nat
  launcher : Nat

def empty : Allowlist := ⟨0, 0⟩

/-- `Allowlist::set`: out-of-range slots are ignored. -/
def set (count : Nat) (l : Allowlist) (slot : Nat) (p q : Bool) : Allowlist :=
  if slot < count then
    ⟨if p then l.plain ||| 2 ^ slot else l.plain, if q then l.launcher ||| 2 ^ slot else l.launcher⟩
  else l

def plainAt (count : Nat) (l : Allowlist) (slot : Nat) : Bool := slot < count && bit l.plain slot
def launcherAt (count : Nat) (l : Allowlist) (slot : Nat) : Bool := slot < count && bit l.launcher slot

/-- `derive`: every slot below `count`, both answers, from the table `t`. -/
def derive (count : Nat) (t : Nat → Bool → Bool) : Allowlist :=
  (List.range count).foldl (fun l s => set count l s (t s false) (t s true)) empty

/-- `allowed(list, nr, caller_launches)` with `slotOf` the registry lookup. -/
def allowed (count : Nat) (slotOf : Nat → Option Nat) (l : Allowlist) (nr : Nat) (launches : Bool) : Bool :=
  match slotOf nr with
  | some s => if launches then launcherAt count l s else plainAt count l s
  | none => false

/-- `decide`: the launcher question is asked only when the two answers differ. -/
def decide' (count : Nat) (slotOf : Nat → Option Nat) (l : Allowlist) (nr : Nat) (launches : Unit → Bool) : Bool :=
  match slotOf nr with
  | none => false
  | some s =>
    let p := plainAt count l s
    let q := launcherAt count l s
    if p == q then p else if launches () then q else p

theorem bit_two_pow (n i : Nat) : bit (2 ^ n) i = (i == n) := by
  unfold bit
  rw [Nat.testBit_two_pow]
  by_cases h : i = n
  · subst h; simp
  · simp [h, Ne.symm h]

theorem plain_set (count : Nat) (l : Allowlist) (s : Nat) (p q : Bool) (j : Nat) :
    plainAt count (set count l s p q) j =
      if j = s ∧ s < count then (p || plainAt count l j) else plainAt count l j := by
  unfold plainAt set
  by_cases hs : s < count
  · simp only [hs, if_true]
    by_cases hj : j = s
    · subst hj
      cases p <;> simp [bit_or, bit_two_pow, hs]
    · have : (j == s) = false := by simpa using hj
      cases p <;> simp [bit_or, bit_two_pow, hj, this]
  · simp [hs]

theorem launcher_set (count : Nat) (l : Allowlist) (s : Nat) (p q : Bool) (j : Nat) :
    launcherAt count (set count l s p q) j =
      if j = s ∧ s < count then (q || launcherAt count l j) else launcherAt count l j := by
  unfold launcherAt set
  by_cases hs : s < count
  · simp only [hs, if_true]
    by_cases hj : j = s
    · subst hj
      cases q <;> simp [bit_or, bit_two_pow, hs]
    · have : (j == s) = false := by simpa using hj
      cases q <;> simp [bit_or, bit_two_pow, hj, this]
  · simp [hs]

/-- After folding `set` over a prefix of slots, a slot answers what the table
    said if it was visited, and false otherwise. -/
theorem fold_spec (count : Nat) (t : Nat → Bool → Bool) :
    ∀ (xs : List Nat) (l : Allowlist), xs.Nodup → (∀ x ∈ xs, x < count) →
      (∀ j, j ∉ xs → plainAt count l j = plainAt count l j) →
      ∀ j, plainAt count (xs.foldl (fun l s => set count l s (t s false) (t s true)) l) j =
        (if j ∈ xs then (t j false || plainAt count l j) else plainAt count l j) ∧
      launcherAt count (xs.foldl (fun l s => set count l s (t s false) (t s true)) l) j =
        (if j ∈ xs then (t j true || launcherAt count l j) else launcherAt count l j)
  | [], l, _, _, _, j => by simp
  | x :: xs, l, hnd, hlt, _, j => by
      simp only [List.foldl]
      have hnd' := (List.nodup_cons.1 hnd)
      have ih := fold_spec count t xs (set count l x (t x false) (t x true)) hnd'.2
        (fun y hy => hlt y (List.mem_cons_of_mem _ hy)) (fun _ _ => rfl) j
      rw [ih.1, ih.2, plain_set, launcher_set]
      have hx : x < count := hlt x (List.mem_cons_self _ _)
      by_cases hj : j = x
      · subst hj
        simp [hnd'.1, hx]
      · by_cases hm : j ∈ xs
        · simp [hm, hj]
        · simp [hm, hj]

/-- **The list is the table.** For every slot below `count` and both launcher
    answers, the derived list answers what the table answered. -/
theorem derive_is_table (count : Nat) (t : Nat → Bool → Bool) (s : Nat) (hs : s < count) :
    plainAt count (derive count t) s = t s false ∧ launcherAt count (derive count t) s = t s true := by
  have h := fold_spec count t (List.range count) empty (List.nodup_range count)
    (fun x hx => List.mem_range.1 hx) (fun _ _ => rfl) s
  have hm : s ∈ List.range count := List.mem_range.2 hs
  simp only [derive, hm, if_true] at h ⊢
  have e1 : plainAt count empty s = false := by simp [plainAt, empty, bit_zero]
  have e2 : launcherAt count empty s = false := by simp [launcherAt, empty, bit_zero]
  rw [h.1, h.2, e1, e2]
  simp

/-- An out-of-range slot is never permitted. -/
theorem out_of_range_refused (count : Nat) (l : Allowlist) (s : Nat) (hs : ¬ s < count) :
    plainAt count l s = false ∧ launcherAt count l s = false := by
  simp [plainAt, launcherAt, hs]

/-- A number with no registry slot is refused, whatever the list. -/
theorem unslotted_refused (count : Nat) (slotOf : Nat → Option Nat) (l : Allowlist) (nr : Nat)
    (h : slotOf nr = none) (launches : Bool) : allowed count slotOf l nr launches = false := by
  simp [allowed, h]

/-- `decide` and `allowed` agree: asking the launcher question only where it
    matters changes no answer. -/
theorem decide_eq_allowed (count : Nat) (slotOf : Nat → Option Nat) (l : Allowlist) (nr : Nat)
    (launches : Bool) :
    decide' count slotOf l nr (fun _ => launches) = allowed count slotOf l nr launches := by
  unfold decide' allowed
  cases slotOf nr with
  | none => rfl
  | some s =>
      simp only
      cases hp : plainAt count l s <;> cases hq : launcherAt count l s <;> cases launches <;> rfl

/-- The empty list permits nothing: a token that fails its MAC check under
    token-once gets it. -/
theorem empty_permits_nothing (count : Nat) (slotOf : Nat → Option Nat) (nr : Nat) (launches : Bool) :
    allowed count slotOf empty nr launches = false := by
  unfold allowed
  cases slotOf nr with
  | none => rfl
  | some s => cases launches <;> simp [plainAt, launcherAt, empty, bit_zero]

end Nonos.AllowlistBits
