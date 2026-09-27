/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

How many evaluations a column can afford to reveal.

The inner proof is blinded, and blinded is not the same as hiding anything. Each
column carries thirty-five coefficients and the queries reveal thirty-six
evaluations of it, and thirty-six evaluations of a polynomial with thirty-five
coefficients are the polynomial. The blinding randomises which polynomial
interpolates the trace, and then hands over enough points to recover it.

So the arithmetic is the property. `revealing_more_than_the_coefficients_determines`
is the statement, `the_shipped_numbers_leave_nothing_hidden` is it at the numbers
that shipped, and `least_sufficient_mask` says how many extra coefficients close
it: one more than the surplus, which here is two.

What this file does not do is prove interpolation. That a polynomial of degree
below `n` is fixed by `n` evaluations is the fact everything here rests on, and it
is carried as the hypothesis `Interpolating` rather than assumed silently, so a
reader can see exactly where it enters. Everything else is counting, and the
counting is what was wrong.

The composition polynomial has the same shape and is worth stating separately: it
is the one place where a mask that is correct per column can still be insufficient,
because the composition's degree is a sum and its query count is not.
-/

namespace Nonos.Stark.Blinding

/-! ### What a column is -/

/-- A column, as the prover holds it: a coefficient count and the evaluations the
    verifier will be shown. -/
structure Column where
  coeffs : Nat
  revealed : Nat
  deriving DecidableEq, Repr

/-- The surplus: how many more evaluations are revealed than there are
    coefficients to hide. -/
def surplus (c : Column) : Nat := c.revealed - c.coeffs

/-- A column is determined when at least as many evaluations are revealed as it
    has coefficients. -/
def Determined (c : Column) : Prop := c.coeffs ≤ c.revealed

instance (c : Column) : Decidable (Determined c) := by
  unfold Determined; infer_instance

/-- The interpolation fact, stated as a property of a family of columns rather
    than asserted: within this family, agreeing on `coeffs` many evaluations means
    being the same column. This is where the algebra enters, and it enters once. -/
def Interpolating (family : Nat → Nat → Nat) (n : Nat) : Prop :=
  ∀ a b : Nat, (∀ i, i < n → family a i = family b i) → a = b

/-! ### The counting -/

/-- Revealing at least as many evaluations as there are coefficients determines
    the column: two columns of the family that produce the revealed evaluations
    are the same column. -/
theorem revealing_more_than_the_coefficients_determines
    (family : Nat → Nat → Nat) (c : Column) (h : Interpolating family c.coeffs)
    (hd : Determined c) (a b : Nat)
    (hagree : ∀ i, i < c.revealed → family a i = family b i) : a = b := by
  refine h a b ?_
  intro i hi
  exact hagree i (Nat.lt_of_lt_of_le hi hd)

/-- Masking raises the coefficient count without raising the number of
    evaluations revealed. -/
def masked (c : Column) (mask : Nat) : Column := ⟨c.coeffs + mask, c.revealed⟩

/-- A mask larger than the surplus leaves the column undetermined: there are more
    coefficients than the revealed evaluations can pin down. -/
theorem masking_past_the_surplus_hides (c : Column) (mask : Nat)
    (h : c.revealed < c.coeffs + mask) : ¬ Determined (masked c mask) := by
  unfold Determined masked
  simp only
  omega

/-- A mask no larger than the surplus does not: the column is still determined, so
    a mask that is too small is not a partial improvement, it is no improvement. -/
theorem masking_within_the_surplus_hides_nothing (c : Column) (mask : Nat)
    (h : c.coeffs + mask ≤ c.revealed) : Determined (masked c mask) := by
  unfold Determined masked
  simp only
  omega

/-- The least mask that hides anything, for a column that is determined without
    one: strictly more than the surplus, which is the surplus plus one. The
    hypothesis is not decoration, a column that was already undetermined needs no
    mask and the characterisation would be false for it. -/
