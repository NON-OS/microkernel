/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Four implementations of align_up, and they do not agree.

`align_up` and `align_down` are defined independently in `memory/boot_memory`,
`memory/buddy_alloc`, `memory/layout` and `memory/phys`, and `PhysAddr` and
`VirtAddr` carry a fifth and sixth in `memory/addr`. Six copies of one operation
in one subsystem. They agree on the ordinary case, which is a power of two
alignment well away from the top of the address space, and that is the only case
anybody tests.

They disagree in three places, and each disagreement is a different answer to the
same question rather than a different shade of the same one.

An alignment of zero. Four of them return the value unchanged, silently declining
to align. `PhysAddr::align_up` halts, which `NonosExtraction.AddrPhys` records.

An alignment that is not a power of two. `memory/phys` divides, so it aligns
correctly to any modulus. The other three test `a & (a - 1)` and return the value
unchanged, declining. So `align_up(10, 3)` is 12 in one module and 10 in another,
and both are the answer this kernel gives.

Overflow at the top of the address space, which is the finding. `memory/layout`
and `memory/phys` add unchecked and halt, because this kernel builds with overflow
checks and panic set to abort. The other two do not halt and do not refuse: they
return an address strictly below the value they were asked to round up.
`memory/boot_memory` gets there through `saturating_add`, and
`memory/buddy_alloc` through a `checked_add` whose `None` arm is written
`usize::MAX & !(align - 1)`. Two independent implementations, two different
routes, the same wrong answer.

A caller cannot tell which of these it is calling from the name, and the modules
are close enough together that moving a helper between them changes behaviour
with no signal.
-/

import NonosExtraction.Align

open Aeneas Aeneas.Std Result
open nonos_align

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction.Align

/-- `boot_memory` reaches `saturating_add`, which is a real definition rather
    than an opaque one but does not reduce on its own. Naming it, the width
    constant and the guard subtraction lets the whole term compute. -/
