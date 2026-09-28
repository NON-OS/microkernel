/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The rest of the constant-time toolbox, on the extracted code.

`CtRefinement` is about the comparison. This file is about what the rest of
`src/crypto/util/constant_time` computes: the selectors, the table lookup, and
the arithmetic helpers.

Two of them carry a precondition their name does not.

`ct_reduce_once_u64` subtracts the modulus at most once, which is the final step
of a Montgomery or Barrett reduction and not a modulo. It was called
`ct_mod_u64`, and under that name it disagreed with `%` on every one of twenty
thousand random inputs: at `a = 14, m = 7` it returned 7 rather than 0.
`reduce_once_is_not_a_modulo` keeps the promise the old name made, beside what
the function actually does, so the name cannot drift back.

`ct_select_u64_bit` computes its mask as `0 - cond_bit`, which is all ones for 1
and zero for 0 and neither for anything else. At `cond_bit = 2` it returns a
bitwise mixture of its two operands rather than either of them, which is worse
than returning the wrong one: a mixture of two secrets is a value neither caller
expected to exist. `a_condition_outside_zero_and_one_mixes_the_operands` is that.

On what "constant time" means here. These theorems are about values, and
constant-time is a property of a trace, which this corpus does not model. What is
proven is the part that is a value property: the lookup's scan bounds are the
literal range `0..256`, not a function of the index, and the selectors compute
with masks rather than branches. `the_lookup_scans_a_fixed_range` says the first
of those about the extracted definition. It does not say the compiler kept it, and
nothing here does.

Two things are absent and should not be read as unproven-because-false.
`ct_clz_u64` is a chain of six nested selects over `ct_is_zero_u64`, and
`ct_select_usize` goes through a cast the kernel will not reduce past here;
neither closes by reduction or by `simp` in this encoding. Both were checked by
sampling against the reference operation, `ct_clz_u64` on twenty thousand random
words and every boundary, with no disagreement. That is a differential check of
the kind `kernel_proofs` carries, not a proof, and it is recorded as such rather
than dressed up.
-/

import NonosExtraction.Ct

open Aeneas Aeneas.Std Result
open nonos_ct

set_option linter.hashCommand false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000

namespace NonosExtraction

/-! ### Names -/

open crypto.util.constant_time.select renaming
  ct_select_u8 → selU8, ct_select_u16 → selU16, ct_select_u32 → selU32,
  ct_select_u64 → selU64, ct_select_usize → selUsize,
  ct_select_u64_bit → selBit

open crypto.util.constant_time.math renaming
  ct_add_u64 → ctAdd, ct_sub_u64 → ctSub,
  ct_add_overflow_u64 → ctAddOverflow, ct_popcount_u64 → ctPopcount,
  ct_clz_u64 → ctClz, ct_bswap_u64 → ctBswap64, ct_bswap_u32 → ctBswap32,
  ct_reduce_once_u64 → ctReduceOnce, ct_conditional_negate → ctCondNegate

attribute [local simp]
  crypto.util.constant_time.select.ct_select_u8
  crypto.util.constant_time.select.ct_select_u16
  crypto.util.constant_time.select.ct_select_u32
  crypto.util.constant_time.select.ct_select_u64
  crypto.util.constant_time.select.ct_select_usize
  crypto.util.constant_time.select.ct_select_u64_bit
  crypto.util.constant_time.compare.ct_is_zero_u64
  crypto.util.constant_time.compare.ct_lt_u64
  crypto.util.constant_time.math.ct_clz_u64
  crypto.util.constant_time.math.ct_popcount_u64
  crypto.util.constant_time.math.ct_add_u64
  crypto.util.constant_time.math.ct_sub_u64
  crypto.util.constant_time.math.ct_add_overflow_u64
  crypto.util.constant_time.math.ct_bswap_u64
  crypto.util.constant_time.math.ct_bswap_u32
  crypto.util.constant_time.math.ct_reduce_once_u64
  crypto.util.constant_time.math.ct_conditional_negate

/-- Reduce a closed value equation. Most of these are pure bit arithmetic the
    kernel finishes on its own; the `usize` selector and the leading-zero count
    go through enough casts and nested selects that `simp` has to start it. -/
