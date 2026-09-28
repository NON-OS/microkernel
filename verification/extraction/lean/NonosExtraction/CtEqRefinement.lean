/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

ed25519 reaches the fenced comparison.

The kernel used to have `ct_eq_32` twice. One in
`crypto/util/constant_time/compare.rs`, which calls `compiler_fence` between the
accumulating loop and the test against zero, and a `pub(crate)` copy inside
`crypto/asymmetric/ed25519/field/` which did not.

Nothing in the loop's value depended on that fence, which is exactly why an
optimiser was free without it to notice `diff` only grows, exit once it is
non-zero, and make the running time depend on where the first differing byte is.
The unfenced copy was what `signature.rs` used for the final verification
equality, `ct_eq_32(ge_pack(sb), ge_pack(rp3))`, so the timing of the line that
decides valid or invalid was a function of attacker-supplied points.

An earlier version of this file proved the two accumulating loops were
definitionally the same function. That is what made deleting one of them a
substitution rather than a behaviour change, and it is why the fix could be a
single line.

The duplicate is gone and the property worth holding has changed with it. What
matters now is that ed25519 really does reach the fenced implementation, and that
it has not quietly grown a second one again. That is what this file checks, and
it checks it on the extracted code rather than by reading the source.

The sixty-four byte comparison is covered in `NonosExtraction.CtRefinement`
rather than here, because `Ct` and `EdField` cannot be imported into one
environment: Aeneas emits the same derived instance in both crates and the two
definitions collide.

What it still cannot do is establish that either implementation runs in constant
time. Timing is not a function of the value a function returns, so no statement
relating return values decides it. The fence is the part of that argument this
file can point at, and pointing at it is the whole reason the duplicate mattered.
-/

import NonosExtraction.EdField

open Aeneas Aeneas.Std Result
open nonos_ed_field

set_option linter.hashCommand false
set_option maxRecDepth 20000

namespace NonosExtraction.CtEq

/-! ### The duplicate is gone -/

/-- ed25519's `ct_eq_32` is a call to the shared one and nothing else. There is
    no second loop left to drift. -/
theorem the_ed25519_entry_point_is_a_call_to_the_shared_one
    (a b : Array Std.U8 32#usize) :
    field.compare.ct_eq_32 a b = crypto.util.constant_time.compare.ct_eq_32 a b :=
  rfl

/-- So the two names are one function, for every pair of thirty-two byte arrays.
    Before the fix this needed a proof that two separate loops agreed. Now it
    holds because there is only one loop. -/
theorem ed25519_and_the_shared_comparison_are_one_function
    (a b : Array Std.U8 32#usize) : ed25519_ct_eq_32 a b = shared_ct_eq_32 a b :=
  rfl

/-- And the thing ed25519 now reaches is the one that fences: the shared
    implementation runs its loop, calls `compiler_fence`, and only then tests
    against zero. -/
theorem the_shared_comparison_fences_between_the_loop_and_the_test
    (a b : Array Std.U8 32#usize) :
    crypto.util.constant_time.compare.ct_eq_32 a b =
      (do let diff ← crypto.util.constant_time.compare.ct_eq_32_loop
                       { start := 0#usize, «end» := 32#usize } a b 0#u8
          crypto.util.constant_time.barriers.compiler_fence
          ok (diff = 0#u8)) := rfl

/-- Put together: an ed25519 signature check now goes through a comparison with a
    fence in it, and that is true of the extracted code rather than of the source
    as read. -/
theorem an_ed25519_comparison_goes_through_the_fence
    (a b : Array Std.U8 32#usize) :
    ed25519_ct_eq_32 a b =
      (do let diff ← crypto.util.constant_time.compare.ct_eq_32_loop
                       { start := 0#usize, «end» := 32#usize } a b 0#u8
          crypto.util.constant_time.barriers.compiler_fence
          ok (diff = 0#u8)) := rfl

/-! ### What this does not establish

    There is no theorem here saying the comparison runs in constant time, and
    there will not be one of this kind. Timing is not a function of the value a
    function returns, so no statement relating return values can decide it. The
    corpus can show which implementation is reached, which is what stops the
    fence being lost again. Whether the fence is sufficient is an argument about
    emitted code.
-/

/-! ### Axiom profile -/

#print axioms NonosExtraction.CtEq.the_ed25519_entry_point_is_a_call_to_the_shared_one
#print axioms NonosExtraction.CtEq.ed25519_and_the_shared_comparison_are_one_function
#print axioms NonosExtraction.CtEq.the_shared_comparison_fences_between_the_loop_and_the_test
#print axioms NonosExtraction.CtEq.an_ed25519_comparison_goes_through_the_fence

end NonosExtraction.CtEq
