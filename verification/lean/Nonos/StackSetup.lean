/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The initial user stack, and the sixteen bytes that make it fit.

`StackConfig::total_setup_size` is `strings_size + pointers_size +
STACK_ALIGNMENT`, and `StackSetup::setup` refuses the whole thing when that
exceeds the stack. Then the writer runs, and the writer does two things the sum
does not obviously account for: it rounds the pointer down to eight before the
pointer arrays, and it rounds it down to sixteen at the end. Both move the
pointer toward the bottom of the stack, and neither one checks whether there is
room to move.

So the `STACK_ALIGNMENT` term in that sum is not decoration, it is the budget for
those two roundings, and the question is whether it is big enough. It is, and
only just: the worst case is fifteen bytes against a budget of sixteen. This file
proves that, and then proves the three facts it rests on, because each of them is
something a later change could take away without touching the arithmetic.

The first is the order. `write_strings` runs before `write_pointers`, so the only
rounding that meets an arbitrary offset is the one to eight, which costs at most
seven. Everything after it is a whole number of eight-byte pushes, so the
rounding to sixteen meets an eight-aligned pointer and costs zero or eight, not
zero to fifteen. Swap the order and the budget is short.

The second is the constants. Seven plus eight is fifteen because the alignment is
sixteen and the pointer is eight. Raise the alignment to thirty-two and the same
sixteen-byte term has to cover thirty-one.

The third is what happens if it ever is short, which is the reason this is worth
proving rather than testing. `available_space` is
`current - stack_bottom` on `u64`. One byte below the bottom that subtraction
wraps, and it reports the entire address space as free, so every bound check
after it passes. The stack does not overflow into a refusal, it overflows into no
checks at all.
-/

namespace Nonos.StackSetup

/-! ### The constants the code ships -/

/-- `POINTER_SIZE`. -/
def pointerSize : Nat := 8

/-- `STACK_ALIGNMENT`, and the slack term in `total_setup_size`. One constant
    doing both jobs is exactly why this file exists. -/
def stackAlignment : Nat := 16

/-- `AuxEntry::SIZE`, and also what `push_auxv` writes per entry, two `u64`. -/
def auxSize : Nat := 16

/-- The auxv entry size has to be what the push writes, or the accounting is
    wrong before any alignment is considered. -/
theorem the_aux_size_is_two_pushes : auxSize = 2 * pointerSize := by
  unfold auxSize pointerSize; rfl

/-! ### The accounting

    `strings_size` is carried as one number because nothing below depends on how
    it splits between the arguments and the environment. -/

/-- `pointers_size`: the argc slot, a null-terminated pointer array each for the
    arguments and the environment, and the auxiliary vector. -/
def pointersSize (nargs nenv nauxv : Nat) : Nat :=
  pointerSize + (nargs + 1) * pointerSize + (nenv + 1) * pointerSize + nauxv * auxSize

/-- `total_setup_size`. -/
def totalSetupSize (strings nargs nenv nauxv : Nat) : Nat :=
  strings + pointersSize nargs nenv nauxv + stackAlignment

/-- Everything the writer pushes after the strings is a whole number of pointers,
    which is the fact the second rounding depends on. -/
theorem the_pushes_are_whole_pointers (nargs nenv nauxv : Nat) :
    pointersSize nargs nenv nauxv % pointerSize = 0 := by
  unfold pointersSize pointerSize auxSize
  omega

/-! ### What a rounding costs

    `align_to` clears the low bits, so it consumes exactly the pointer's offset
    within the alignment. -/

/-- The cost of rounding `x` down to a multiple of `a`. -/
def alignCost (x a : Nat) : Nat := x % a

/-- Rounding to eight costs at most seven, whatever the strings left behind. -/
theorem rounding_to_a_pointer_costs_at_most_seven (x : Nat) :
    alignCost x pointerSize ≤ 7 := by
  unfold alignCost pointerSize
  omega

/-- Rounding to sixteen an offset that is already a multiple of eight costs zero
    or eight. This is the step that makes the budget work, and it holds only
    because every push between the two roundings is a whole pointer. -/
theorem rounding_an_aligned_pointer_costs_zero_or_eight (x : Nat)
    (h : x % pointerSize = 0) :
    alignCost x stackAlignment = 0 ∨ alignCost x stackAlignment = 8 := by
  unfold alignCost stackAlignment
  unfold pointerSize at h
  omega

