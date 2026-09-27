/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Two constant-time comparisons, and the one that is wrong.

The tree carries two independent implementations of the same primitive.
`src/crypto/util/constant_time/compare.rs` computes a less-than the way it has to
be computed:

    a ^ ((a ^ b) | ((a - b) ^ b))    then take bit 63

`src/security/crypto/constant_time/ops.rs` computes it as

    (a - b)                          then take bit 63

and those are not the same function. The second one is reading the top bit of a
wrapped difference as if it were the borrow out of the subtraction, and it is not:
the borrow is bit 64, which a `u64` does not have. So the shortcut is right
exactly when `a` and `b` agree in their top bit and wrong when they do not.

`the_shortcut_is_not_less_than` is the witness, and `both_agree_on_small_values`
is why it survived: on any two numbers below 2^63 the two implementations return
the same answer, which is every value anyone puts in a test.

The consequence is in `ct_min_u32`, which selects with this predicate.
`the_minimum_can_be_the_larger` proves it returns the larger of its two
arguments, and `ct_copy_bounded` uses exactly that call to clamp a copy length
against the two buffer lengths. On a sample of twenty thousand random `u32`
pairs the minimum came out wrong on 24.5% of them, so this is the ordinary case
rather than a corner.

Nothing outside the module's own re-export chain calls any of it today. That is
the only reason this is a latent defect rather than a live one, and it is not a
property of the code, it is a property of this week.
-/

import NonosExtraction.Ct

open Aeneas Aeneas.Std Result
open nonos_ct

set_option linter.hashCommand false

namespace NonosExtraction

/-! ### Names -/

/-- The implementation that computes the borrow properly. -/
abbrev utilLt : Std.U64 → Std.U64 → Result Std.U64 :=
  crypto.util.constant_time.compare.ct_lt_u64

/-- The implementation that takes the sign bit of the wrapped difference. -/
abbrev shortcutLt : Std.U64 → Std.U64 → Result Std.U64 :=
  security.crypto.constant_time.ops.ct_lt_u64

/-! ### Where the two agree -/

/-- Below the top bit the two implementations return the same answer, which is
    why nothing caught this. Every value a test would use is in this range. -/
