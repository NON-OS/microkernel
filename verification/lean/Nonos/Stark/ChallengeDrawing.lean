/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Where the permutation argument's challenges come from.

The copy constraint is a grand product. Two multisets agree exactly when their
shifted products agree as polynomials in the challenge, and the argument works
because the prover commits to its columns before the challenge exists. Ours was
evaluated at fixed constants: beta was five and gamma was seven, written into the
circuit, never drawn from the transcript. A prover that knows the challenge in
advance is not being asked a question, it is being shown the answer.

`fixed_challenge_is_unsound` is the forgery, at the smallest scale that shows it.
Two lists whose products at the challenge agree and whose sums do not, so not even
the weakest consequence of multiset equality follows from acceptance. The same
pair is rejected at any other challenge, which is the point:
`the_witness_fails_at_a_different_challenge` says the argument would have caught
it if the challenge had been drawn.

The rest says what drawing has to mean. A draw that ignores the commitment is a
constant, and a constant is known before committing, so it is the same as no
challenge at all. A draw that separates distinct commitments is one a prover
cannot answer in advance, because the answer does not exist until it has
committed.

This file does not prove the soundness bound. The number of bad challenges for a
false claim is bounded by the degree, and that argument needs the polynomial
machinery this corpus deliberately does not carry; what is here is the structural
half, which is the half that was wrong.
-/

namespace Nonos.Stark.ChallengeDrawing

/-! ### The grand product -/

/-- The shifted product of a column at a challenge, as the argument computes it. -/
def product (beta : Nat) : List Nat → Nat
  | [] => 1
  | a :: rest => (a + beta) * product beta rest

/-- The sum of a column, used below only as a witness that two lists are not
    rearrangements of one another. Any rearrangement preserves it, so a
    difference in sums is a difference in multisets. -/
def total : List Nat → Nat
  | [] => 0
  | a :: rest => a + total rest

/-- What the argument concludes: the two columns hold the same values. `total` is
    a necessary consequence, so an argument that can accept while the totals
    differ has concluded something false. -/
def SoundAt (beta : Nat) : Prop :=
  ∀ A B : List Nat, product beta A = product beta B → total A = total B

/-! ### A challenge the prover knows is not a challenge -/

/-- At a fixed challenge the argument accepts a pair of columns that are not
    rearrangements of each other.

    The witness is `[0, 5]` against `[1, 2]` at a challenge of one: the products
    are both six and the totals are five and three. Nothing about it is special
    to these numbers; a prover free to choose its columns after seeing the
    challenge solves one equation in as many unknowns as it likes. -/
theorem fixed_challenge_is_unsound : ¬ SoundAt 1 := by
  intro h
  have hprod : product 1 [0, 5] = product 1 [1, 2] := by
    simp [product]
  have := h [0, 5] [1, 2] hprod
  simp [total] at this

/-- And the same pair is rejected at a different challenge, so the argument is
    not broken, only unasked. At two the products are fourteen and twelve. -/
theorem the_witness_fails_at_a_different_challenge :
    product 2 [0, 5] ≠ product 2 [1, 2] := by
  simp [product]

/-- The shipped constants, kept here so the file names what it is about. -/
def shippedBeta : Nat := 5
def shippedGamma : Nat := 7

/-- The shipped beta is unsound as well: a fixed challenge is unsound whatever
    the constant is, and this exhibits it at the value that shipped. -/
theorem shipped_beta_is_unsound : ¬ SoundAt shippedBeta := by
  intro h
  have hprod : product shippedBeta [0, 11] = product shippedBeta [3, 5] := by
    simp [product, shippedBeta]
  have := h [0, 11] [3, 5] hprod
  simp [total] at this

/-! ### What a draw has to be -/

/-- How the challenge is produced from what has been committed. -/
abbrev Draw := Nat → Nat

/-- A draw that gives the same answer whatever was committed. -/
def Constant (d : Draw) : Prop := ∀ a b, d a = d b

/-- A draw that separates commitments: two different commitments never get the
    same challenge. -/
def Separating (d : Draw) : Prop := ∀ a b, a ≠ b → d a ≠ d b

/-- The circuit's constants, as a draw. -/
def constantDraw : Draw := fun _ => shippedBeta

theorem constantDraw_is_constant : Constant constantDraw := by
  intro a b
  rfl

/-- A constant draw hands out a challenge that is fixed before anything is
    committed, so a prover computes its columns knowing it. -/
theorem a_constant_draw_is_known_in_advance (d : Draw) (h : Constant d) :
    ∃ c, ∀ commitment, d commitment = c :=
  ⟨d 0, fun commitment => h commitment 0⟩

/-- And therefore a constant draw admits the forgery: the challenge is a fixed
    value, and a fixed value is unsound. -/
theorem a_constant_draw_admits_a_forgery (d : Draw) (hc : Constant d)
    (h1 : d 0 = 1) : ∀ commitment, ¬ SoundAt (d commitment) := by
  intro commitment
  rw [hc commitment 0, h1]
  exact fixed_challenge_is_unsound

/-- A separating draw cannot be answered in advance: for any challenge a prover
    might prepare for, there is at most one commitment that receives it, so
    committing to anything else changes the question. -/
theorem a_separating_draw_pins_one_commitment (d : Draw) (h : Separating d)
    (a b : Nat) (heq : d a = d b) : a = b := by
  by_cases hab : a = b
  · exact hab
  · exact absurd heq (h a b hab)

/-- A separating draw is not constant, provided there is more than one possible
    commitment. The two conditions are genuinely opposed rather than merely
    different. -/
theorem separating_is_not_constant (d : Draw) (hs : Separating d) :
    ¬ Constant d := by
  intro hc
  exact hs 0 1 (by omega) (hc 0 1)

/-! ### The transcript -/

/-- A round of the argument: what was committed, and the challenge that came
    back. -/
structure Round where
  commitment : Nat
  challenge : Nat
  deriving DecidableEq, Repr

/-- The round is honest when the challenge is what the draw gives for the
    commitment that was actually made. -/
def Honest (d : Draw) (r : Round) : Prop := r.challenge = d r.commitment

/-- Under a separating draw, a prover that changes its commitment gets a
    different challenge, so it cannot commit after choosing the challenge it
    wants to answer. -/
theorem changing_the_commitment_changes_the_challenge (d : Draw) (h : Separating d)
    (r s : Round) (hr : Honest d r) (hs : Honest d s)
    (hne : r.commitment ≠ s.commitment) : r.challenge ≠ s.challenge := by
  unfold Honest at hr hs
  rw [hr, hs]
  exact h _ _ hne

/-- Under a constant draw every honest round carries the same challenge, so the
    transcript contains no information about the commitment at all. That is the
    state the copy constraint was in. -/
theorem a_constant_draw_makes_the_transcript_empty (d : Draw) (h : Constant d)
    (r s : Round) (hr : Honest d r) (hs : Honest d s) : r.challenge = s.challenge := by
  unfold Honest at hr hs
  rw [hr, hs]
  exact h _ _

end Nonos.Stark.ChallengeDrawing
