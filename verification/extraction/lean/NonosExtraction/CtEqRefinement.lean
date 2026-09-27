/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Two constant-time comparisons of thirty-two bytes, and the difference a value
proof cannot see.

The kernel has `ct_eq_32` twice. One is in `crypto/util/constant_time/compare.rs`
and is what the rest of the tree reaches for, including secure boot's signature
check. The other is `pub(crate)` inside `crypto/asymmetric/ed25519/field/`, and
ed25519 uses its own rather than the shared one.

They are the same function. `the_two_loops_are_one_function` proves the
accumulating loops are definitionally identical, so neither implementation can
return a different answer from the other for any pair of inputs.

They are not the same code. The shared one calls `compiler_fence` between the
loop and the comparison against zero; the ed25519 one does not.
`the_only_difference_is_the_fence` is that, stated as an equation: the shared
function is the ed25519 function with a fence spliced into the middle.

That fence is the whole point of the primitive. Nothing in the loop's value
depends on it, which is precisely why the compiler is free to remove the loop's
data independence without it: an optimiser may notice that `diff` can only grow
and stop early once it is non-zero, and the running time then depends on where
the first differing byte is. The fence is what forbids that.

So this file proves the two agree and cannot prove the thing that matters. That
is not a gap in the effort, it is the shape of the problem: a theorem about the
value a function returns says nothing about how long it took. Stating that
plainly is the honest version of "we proved the constant-time comparison".

Where the unfenced copy runs: `fe_equal` and `fe_is_zero` in
`crypto/asymmetric/ed25519/field/compare.rs`, called from
`point/pack.rs` during point decompression and from `point/scalarmult.rs` for the
small order check. Both run during signature verification, on points an attacker
supplies.

The fix is one line, and it is not ours to make in a verification branch: have
ed25519 call the shared `ct_eq_32` rather than carry its own. The theorem below
is what makes that safe to do, since it shows the substitution changes no value.
-/

import NonosExtraction.Ct
import NonosExtraction.EdField

open Aeneas Aeneas.Std Result

set_option linter.hashCommand false
set_option maxRecDepth 20000

namespace NonosExtraction.CtEq

open nonos_ct.crypto.util.constant_time renaming
  compare.ct_eq_32 → sharedEq, compare.ct_eq_32_loop → sharedLoop,
  barriers.compiler_fence → fence
open nonos_ct.crypto.util.constant_time renaming
  compare.ct_eq_64 → sharedEq64, compare.ct_eq_64_loop → sharedLoop64
open nonos_ed_field renaming
  field.compare.ct_eq_32 → ed25519Eq, field.compare.ct_eq_32_loop → ed25519Loop,
  ed25519_ct_eq_32 → ed25519Wrapper

/-! ### The two are one function -/

/-- The accumulating loops are the same term. Two copies of a primitive owe each
    other this, and here it holds by definition rather than by argument, because
    the bodies are identical: both walk the same range, index both arrays, xor
    and accumulate with the same operations. -/
theorem the_two_loops_are_one_function
    (iter : core.ops.range.Range Std.Usize)
    (a b : Array Std.U8 32#usize) (d : Std.U8) :
    sharedLoop iter a b d = ed25519Loop iter a b d := rfl

/-- And so the shared implementation is the ed25519 one with a fence spliced
    between the loop and the test against zero. Nothing else differs. -/
theorem the_only_difference_is_the_fence (a b : Array Std.U8 32#usize) :
    sharedEq a b =
      (do let diff ← ed25519Loop { start := 0#usize, «end» := 32#usize } a b 0#u8
          fence
          ok (diff = 0#u8)) := rfl

/-- The ed25519 side, for comparison: the same loop and the same test, with
    nothing between them. -/
theorem the_ed25519_copy_has_no_fence (a b : Array Std.U8 32#usize) :
    ed25519Eq a b =
      (do let diff ← ed25519Loop { start := 0#usize, «end» := 32#usize } a b 0#u8
          ok (diff = 0#u8)) := rfl

/-- Therefore the two return the same answer for every pair of thirty-two byte
    arrays, given the fence succeeds.

    The hypothesis is not a weakening. `compiler_fence` is an ordering barrier
    for the compiler and emits no instruction, so it cannot fail; it is opaque
    here only because Aeneas models `core` intrinsics as axioms. Anyone
    substituting one implementation for the other gets this theorem, and the
    substitution is the fix. -/
theorem the_two_implementations_agree (a b : Array Std.U8 32#usize)
    (hfence : fence = ok ()) : sharedEq a b = ed25519Eq a b := by
  rw [the_only_difference_is_the_fence, the_ed25519_copy_has_no_fence]
  cases h : ed25519Loop { start := 0#usize, «end» := 32#usize } a b 0#u8 with
  | ok d => simp [hfence]
  | fail e => simp
  | div => simp


/-- The sixty-four byte comparison is the same shape and carries the fence too,
    so the shared module is internally consistent even where ed25519 is not. -/
theorem the_sixty_four_byte_comparison_also_fences (a b : Array Std.U8 64#usize) :
    sharedEq64 a b =
      (do let diff ← sharedLoop64 { start := 0#usize, «end» := 64#usize } a b 0#u8
          fence
          ok (diff = 0#u8)) := rfl

/-! ### What this does not establish

    There is no theorem here saying the comparison runs in constant time, and
    there is not going to be one of this kind. Timing is not a function of the
    value a function returns, so no statement relating two return values can
    decide it. The corpus can show the two implementations agree, which makes
    substituting one for the other safe; it cannot show that either is the thing
    the name claims. That argument has to be made about the emitted code, and the
    fence is the part of it this file can point at.
-/

/-! ### Axiom profile -/

#print axioms NonosExtraction.CtEq.the_two_loops_are_one_function
#print axioms NonosExtraction.CtEq.the_only_difference_is_the_fence
#print axioms NonosExtraction.CtEq.the_ed25519_copy_has_no_fence
#print axioms NonosExtraction.CtEq.the_two_implementations_agree
#print axioms NonosExtraction.CtEq.the_sixty_four_byte_comparison_also_fences

end NonosExtraction.CtEq
