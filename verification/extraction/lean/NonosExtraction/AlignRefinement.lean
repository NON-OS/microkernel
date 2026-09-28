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

Overflow at the top of the address space. `memory/layout` and `memory/phys` add
unchecked and halt, because this kernel builds with overflow checks and panic set
to abort. `memory/boot_memory` and `memory/buddy_alloc` return the value
unchanged.

That last part is a fix rather than a description. Both of them used to return an
address strictly below the value they were asked to round up: `boot_memory`
through `saturating_add`, and `buddy_alloc` through a `checked_add` whose `None`
arm was written `usize::MAX & !(align - 1)`. Two independent implementations, two
different routes, the same wrong answer, and it is the direction that turns a
bounds check into a pass. The theorems below are the ones that now hold, and the
ones that used to hold are named so the regression has something to fail
against.

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

/-- `boot_memory` reaches `checked_add`, which reduces once it and the guard
    subtraction are named. It used to reach `saturating_add`, which is why an
    earlier version of this tactic named that instead. -/
local macro "bootcalc" h:ident : tactic =>
  `(tactic|
    (simp [boot_align_up, boot_memory.align_up, boot_align_down,
           boot_memory.align_down, $h:ident, Std.U64.checked_add,
           core.num.checked_add_UScalar, Std.Option.ofResult, Std.U64.max,
           Std.U64.numBits, Std.lift, bind_tc_ok] <;> first | rfl | decide))

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

/-- The property `align_up` exists to have: its result is never below its input.
    At the very top of the address space there is nothing to round up to, so it
    returns the value unchanged rather than an address below it. -/
theorem boot_align_up_never_returns_less_than_its_input :
    boot_align_up 0xFFFFFFFFFFFFFFFF#u64 4096#u64 = ok 0xFFFFFFFFFFFFFFFF#u64 := by
  have h : (4096#u64 - 1#u64) = ok 4095#u64 := rfl
  bootcalc h

/-- What it used to return, kept as a named value so the regression has something
    to fail against. `saturating_add` pinned the sum at the maximum and the mask
    then cleared the low bits, giving an address four thousand and ninety six
    below the input. -/
theorem the_value_it_used_to_return_is_below_the_input :
    (0xFFFFFFFFFFFFF000#u64).val < (0xFFFFFFFFFFFFFFFF#u64).val := by decide

/-- Where the boundary actually is. Two pages below the top still rounds, and
    one page below does not, because the rounded value would be the first address
    outside the space. That is the correct answer rather than a lost page: there
    is no aligned address above the input to return. -/
theorem the_boundary_is_where_rounding_leaves_the_address_space :
    boot_align_up 0xFFFFFFFFFFFFE001#u64 4096#u64 = ok 0xFFFFFFFFFFFFF000#u64 ∧
    boot_align_up 0xFFFFFFFFFFFFF001#u64 4096#u64 = ok 0xFFFFFFFFFFFFF001#u64 := by
  have h : (4096#u64 - 1#u64) = ok 4095#u64 := rfl
  refine ⟨by bootcalc h, by bootcalc h⟩

/-- The other two still halt at the same input, which is a controlled abort under
    this build profile rather than a wrong answer. Three implementations, two
    behaviours, and neither of them is an address below the input. -/
theorem the_unchecked_pair_still_halts :
    layout_align_up 0xFFFFFFFFFFFFFFFF#u64 4096#u64 = fail Error.integerOverflow ∧
    phys_align_up 0xFFFFFFFFFFFFFFFF#u64 4096#u64 = fail Error.integerOverflow := by
  refine ⟨rfl, rfl⟩

/-- `memory/buddy_alloc` declines an alignment of zero the way the others do,
    rather than halting the way `PhysAddr::align_up` does. Only the zero case is
    stated: the rest are at `usize`, which Aeneas models as at least thirty two
    bits, so an arithmetic witness there carries a width side condition not worth
    the noise. -/
theorem buddy_declines_a_zero_alignment :
    buddy_align_up 10#usize 0#usize = ok 10#usize := rfl

/-! ### Alignment of zero -/

/-- Three of the four decline rather than halt, which is the opposite of what
    `PhysAddr::align_up` does with the same argument. -/
theorem zero_alignment_is_declined_here_and_halts_there :
    layout_align_up 4096#u64 0#u64 = ok 4096#u64 ∧
    phys_align_up 4096#u64 0#u64 = ok 4096#u64 ∧
    boot_align_up 4096#u64 0#u64 = ok 4096#u64 := by
  refine ⟨rfl, rfl, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.Align.buddy_declines_a_zero_alignment
#print axioms NonosExtraction.Align.the_unchecked_pair_still_halts
#print axioms NonosExtraction.Align.the_boundary_is_where_rounding_leaves_the_address_space
#print axioms NonosExtraction.Align.the_value_it_used_to_return_is_below_the_input
#print axioms NonosExtraction.Align.boot_align_up_never_returns_less_than_its_input
#print axioms NonosExtraction.Align.on_the_ordinary_case_they_agree
#print axioms NonosExtraction.Align.the_down_variants_agree_too
#print axioms NonosExtraction.Align.a_non_power_of_two_gets_two_different_answers
#print axioms NonosExtraction.Align.declining_is_indistinguishable_from_succeeding
#print axioms NonosExtraction.Align.zero_alignment_is_declined_here_and_halts_there

end NonosExtraction.Align