theorem least_sufficient_mask (c : Column) (mask : Nat) (hd : Determined c) :
    ¬ Determined (masked c mask) ↔ surplus c < mask := by
  unfold Determined surplus at *
  simp only [masked]
  omega

/-! ### The numbers that shipped -/

/-- Thirty-five coefficients per column. -/
def shippedCoeffs : Nat := 35

/-- Thirty-six evaluations revealed per column. -/
def shippedRevealed : Nat := 36

def shippedColumn : Column := ⟨shippedCoeffs, shippedRevealed⟩

/-- The surplus is one. -/
theorem shipped_surplus_is_one : surplus shippedColumn = 1 := by
  unfold surplus shippedColumn shippedCoeffs shippedRevealed
  rfl

/-- And the column is determined, so the revealed evaluations are the witness. -/
theorem the_shipped_numbers_leave_nothing_hidden : Determined shippedColumn := by
  unfold Determined shippedColumn shippedCoeffs shippedRevealed
  simp only
  omega

/-- One extra coefficient is not enough: thirty-six coefficients against
    thirty-six evaluations is still determined. A mask of one is the natural
    guess and it buys nothing. -/
theorem a_mask_of_one_is_not_enough : Determined (masked shippedColumn 1) := by
  unfold Determined masked shippedColumn shippedCoeffs shippedRevealed
  simp only
  omega

/-- Two is. -/
theorem a_mask_of_two_hides : ¬ Determined (masked shippedColumn 2) := by
  unfold Determined masked shippedColumn shippedCoeffs shippedRevealed
  simp only
  omega

/-- And two is the least, so the fix is a specific number rather than a
    direction. -/
theorem two_is_the_least_sufficient_mask (mask : Nat) :
    ¬ Determined (masked shippedColumn mask) ↔ 2 ≤ mask := by
  unfold Determined masked shippedColumn shippedCoeffs shippedRevealed
  simp only
  omega

/-! ### The composition polynomial -/

/-- The composition is built from the trace columns, so its degree is a sum while
    the number of evaluations revealed of it is not. A mask that is right for each
    column can still be wrong for the composition. -/
def composition (parts : List Column) (revealed : Nat) : Column :=
  ⟨totalCoeffs parts, revealed⟩
where
  totalCoeffs : List Column → Nat
    | [] => 0
    | c :: rest => c.coeffs + totalCoeffs rest

/-- Masking each part raises the composition's coefficient count by the sum of the
    masks, so per-column masking does carry through. -/
theorem part_masks_add (a b mask : Nat) :
    (masked ⟨a, 0⟩ mask).coeffs + (masked ⟨b, 0⟩ mask).coeffs = a + b + mask + mask := by
  unfold masked
  simp only
  omega

/-- But the composition is determined whenever its own revealed count reaches the
    sum, whatever each part's margin was. Two columns each hiding by a margin of
    one compose into something revealed at the sum, and the margins do not add up
    to a margin on the composition unless its query count stays put. -/
theorem the_composition_needs_its_own_margin (a b revealed : Nat)
    (h : a + b ≤ revealed) : Determined ⟨a + b, revealed⟩ := by
  unfold Determined
  simp only
  omega

/-- Stated as the obligation: the composition hides only if its revealed count is
    below the total. There is no per-column condition that implies it. -/
theorem composition_hides_iff (a b revealed : Nat) :
    ¬ Determined ⟨a + b, revealed⟩ ↔ revealed < a + b := by
  unfold Determined
  simp only
  omega

/-- What must not be claimed while the counting says otherwise: that the relayer
    learns nothing. A determined column is the witness, so the honest statement is
    that the inner proof is blinded and not hiding. -/
def RelayerLearnsNothing (c : Column) : Prop := ¬ Determined c

theorem the_relayer_learns_the_column : ¬ RelayerLearnsNothing shippedColumn := by
  unfold RelayerLearnsNothing
  intro h
  exact h the_shipped_numbers_leave_nothing_hidden

end Nonos.Stark.Blinding
