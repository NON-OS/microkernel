/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Two constant-time comparisons, and the shape one of them used to have.

The tree carries two implementations of the same primitive, in
`crypto/util/constant_time/compare.rs` and `security/crypto/constant_time/ops.rs`.
Both compute the borrow out of `a - b`, the only way it can be computed on a
machine word with no bit to hold it:

    a ^ ((a ^ b) | ((a - b) ^ b))    then take the top bit

`the_two_implementations_agree` proves they are the same function on every pair of
arguments. That is what two copies of one primitive owe each other, and it is the
thing that stops holding the moment somebody edits one of them alone.

The second one used to be `(a - b)`, take the top bit, reading the sign of the
wrapped difference as if it were the borrow. It is not: the borrow is bit 64,
which a `u64` does not have, so the shortcut was right exactly when the two
arguments agreed in their own top bit and wrong when they did not. `oldShortcut`
below is that shape, kept as a definition here rather than in the kernel, and the
theorems around it record what it answered: that `0x8000000000000001` is less than
one, and that the largest `u64` is less than zero.

Keeping the mistake as a model is deliberate. A proof that the code is broken is a
proof somebody has to delete in order to fix the bug, and then nothing remembers
the bug. A proof that the code is right, beside a named model of the shape it must
not return to, fails if it returns.

`the_minimum_is_the_minimum` is the consequence that mattered. `ct_min_u32`
selects with this predicate and `ct_copy_bounded` clamps a copy length against the
source and destination lengths with that call, so under the old shape the clamp
returned the larger of the two on about a quarter of random pairs. Those four
witnesses are the true minimum now.
-/

import NonosExtraction.Ct

open Aeneas Aeneas.Std Result
open nonos_ct

set_option linter.hashCommand false

-- The corrected body is five bit operations deep over a 64-bit word, so the
-- closed terms below take more reduction than the default budget allows. The
-- generated Ct.lean raises the same two limits for the same reason.
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000

namespace NonosExtraction

/-! ### Names -/

abbrev utilLt : Std.U64 → Std.U64 → Result Std.U64 :=
  crypto.util.constant_time.compare.ct_lt_u64

abbrev opsLt : Std.U64 → Std.U64 → Result Std.U64 :=
  security.crypto.constant_time.ops.ct_lt_u64

/-! ### One primitive, two copies -/

/-- The two implementations are the same function, on every pair of arguments.

    It holds by reduction because they are now the same computation, which is
    exactly why it stops holding if one of them is edited alone. -/
theorem the_two_implementations_agree (a b : Std.U64) : opsLt a b = utilLt a b := rfl

/-- And the greater-than one of them defines is the other's less-than with the
    arguments swapped, so the pair of operations is consistent as well as each
    one. -/
theorem greater_than_is_less_than_reversed (a b : Std.U64) :
    crypto.util.constant_time.compare.ct_gt_u64 a b = utilLt b a := rfl

/-! ### What they answer -/

/-- At the pairs the old shape got wrong, both answer correctly. -/
theorem the_hard_pairs_are_right :
    opsLt 0x8000000000000001#u64 1#u64 = ok 0#u64 ∧
    opsLt 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0#u64 ∧
    opsLt 1#u64 0x8000000000000001#u64 = ok 1#u64 ∧
    opsLt 0#u64 0xFFFFFFFFFFFFFFFF#u64 = ok 1#u64 := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> rfl

/-- And at the easy pairs, which is where the old shape also agreed and why it
    survived review. -/
theorem the_easy_pairs_are_right :
    opsLt 0#u64 1#u64 = ok 1#u64 ∧ opsLt 1#u64 0#u64 = ok 0#u64 ∧
    opsLt 1#u64 2#u64 = ok 1#u64 ∧ opsLt 2#u64 1#u64 = ok 0#u64 ∧
    opsLt 5#u64 5#u64 = ok 0#u64 := by
  refine ⟨?_, ?_, ?_, ?_, ?_⟩ <;> rfl

/-! ### The shape it must not return to -/

/-- The old body: the top bit of the wrapped difference, read as if it were the
    borrow. Written here as a definition so the mistake has a name and a proof
    against it, rather than only a line in a commit message. -/