local macro "bootcalc" h:ident : tactic =>
  `(tactic|
    (simp [boot_align_up, boot_memory.align_up, boot_align_down,
           boot_memory.align_down, $h:ident, core.num.U64.saturating_add,
           Std.UScalar.saturating_add, Std.U64.max, Std.U64.numBits, Std.lift,
           bind_tc_ok] <;> decide))

/-! ### The ordinary case, where they agree

    A power of two alignment away from the top. This is what every call site
    passes and why the rest of this file has gone unnoticed. -/

theorem on_the_ordinary_case_they_agree :
    layout_align_up 4097#u64 4096#u64 = ok 8192#u64 ∧
    phys_align_up 4097#u64 4096#u64 = ok 8192#u64 ∧
    boot_align_up 4097#u64 4096#u64 = ok 8192#u64 := by
  have h : (4096#u64 - 1#u64) = ok 4095#u64 := rfl
  refine ⟨rfl, rfl, by bootcalc h⟩

theorem the_down_variants_agree_too :
    layout_align_down 4097#u64 4096#u64 = ok 4096#u64 ∧
    phys_align_down 4097#u64 4096#u64 = ok 4096#u64 ∧
    boot_align_down 4097#u64 4096#u64 = ok 4096#u64 := by
  have h : (4096#u64 - 1#u64) = ok 4095#u64 := rfl
  refine ⟨rfl, rfl, by bootcalc h⟩

/-! ### An alignment that is not a power of two -/

/-- `memory/phys` divides, so it rounds ten up to a multiple of three correctly.
    `memory/layout` and `memory/boot_memory` test `a & (a - 1)` and hand the
    value back untouched. Both answers ship. -/
theorem a_non_power_of_two_gets_two_different_answers :
    phys_align_up 10#u64 3#u64 = ok 12#u64 ∧
    layout_align_up 10#u64 3#u64 = ok 10#u64 ∧
    boot_align_up 10#u64 3#u64 = ok 10#u64 := by
  have h : (3#u64 - 1#u64) = ok 2#u64 := rfl
  refine ⟨rfl, rfl, by bootcalc h⟩

/-- Declining to align is not obviously wrong, but it is silent. The caller gets
    a value that is not aligned and no indication that the request was refused,
    which is the part that matters. -/
theorem declining_is_indistinguishable_from_succeeding :
    layout_align_up 10#u64 3#u64 = ok 10#u64 ∧
    layout_align_up 10#u64 1#u64 = ok 10#u64 := by
  refine ⟨rfl, rfl⟩

/-! ### The top of the address space

    This is the one worth acting on. -/

/-- `memory/boot_memory` uses `saturating_add`, so at the top of the address
    space the sum stops at the maximum and the mask then clears the low bits.
    The result is strictly below the value it was asked to round up.

    `align_up` returning an address lower than its input is not a rounding, it is
    a wrong answer in the direction that turns a bounds check into a pass. A
    caller computing an end address from it gets a region that appears to end
    before it begins. -/
theorem boot_align_up_can_return_less_than_its_input :
    boot_align_up 0xFFFFFFFFFFFFFFFF#u64 4096#u64 = ok 0xFFFFFFFFFFFFF000#u64 := by
  have h : (4096#u64 - 1#u64) = ok 4095#u64 := rfl
  bootcalc h

/-- Stated as the order violation it is, so the theorem says the property rather
    than the number. -/
theorem boot_align_up_violates_the_only_thing_align_up_promises :
    ∃ v a r, boot_align_up v a = ok r ∧ r.val < v.val := by
  refine ⟨0xFFFFFFFFFFFFFFFF#u64, 4096#u64, 0xFFFFFFFFFFFFF000#u64,
          boot_align_up_can_return_less_than_its_input, ?_⟩
  decide

/-- The other three do not do that. Two halt, which under this build profile is
    an abort, and one returns the input unchanged. Three different behaviours at
    one input, none of them agreeing with the fourth. -/
theorem the_others_behave_differently_at_the_same_input :
    layout_align_up 0xFFFFFFFFFFFFFFFF#u64 4096#u64 = fail Error.integerOverflow ∧
    phys_align_up 0xFFFFFFFFFFFFFFFF#u64 4096#u64 = fail Error.integerOverflow := by
  refine ⟨rfl, rfl⟩


/-- `memory/buddy_alloc` declines an alignment of zero the way the others do,
    rather than halting the way `PhysAddr::align_up` does.

    Only the zero case is stated. The others are at `usize`, and Aeneas models
    that as at least thirty two bits rather than exactly sixty four, so an
    arithmetic witness at this type carries a width side condition that is not
    worth the noise here. -/
theorem buddy_declines_a_zero_alignment :
    buddy_align_up 10#usize 0#usize = ok 10#usize := rfl

/-- Its overflow arm is the same defect as `boot_memory`, reached differently,
    and it is not proven here.

    The source catches the overflow with `checked_add` and then writes
    `usize::MAX & !(align - 1)` in the arm that handles it, which is an address
    below the input for the same reason the saturating version is. The witness
    cannot be written at this type: Aeneas models `usize` as at least thirty two
    bits rather than exactly sixty four, so a literal of `usize::MAX` is not
    provably in range and the overflow case is not reachable in the model.

    The `u64` theorem above is the same defect at a type where it can be stated,
    and the two functions are the same shape. Recorded rather than proven, and
    said here instead of left out. -/
theorem the_ordering_is_violated_where_it_can_be_stated :
    ∃ v a r, boot_align_up v a = ok r ∧ r.val < v.val :=
  boot_align_up_violates_the_only_thing_align_up_promises

/-! ### Alignment of zero -/

/-- Three of the four decline rather than halt, which is the opposite of what
    `PhysAddr::align_up` does with the same argument. -/
theorem zero_alignment_is_declined_here_and_halts_there :
    layout_align_up 4096#u64 0#u64 = ok 4096#u64 ∧
    phys_align_up 4096#u64 0#u64 = ok 4096#u64 ∧
    boot_align_up 4096#u64 0#u64 = ok 4096#u64 := by
  refine ⟨rfl, rfl, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.Align.on_the_ordinary_case_they_agree
#print axioms NonosExtraction.Align.the_down_variants_agree_too
#print axioms NonosExtraction.Align.a_non_power_of_two_gets_two_different_answers
#print axioms NonosExtraction.Align.declining_is_indistinguishable_from_succeeding
#print axioms NonosExtraction.Align.boot_align_up_can_return_less_than_its_input
#print axioms NonosExtraction.Align.boot_align_up_violates_the_only_thing_align_up_promises
#print axioms NonosExtraction.Align.buddy_declines_a_zero_alignment
#print axioms NonosExtraction.Align.the_ordering_is_violated_where_it_can_be_stated
#print axioms NonosExtraction.Align.the_others_behave_differently_at_the_same_input
#print axioms NonosExtraction.Align.zero_alignment_is_declined_here_and_halts_there

end NonosExtraction.Align
