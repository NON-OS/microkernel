/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

layout_stack_slots, on the extracted code.

`get_all_stack_regions` describes each CPU's kernel stack and eight IST stacks,
and `get_guard_regions` takes a guard page below and above each one. The stacks
used to be packed back to back, so the kernel stack's upper guard was the first
page of IST stack 0 and each IST stack's upper guard the first page of the
next: `verify_stack_integrity` would have reported a live neighbouring stack as
a compromised guard, or, with the guards unmapped, a stack would have lost its
top page. The stacks now start at `stack_slot_offset`, which leaves a guard page
below the first stack, between every two stacks and above the last.

The theorems below give the offsets exactly and prove the layout property: any
two slots are separated by a full guard page, and the whole area, guards
included, fits the per-CPU stride.

What these cannot establish: that the pages are mapped or unmapped as the
table says. The table is a description that the integrity check and the
hardening region list read; `get_all_stack_regions` builds a `Vec`, which is
not extracted.
-/

import NonosExtraction.LayoutStackSlots

open Aeneas Aeneas.Std Result
open nonos_x_layout_stack_slots

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction.LayoutStackSlots

/-! ### The forwarding functions add nothing -/

theorem the_stack_slot_offset_wrapper_is_its_method (a : Std.Usize) :
    stack_slot_offset a = memory.layout.manager.stack_slots.stack_slot_offset a := rfl

/-! ### The stack slots and their guards -/

/-- Slot 0 starts one guard page into the area; slot `i + 1` starts after the
    kernel stack, its guard, and `i` IST stacks with a guard each. -/
theorem stack_slot_offset_spec (s : Std.Usize) (hs : s.val ≤ 8) :
    memory.layout.manager.stack_slots.stack_slot_offset s ⦃ o =>
      o.val = if s.val = 0 then 4096 else 73728 + (s.val - 1) * 36864 ⦄ := by
  unfold memory.layout.manager.stack_slots.stack_slot_offset
    memory.layout.constants.percpu.GUARD_PAGES memory.layout.constants.page.PAGE_SIZE
    memory.layout.constants.percpu.KSTACK_SIZE memory.layout.constants.percpu.IST_STACK_SIZE
  step*
  all_goals (simp_all [U64.max, U64.numBits, UScalarTy.numBits]; try omega)

/-- Size of the stack in slot `s`: the 64 KiB kernel stack, then 32 KiB IST
    stacks. -/
def slotSize (s : Nat) : Nat := if s = 0 then 65536 else 32768

/-- For any two slots, the earlier stack and a full guard page above it end at
    or before the later stack begins. So a stack's upper guard is never a page
    of the next stack, and a stack's lower guard is never a page of the one
    before it. -/
theorem stack_slots_keep_a_guard_page_between_every_two_stacks (a b : Std.Usize)
    (hab : a.val < b.val) (hb : b.val ≤ 8) :
    ∃ oa ob : Std.U64,
      memory.layout.manager.stack_slots.stack_slot_offset a = ok oa ∧
      memory.layout.manager.stack_slots.stack_slot_offset b = ok ob ∧
      oa.val + slotSize a.val + 4096 ≤ ob.val := by
  obtain ⟨oa, ha, hoa⟩ := WP.spec_imp_exists (stack_slot_offset_spec a (by omega))
  obtain ⟨ob, hb', hob⟩ := WP.spec_imp_exists (stack_slot_offset_spec b hb)
  refine ⟨oa, ob, ha, hb', ?_⟩
  unfold slotSize
  rw [hoa, hob]
  have step : (a.val - 1) * 36864 + 36864 ≤ (b.val - 1) * 36864 ∨ a.val = 0 := by
    rcases Nat.eq_zero_or_pos a.val with h | h
    · exact Or.inr h
    · left
      rw [← Nat.succ_mul]
      exact Nat.mul_le_mul_right _ (by omega)
  split_ifs <;> omega

/-- The first stack has its guard below it inside the CPU's area, and the last
    IST stack with its guard above ends inside the 16 MiB per-CPU stride. -/
theorem the_stack_area_has_a_guard_below_and_above_and_fits_its_stride :
    ∃ o0 o8 : Std.U64,
      memory.layout.manager.stack_slots.stack_slot_offset 0#usize = ok o0 ∧
      memory.layout.manager.stack_slots.stack_slot_offset 8#usize = ok o8 ∧
      4096 ≤ o0.val ∧ o8.val + 32768 + 4096 ≤ 0x100_0000 := by
  obtain ⟨o0, h0, v0⟩ := WP.spec_imp_exists (stack_slot_offset_spec 0#usize (by decide))
  obtain ⟨o8, h8, v8⟩ := WP.spec_imp_exists (stack_slot_offset_spec 8#usize (by decide))
  refine ⟨o0, o8, h0, h8, ?_, ?_⟩
  · rw [v0]; decide
  · rw [v8]; decide

/-! ### Axiom profile -/

#print axioms NonosExtraction.LayoutStackSlots.the_stack_slot_offset_wrapper_is_its_method
#print axioms NonosExtraction.LayoutStackSlots.stack_slot_offset_spec
#print axioms NonosExtraction.LayoutStackSlots.stack_slots_keep_a_guard_page_between_every_two_stacks
#print axioms NonosExtraction.LayoutStackSlots.the_stack_area_has_a_guard_below_and_above_and_fits_its_stride

end NonosExtraction.LayoutStackSlots