/-- Without that, the second rounding is worth as much as the first plus eight. -/
theorem rounding_an_arbitrary_pointer_costs_up_to_fifteen (x : Nat) :
    alignCost x stackAlignment ≤ 15 ∧ alignCost 15 stackAlignment = 15 := by
  refine ⟨?_, ?_⟩
  · unfold alignCost stackAlignment; omega
  · unfold alignCost stackAlignment; rfl

/-! ### The budget -/

/-- What the writer actually takes: the strings, the rounding to eight, the
    pointer region, and the rounding to sixteen. In the order the code runs
    them. -/
def consumed (strings nargs nenv nauxv a8 a16 : Nat) : Nat :=
  strings + a8 + pointersSize nargs nenv nauxv + a16

/-- The load-bearing one. Whatever the strings are and however many arguments,
    environment entries and auxiliary entries there are, the writer takes no more
    than `total_setup_size` allowed for. So a configuration that passed the guard
    in `setup` cannot leave the stack.

    Stated for every input rather than at a witness, because a bound that holds
    at the sizes someone tried is not a bound. -/
theorem the_slack_covers_both_roundings
    (strings nargs nenv nauxv a8 a16 : Nat)
    (h8 : a8 ≤ 7) (h16 : a16 ≤ 8) :
    consumed strings nargs nenv nauxv a8 a16 ≤ totalSetupSize strings nargs nenv nauxv := by
  unfold consumed totalSetupSize stackAlignment
  omega

/-- And it is that tight. The worst case is fifteen against sixteen, so there is
    one byte spare and no more. -/
theorem the_slack_has_exactly_one_byte_spare
    (strings nargs nenv nauxv : Nat) :
    consumed strings nargs nenv nauxv 7 8 + 1
      = totalSetupSize strings nargs nenv nauxv := by
  unfold consumed totalSetupSize stackAlignment
  omega

/-- Fourteen would not do, so the sixteen is not a round number that happens to
    be large. Written as the counterexample rather than as an inequality, so the
    shape that fails is visible. -/
theorem fourteen_bytes_of_slack_would_not_cover_it :
    consumed 0 0 0 0 7 8 > 0 + pointersSize 0 0 0 + 14 := by
  unfold consumed pointersSize pointerSize auxSize
  omega

/-- Raise the alignment and the same term has to cover thirty-one. The rounding
    to eight still costs up to seven, and rounding an eight-aligned pointer to
    thirty-two costs up to twenty-four.

    This is the change that breaks the file: `STACK_ALIGNMENT` is both the
    alignment the writer applies and the slack the sum reserves, and they stop
    agreeing the moment it moves. -/
theorem a_thirty_two_byte_alignment_needs_more_than_sixteen :
    (24 : Nat) % 32 = 24 ∧ 7 + 24 > stackAlignment := by
  refine ⟨rfl, ?_⟩
  unfold stackAlignment
  omega

/-- The order is load-bearing too. If the rounding to sixteen met an arbitrary
    offset instead of an eight-aligned one, the worst case would be twenty-two
    and the budget would be six bytes short. -/
theorem the_order_of_the_roundings_is_load_bearing :
    7 + 15 > stackAlignment ∧ 7 + 8 < stackAlignment + 1 := by
  unfold stackAlignment
  omega

/-! ### Why it matters that the budget holds

    `available_space` is a `u64` subtraction, so below the bottom it does not go
    negative, it goes enormous. -/

/-- 2^64. -/
def wordSpan : Nat := 18446744073709551616

theorem the_word_span_is_two_to_the_sixty_four : wordSpan = 2 ^ 64 := by
  unfold wordSpan; rfl

/-- A wrapping `u64` subtraction. -/
def wrapSub (a b : Nat) : Nat := (a + wordSpan - b) % wordSpan

/-- `available_space`. -/
def availableSpace (current bottom : Nat) : Nat := wrapSub current bottom

/-- Above the bottom it is the honest distance, which is what every bound check
    in the writer assumes it is. -/
theorem above_the_bottom_it_is_the_distance (current bottom : Nat)
    (hle : bottom ≤ current) (hb : bottom < wordSpan) (hc : current < wordSpan) :
    availableSpace current bottom = current - bottom := by
  unfold availableSpace wrapSub
  have h : current + wordSpan - bottom = (current - bottom) + wordSpan := by omega
  rw [h, Nat.add_mod_right, Nat.mod_eq_of_lt]
  omega

/-- One byte below it, the whole address space is free. Every check of the form
    `need > available_space()` passes from here on, so a single byte of overshoot
    does not become a refusal, it removes the refusals. -/