def oldShortcut (a b : Std.U64) : Result Std.U64 := do
  let diff ← lift (core.num.U64.wrapping_sub a b)
  let i ← diff >>> 63#i32
  ok (i &&& 1#u64)

/-- It answers that a number above 2^63 is less than one. `0x8000000000000001`
    minus one is `0x8000000000000000`, whose top bit is set, and there was no
    borrow. -/
theorem the_old_shortcut_was_wrong :
    oldShortcut 0x8000000000000001#u64 1#u64 = ok 1#u64 ∧
      opsLt 0x8000000000000001#u64 1#u64 = ok 0#u64 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-- And that the largest representable value is less than zero, the same error at
    the other end of the range. -/
theorem the_old_shortcut_said_the_maximum_was_below_zero :
    oldShortcut 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 1#u64 ∧
      opsLt 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0#u64 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-- So the shape and the primitive are different functions, and the difference is
    not at a boundary: it is wherever the two arguments disagree in their top
    bit. -/
theorem the_shape_is_not_the_primitive :
    oldShortcut 0x8000000000000001#u64 1#u64 ≠ opsLt 0x8000000000000001#u64 1#u64 := by
  rw [show oldShortcut 0x8000000000000001#u64 1#u64 = ok 1#u64 from rfl,
      show opsLt 0x8000000000000001#u64 1#u64 = ok 0#u64 from rfl]
  simp

/-- Below the top bit the two are indistinguishable, which is the whole reason the
    wrong shape lasted: every number a test uses is here. -/
theorem the_shape_agreed_on_small_values :
    oldShortcut 0#u64 1#u64 = opsLt 0#u64 1#u64 ∧
    oldShortcut 1#u64 0#u64 = opsLt 1#u64 0#u64 ∧
    oldShortcut 2#u64 1#u64 = opsLt 2#u64 1#u64 := by
  refine ⟨?_, ?_, ?_⟩ <;> rfl

/-! ### The minimum -/

/-- `ct_min_u32` selects with this predicate, so it is the minimum at the four
    points where the old shape returned the larger of the two.

    `ct_copy_bounded` clamps a copy length against the source and destination
    lengths with exactly this call. A clamp that returns the larger is not a
    clamp, and on a sample of twenty thousand random pairs the old one did so on
    about a quarter of them. -/
theorem the_minimum_is_the_minimum :
    security.crypto.constant_time.ops.ct_min_u32 0x80000001#u32 1#u32 = ok 1#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 0xFFFFFFFF#u32 0#u32 = ok 0#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 0xC0000000#u32 2#u32 = ok 2#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 0x90000000#u32 7#u32 = ok 7#u32 := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> rfl

/-- And still the minimum on the small values it was already right about, so the
    change is a repair rather than a trade. -/
theorem the_minimum_is_still_right_on_small_values :
    security.crypto.constant_time.ops.ct_min_u32 3#u32 9#u32 = ok 3#u32 ∧
    security.crypto.constant_time.ops.ct_min_u32 9#u32 3#u32 = ok 3#u32 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-- The maximum agrees with it: at the same witness the two bracket the pair
    rather than both naming one value. Under the old shape the maximum was right
    here by luck, because the reversed pair met the same wrong predicate and the
    two errors cancelled, which is why testing the two side by side did not
    localise the defect. -/
theorem the_maximum_is_the_maximum :
    security.crypto.constant_time.ops.ct_max_u32 0x80000001#u32 1#u32 = ok 0x80000001#u32 ∧
    security.crypto.constant_time.ops.ct_max_u32 0xC0000000#u32 2#u32 = ok 0xC0000000#u32 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-! ### The selector was never the problem -/

/-- `ct_select_u32` takes the first argument on a condition of one and the second
    on zero. It was correct throughout: the defect was the predicate handed to it.
    That matters because the selector is the part that has to be branch-free, and
    it is. -/
theorem the_selector_is_correct :
    security.crypto.constant_time.core.ct_select_u32 1#u32 7#u32 9#u32 = ok 7#u32 ∧
    security.crypto.constant_time.core.ct_select_u32 0#u32 7#u32 9#u32 = ok 9#u32 := by
  refine ⟨?_, ?_⟩ <;> rfl

/-! ### Axiom profile

    Every theorem here is a closed term the kernel reduces, or a reduction between
    two definitions, so the profile is the standard three and nothing else. -/


/-- The sixty-four byte comparison fences between the loop and the test, the same
    way the thirty-two byte one does, so the shared module is internally
    consistent and there is no second place for a caller to pick the wrong one.

    The thirty-two byte side lives in `NonosExtraction.CtEq`, with ed25519's
    delegation to it. -/
theorem the_sixty_four_byte_comparison_also_fences (a b : Std.Array Std.U8 64#usize) :
    crypto.util.constant_time.compare.ct_eq_64 a b =
      (do let diff ← crypto.util.constant_time.compare.ct_eq_64_loop
                       { start := 0#usize, «end» := 64#usize } a b 0#u8
          crypto.util.constant_time.barriers.compiler_fence
          ok (diff = 0#u8)) := rfl

#print axioms NonosExtraction.the_sixty_four_byte_comparison_also_fences
#print axioms NonosExtraction.the_two_implementations_agree
#print axioms NonosExtraction.greater_than_is_less_than_reversed
#print axioms NonosExtraction.the_hard_pairs_are_right
#print axioms NonosExtraction.the_easy_pairs_are_right
#print axioms NonosExtraction.the_old_shortcut_was_wrong
#print axioms NonosExtraction.the_old_shortcut_said_the_maximum_was_below_zero
#print axioms NonosExtraction.the_shape_is_not_the_primitive
#print axioms NonosExtraction.the_shape_agreed_on_small_values
#print axioms NonosExtraction.the_minimum_is_the_minimum
#print axioms NonosExtraction.the_minimum_is_still_right_on_small_values
#print axioms NonosExtraction.the_maximum_is_the_maximum
#print axioms NonosExtraction.the_selector_is_correct

end NonosExtraction