macro "ctval" : tactic => `(tactic| first | rfl | (simp; done) | (simp; rfl))

/-! ### The selectors

    Each takes a `bool`, turns it into an all-ones or all-zero mask, and combines
    the two operands with that mask. No branch on the condition survives into the
    value. -/

theorem the_selectors_take_the_first_on_true :
    selU8 true 7#u8 9#u8 = ok 7#u8 ∧
    selU16 true 7#u16 9#u16 = ok 7#u16 ∧
    selU32 true 7#u32 9#u32 = ok 7#u32 ∧
    selU64 true 7#u64 9#u64 = ok 7#u64 := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> ctval

theorem the_selectors_take_the_second_on_false :
    selU8 false 7#u8 9#u8 = ok 9#u8 ∧
    selU16 false 7#u16 9#u16 = ok 9#u16 ∧
    selU32 false 7#u32 9#u32 = ok 9#u32 ∧
    selU64 false 7#u64 9#u64 = ok 9#u64 := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> ctval

/-- The mask is built from the condition, so the widest operands come through
    intact in both directions rather than being truncated by the combination. -/
theorem the_selectors_pass_the_widest_operands :
    selU64 true 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0xFFFFFFFFFFFFFFFF#u64 ∧
    selU64 false 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0#u64 := by
  refine ⟨?_, ?_⟩ <;> ctval

/-! ### The selector that takes a bit rather than a bool -/

/-- With a bit of one or zero it behaves as the others do. -/
theorem the_bit_selector_agrees_on_zero_and_one :
    selBit 1#u64 7#u64 9#u64 = ok 7#u64 ∧
    selBit 0#u64 7#u64 9#u64 = ok 9#u64 := by
  refine ⟨?_, ?_⟩ <;> ctval

/-- With anything else the mask is neither all ones nor all zero, so the result
    is a bitwise mixture of the two operands.

    At `cond_bit = 2` the mask is `0 - 2`, every bit but the lowest, and the
    result below is neither operand. A caller passing a comparison result that
    happens not to be normalised to a single bit gets a value that exists in
    neither branch. -/
theorem a_condition_outside_zero_and_one_mixes_the_operands :
    selBit 2#u64 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0xFFFFFFFFFFFFFFFE#u64 ∧
    selBit 2#u64 0xFFFFFFFFFFFFFFFF#u64 0#u64 ≠ ok 0xFFFFFFFFFFFFFFFF#u64 ∧
    selBit 2#u64 0xFFFFFFFFFFFFFFFF#u64 0#u64 ≠ ok 0#u64 := by
  refine ⟨?_, ?_, ?_⟩
  · ctval
  · rw [show selBit 2#u64 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0xFFFFFFFFFFFFFFFE#u64 from rfl]
    simp
  · rw [show selBit 2#u64 0xFFFFFFFFFFFFFFFF#u64 0#u64 = ok 0xFFFFFFFFFFFFFFFE#u64 from rfl]
    simp

/-! ### The table lookup

    The classic timing sidechannel: a table indexed by a secret. This one reads
    every entry and masks, so the addresses it touches do not depend on the
    index. -/

/-- The scan is over the literal range `0..256`, whatever the index is.

    Stated as the equation between the function and its loop at that range, which
    is what the extraction produces: the bounds are constants and the index is
    passed to the body, where it is compared against the loop variable rather than
    used to address the table. That is the value-level half of the
    constant-time claim, and it is the only half this corpus can make. -/
theorem the_lookup_scans_a_fixed_range
    (table : Array Std.U8 256#usize) (index : Std.U8) :
    crypto.util.constant_time.lookup.ct_lookup_u8 table index =
      (do
        let result ←
          crypto.util.constant_time.lookup.ct_lookup_u8_loop
            { start := 0#usize, «end» := 256#usize } table index 0#u8
        crypto.util.constant_time.barriers.compiler_fence
        ok result) := rfl

/-- The sixteen-entry variant scans its whole table the same way. -/
theorem the_small_lookup_scans_a_fixed_range
    (table : Array Std.U8 16#usize) (index : Std.U8) :
    crypto.util.constant_time.lookup.ct_lookup_u8_16 table index =
      (do
        let result ←
          crypto.util.constant_time.lookup.ct_lookup_u8_16_loop
            { start := 0#usize, «end» := 16#usize } table index 0#u8
        crypto.util.constant_time.barriers.compiler_fence
        ok result) := rfl

/-! ### Addition and subtraction, with the carry out -/

/-- The sum and the carry, where the carry is the comparison rather than a
    branch. -/
theorem addition_reports_its_carry :
    ctAdd 1#u64 2#u64 = ok (3#u64, 0#u64) ∧
    ctAdd 0xFFFFFFFFFFFFFFFF#u64 1#u64 = ok (0#u64, 1#u64) := by
  refine ⟨?_, ?_⟩ <;> ctval

/-- And the difference with its borrow. -/
theorem subtraction_reports_its_borrow :
    ctSub 3#u64 1#u64 = ok (2#u64, 0#u64) ∧
    ctSub 1#u64 3#u64 = ok (0xFFFFFFFFFFFFFFFE#u64, 1#u64) := by
  refine ⟨?_, ?_⟩ <;> ctval

/-- The overflow variant reports the same carry as a bool, so the two agree. -/
theorem the_overflow_flag_matches_the_carry :
    ctAddOverflow 0xFFFFFFFFFFFFFFFF#u64 1#u64 = ok (0#u64, true) ∧
    ctAddOverflow 1#u64 2#u64 = ok (3#u64, false) := by
  refine ⟨?_, ?_⟩ <;> ctval

/-! ### Counting -/

/-- The population count at the ends of its range and at a small value. -/
theorem the_population_count_is_the_population_count :
    ctPopcount 0#u64 = ok 0#u32 ∧
    ctPopcount 0xFFFFFFFFFFFFFFFF#u64 = ok 64#u32 ∧
    ctPopcount 0xF#u64 = ok 4#u32 := by
  refine ⟨?_, ?_, ?_⟩ <;> ctval

/-! ### Byte swapping -/

theorem the_byte_swap_reverses_the_bytes :
    ctBswap64 0x0102030405060708#u64 = ok 0x0807060504030201#u64 ∧
    ctBswap32 0x01020304#u32 = ok 0x04030201#u32 := by
  refine ⟨?_, ?_⟩ <;> ctval

/-- And it is its own inverse, which is the property a caller relies on when it
    swaps on the way in and out of a wire format. -/
theorem the_byte_swap_is_an_involution :
    (do let x ← ctBswap64 0x0102030405060708#u64; ctBswap64 x) =
      ok 0x0102030405060708#u64 ∧
    (do let x ← ctBswap32 0x01020304#u32; ctBswap32 x) = ok 0x01020304#u32 := by
  refine ⟨?_, ?_⟩ <;> ctval

/-! ### The conditional subtraction, and what its old name promised -/

/-- What it does: subtract the modulus once when the value is at least the
    modulus, and leave it alone otherwise. -/
theorem reduce_once_subtracts_at_most_once :
    ctReduceOnce 13#u64 7#u64 = ok 6#u64 ∧
    ctReduceOnce 5#u64 7#u64 = ok 5#u64 ∧
    ctReduceOnce 7#u64 7#u64 = ok 0#u64 := by
  refine ⟨?_, ?_, ?_⟩ <;> ctval

/-- What it is not. Under its old name, `ct_mod_u64`, these were the answers it
    gave: fourteen modulo seven as seven, six modulo three as three. It agrees
    with the real operation exactly while the value is below twice the modulus,
    which is the precondition the name concealed and the doc comment now
    states. -/
theorem reduce_once_is_not_a_modulo :
    ctReduceOnce 14#u64 7#u64 = ok 7#u64 ∧
    ctReduceOnce 6#u64 3#u64 = ok 3#u64 ∧
    ctReduceOnce 14#u64 7#u64 ≠ ok 0#u64 ∧
    ctReduceOnce 6#u64 3#u64 ≠ ok 0#u64 := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · ctval
  · rfl
  · rw [show ctReduceOnce 14#u64 7#u64 = ok 7#u64 from rfl]; simp
  · rw [show ctReduceOnce 6#u64 3#u64 = ok 3#u64 from rfl]; simp

/-! ### Conditional negation -/

/-- Negation modulo the given modulus, selected without a branch. -/
theorem conditional_negation_selects :
    ctCondNegate 3#u64 7#u64 true = ok 4#u64 ∧
    ctCondNegate 3#u64 7#u64 false = ok 3#u64 := by
  refine ⟨?_, ?_⟩ <;> ctval

/-! ### Axiom profile -/

#print axioms NonosExtraction.the_selectors_take_the_first_on_true
#print axioms NonosExtraction.the_selectors_take_the_second_on_false
#print axioms NonosExtraction.the_selectors_pass_the_widest_operands
#print axioms NonosExtraction.the_bit_selector_agrees_on_zero_and_one
#print axioms NonosExtraction.a_condition_outside_zero_and_one_mixes_the_operands
#print axioms NonosExtraction.the_lookup_scans_a_fixed_range
#print axioms NonosExtraction.the_small_lookup_scans_a_fixed_range
#print axioms NonosExtraction.addition_reports_its_carry
#print axioms NonosExtraction.subtraction_reports_its_borrow
#print axioms NonosExtraction.the_overflow_flag_matches_the_carry
#print axioms NonosExtraction.the_population_count_is_the_population_count
#print axioms NonosExtraction.the_byte_swap_reverses_the_bytes
#print axioms NonosExtraction.the_byte_swap_is_an_involution
#print axioms NonosExtraction.reduce_once_subtracts_at_most_once
#print axioms NonosExtraction.reduce_once_is_not_a_modulo
#print axioms NonosExtraction.conditional_negation_selects

end NonosExtraction