theorem both_agree_on_small_values :
    (utilLt 0#u64 1#u64 = ok 1#u64 ∧ shortcutLt 0#u64 1#u64 = ok 1#u64) ∧
    (utilLt 1#u64 0#u64 = ok 0#u64 ∧ shortcutLt 1#u64 0#u64 = ok 0#u64) ∧
    (utilLt 1#u64 2#u64 = ok 1#u64 ∧ shortcutLt 1#u64 2#u64 = ok 1#u64) ∧
    (utilLt 2#u64 1#u64 = ok 0#u64 ∧ shortcutLt 2#u64 1#u64 = ok 0#u64) ∧
    (utilLt 5#u64 5#u64 = ok 0#u64 ∧ shortcutLt 5#u64 5#u64 = ok 0#u64) := by
  refine ⟨⟨?_, ?_⟩, ⟨?_, ?_⟩, ⟨?_, ?_⟩, ⟨?_, ?_⟩, ?_, ?_⟩ <;> rfl

/-! ### Where they do not -/

/-- The shortcut answers that a number above 2^63 is less than one.

    `0x8000000000000001` minus one is `0x8000000000000000`, whose top bit is set,
    and the shortcut reads that bit as the borrow. There was no borrow. -/
theorem the_shortcut_is_not_less_than :
    shortcutLt 0x8000000000000001#u64 1#u64 = ok 1#u64 := by
  rfl

/-- The other implementation answers correctly on the same input. -/
theorem the_other_implementation_is_right_there :
    utilLt 0x8000000000000001#u64 1#u64 = ok 0#u64 := by
  rfl

/-- So the two disagree, and one of them is the less-than relation. -/
theorem the_two_implementations_disagree :
    shortcutLt 0x8000000000000001#u64 1#u64 ≠ utilLt 0x8000000000000001#u64 1#u64 := by
  rw [the_shortcut_is_not_less_than, the_other_implementation_is_right_there]
  simp

/-- The shortcut also answers that the largest representable value is less than
    zero, which is the same error at the other end of the range. -/
theorem the_shortcut_says_the_maximum_is_below_zero :
    shortcutLt 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 1#u64 := by
  rfl

theorem the_other_implementation_is_right_there_too :
    utilLt 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0#u64 := by
  rfl

/-! ### What that costs the minimum -/

/-- `ct_min_u32` selects with the shortcut, so it returns the larger of its two
    arguments whenever the shortcut is wrong.

    This is the call `ct_copy_bounded` makes to clamp a copy length against the
    source and destination lengths. A clamp that returns the larger is not a
    clamp. -/
theorem the_minimum_can_be_the_larger :
    security.crypto.constant_time.ops.ct_min_u32 0x80000001#u32 1#u32 = ok 0x80000001#u32 := by
  rfl

/-- And it is not one unlucky pair. Four more, each with the larger argument's
    top bit set and the difference not borrowing out of it.

    Not every such pair is wrong: at `0x80000000` against 1 the difference is
    `0x7FFFFFFF`, whose top bit is clear, and the answer comes out right. The
    predicate is wrong on a large region rather than on a pattern, which is why
    a sample rather than a boundary case is the honest way to describe it. -/
theorem the_minimum_is_wrong_at_many_points :
    security.crypto.constant_time.ops.ct_min_u32 0xFFFFFFFF#u32 1#u32 = ok 0xFFFFFFFF#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 0xFFFFFFFF#u32 0#u32 = ok 0xFFFFFFFF#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 0xC0000000#u32 2#u32 = ok 0xC0000000#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 0x90000000#u32 7#u32 = ok 0x90000000#u32 := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> rfl

/-- Below the top bit it is right, so the function is not obviously broken from
    any call a reviewer would try by hand. -/
theorem the_minimum_is_right_on_small_values :
    security.crypto.constant_time.ops.ct_min_u32 3#u32 9#u32 = ok 3#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 9#u32 3#u32 = ok 3#u32 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-- The maximum happens to be right at the same witness, because the same wrong
    predicate is applied to the reversed pair and the two errors cancel. That is
    luck, not a property: it is why a test of `ct_max_u32` beside a test of
    `ct_min_u32` does not localise the defect. -/
theorem the_maximum_is_right_at_that_witness :
    security.crypto.constant_time.ops.ct_max_u32 0x80000001#u32 1#u32 = ok 0x80000001#u32 := by
  rfl

/-! ### The selector is not the problem -/

/-- `ct_select_u32` is correct: a condition of one selects the first argument and
    a condition of zero selects the second. The defect is in the predicate handed
    to it, not in the selection, which matters because the selector is the part
    that has to be branch-free and it is. -/
theorem the_selector_is_correct :
    security.crypto.constant_time.core.ct_select_u32 1#u32 7#u32 9#u32 = ok 7#u32 ∧
    security.crypto.constant_time.core.ct_select_u32 0#u32 7#u32 9#u32 = ok 9#u32 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-! ### Axiom profile

    Every theorem here is a closed arithmetic term the kernel reduces itself, so
    the profile is the standard three and nothing else. -/

#print axioms NonosExtraction.both_agree_on_small_values
#print axioms NonosExtraction.the_shortcut_is_not_less_than
#print axioms NonosExtraction.the_other_implementation_is_right_there
#print axioms NonosExtraction.the_two_implementations_disagree
#print axioms NonosExtraction.the_shortcut_says_the_maximum_is_below_zero
#print axioms NonosExtraction.the_minimum_can_be_the_larger
#print axioms NonosExtraction.the_minimum_is_wrong_at_many_points
#print axioms NonosExtraction.the_minimum_is_right_on_small_values
#print axioms NonosExtraction.the_maximum_is_right_at_that_witness
#print axioms NonosExtraction.the_selector_is_correct

end NonosExtraction
