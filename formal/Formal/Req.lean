/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What a syscall asks of the caller's token. `src/syscall/contract/cap_table`
decides each number with a boolean expression over the token: `is_valid()`,
`grants(Capability::X)`, the `can_*` helpers built from those, and for
MkAppLaunch whether the caller launches programs. `Req` is that expression
language; formal/scripts/gen_tables.py reads every arm into it.
-/

import Formal.Bits

namespace Formal

/-- A syscall's requirement, as written in the cap table. -/
inductive Req where
  /-- `true`: no condition beyond the resolver's token checks. -/
  | yes
  /-- `is_valid()`: not expired and holding at least one capability. -/
  | valid
  /-- `launcher()`: the caller is the desktop shell or a listed launcher. -/
  | launcher
  /-- `grants(Capability::X)` for the capability with bit index `i`. -/
  | has (i : Nat)
  | not (r : Req)
  | and (a b : Req)
  | or (a b : Req)
  deriving Repr

/-- The requirement evaluated on a token's permission word. A token minted by
    `process::caps::new_token` has no expiry, so `is_valid()` is the
    non-empty test alone. -/
def Req.eval (perms : Caps) (launcher : Bool) : Req → Bool
  | .yes => true
  | .valid => perms != 0
  | .launcher => launcher
  | .has i => Formal.has perms i
  | .not r => !(r.eval perms launcher)
  | .and a b => a.eval perms launcher && b.eval perms launcher
  | .or a b => a.eval perms launcher || b.eval perms launcher

/-- Whether the requirement contains a negation, the only way holding more
    could refuse a call. -/
def Req.hasNot : Req → Bool
  | .not _ => true
  | .and a b => a.hasNot || b.hasNot
  | .or a b => a.hasNot || b.hasNot
  | _ => false

/-- A non-empty word stays non-empty under widening. -/
theorem ne_zero_of_within {a b : Caps} (h : Within a b) (ha : a ≠ 0) : b ≠ 0 := by
  intro hb
  apply ha
  rw [eq_zero_iff]
  intro i
  cases hai : has a i
  · rfl
  · have := has_of_within h hai
    rw [hb, has_zero] at this
    exact absurd this (by decide)

/-- A requirement without negation is monotone: a token holding more passes
    every such check a smaller token passes. -/
theorem Req.eval_mono {a b : Caps} (hab : Within a b) (l : Bool) :
    ∀ r : Req, r.hasNot = false → r.eval a l = true → r.eval b l = true
  | .yes, _, _ => rfl
  | .valid, _, h => by
      simp only [Req.eval, bne_iff_ne, ne_eq] at h ⊢
      exact ne_zero_of_within hab h
  | .launcher, _, h => h
  | .has _, _, h => by simp only [Req.eval] at h ⊢; exact has_of_within hab h
  | .not _, hn, _ => by simp [Req.hasNot] at hn
  | .and x y, hn, h => by
      simp only [Req.hasNot, Bool.or_eq_false_iff] at hn
      simp only [Req.eval, Bool.and_eq_true] at h ⊢
      exact ⟨Req.eval_mono hab l x hn.1 h.1, Req.eval_mono hab l y hn.2 h.2⟩
  | .or x y, hn, h => by
      simp only [Req.hasNot, Bool.or_eq_false_iff] at hn
      simp only [Req.eval, Bool.or_eq_true] at h ⊢
      cases h with
      | inl h => exact Or.inl (Req.eval_mono hab l x hn.1 h)
      | inr h => exact Or.inr (Req.eval_mono hab l y hn.2 h)

end Formal