theorem one_byte_below_the_bottom_reports_the_whole_address_space
    (bottom : Nat) (h : 0 < bottom) (hb : bottom < wordSpan) :
    availableSpace (bottom - 1) bottom = wordSpan - 1 := by
  unfold availableSpace wrapSub
  have h1 : bottom - 1 + wordSpan - bottom = wordSpan - 1 := by omega
  rw [h1, Nat.mod_eq_of_lt]
  unfold wordSpan
  omega

/-- And it is monotone in the wrong direction below the bottom: the further past
    it the pointer is, the smaller the reported figure, but it stays above any
    size a real push asks for. Stated at a gap of thirty-two, which is larger
    than the biggest single push in the writer. -/
theorem a_small_overshoot_still_reports_more_than_any_push
    (bottom gap : Nat) (hg : 0 < gap) (hg32 : gap ≤ 32) (hb : gap ≤ bottom)
    (hbw : bottom < wordSpan) :
    availableSpace (bottom - gap) bottom ≥ wordSpan - 32 := by
  unfold availableSpace wrapSub
  have h1 : bottom - gap + wordSpan - bottom = wordSpan - gap := by omega
  rw [h1, Nat.mod_eq_of_lt]
  · unfold wordSpan; omega
  · unfold wordSpan; omega

/-! ### The bottom itself

    `StackSetup::new` computes `stack_top - stack_size` with the same wrapping
    subtraction and no check. -/

/-- The bottom, as `new` computes it. -/
def bottomOf (top size : Nat) : Nat := wrapSub top size

/-- A stack larger than the address its top sits at puts the bottom above the
    top, so the stack is inside out and `available_space` is wrong from the first
    call. No caller does this today: both pass a fixed `USER_STACK_SIZE` against
    a fixed base, and `with_stack_size` clamps the size from below. Nothing
    clamps it from above, which is the part written down here. -/
theorem a_stack_larger_than_its_top_wraps_the_bottom_above_it
    (top size : Nat) (h : top < size) (hs : size < wordSpan) (ht : 0 < top) :
    bottomOf top size > top := by
  unfold bottomOf wrapSub
  have h1 : top + wordSpan - size = wordSpan - (size - top) := by omega
  rw [h1, Nat.mod_eq_of_lt]
  · unfold wordSpan at hs ⊢; omega
  · unfold wordSpan at hs ⊢; omega

/-- The ordinary case, so the theorem above is not read as a claim about the
    shipping configuration. -/
theorem an_ordinary_stack_has_its_bottom_below_its_top
    (top size : Nat) (h : size ≤ top) (ht : top < wordSpan) (hs : 0 < size) :
    bottomOf top size < top ∧ bottomOf top size = top - size := by
  have heq : bottomOf top size = top - size := by
    unfold bottomOf wrapSub
    have h1 : top + wordSpan - size = (top - size) + wordSpan := by omega
    rw [h1, Nat.add_mod_right, Nat.mod_eq_of_lt]
    omega
  exact ⟨by omega, heq⟩

/-! ### Axiom profile -/

#print axioms Nonos.StackSetup.the_aux_size_is_two_pushes
#print axioms Nonos.StackSetup.the_pushes_are_whole_pointers
#print axioms Nonos.StackSetup.rounding_to_a_pointer_costs_at_most_seven
#print axioms Nonos.StackSetup.rounding_an_aligned_pointer_costs_zero_or_eight
#print axioms Nonos.StackSetup.rounding_an_arbitrary_pointer_costs_up_to_fifteen
#print axioms Nonos.StackSetup.the_slack_covers_both_roundings
#print axioms Nonos.StackSetup.the_slack_has_exactly_one_byte_spare
#print axioms Nonos.StackSetup.fourteen_bytes_of_slack_would_not_cover_it
#print axioms Nonos.StackSetup.a_thirty_two_byte_alignment_needs_more_than_sixteen
#print axioms Nonos.StackSetup.the_order_of_the_roundings_is_load_bearing
#print axioms Nonos.StackSetup.the_word_span_is_two_to_the_sixty_four
#print axioms Nonos.StackSetup.above_the_bottom_it_is_the_distance
#print axioms Nonos.StackSetup.one_byte_below_the_bottom_reports_the_whole_address_space
#print axioms Nonos.StackSetup.a_small_overshoot_still_reports_more_than_any_push
#print axioms Nonos.StackSetup.a_stack_larger_than_its_top_wraps_the_bottom_above_it
#print axioms Nonos.StackSetup.an_ordinary_stack_has_its_bottom_below_its_top

end Nonos.StackSetup
