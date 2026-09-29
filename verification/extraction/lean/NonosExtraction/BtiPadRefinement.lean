/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

bti_pad, on the extracted code.

Charon cannot take an inherent method as an entry point, so the crate root
carries a free function per method and those are the names the manifest counts.
A wrapper that quietly did something other than call through would make every
theorem about the method a statement about code nobody runs, so each one is
proven to be its method below.

`is_bti_landing_pad` decides whether an indirect branch may land on an
instruction in a guarded page. The Armv8.5 rules: `BTI c`, `BTI j` and `BTI jc`
are landing pads, and `PACIASP` and `PACIBSP` act as `BTI c`. A NOP and a bare
`BTI`, which accepts no branch type, are not: a branch to either raises a
Branch Target exception on a core with FEAT_BTI. `check_bti_landing_pad` used
to accept the NOP and reject `PACIASP` and `PACIBSP`; it now reads the word and
asks this function.

The theorems below give the accepted set exactly and relate it to the words
`BtiGuard::instruction` emits: every guard but `None` emits a landing pad, and
`None`, which emits the NOP, marks code that must not be a branch target.

What these cannot establish: that the word read at an address is the
instruction the core fetches there. `check_bti_landing_pad` reads memory
through a raw pointer, which is not extracted.
-/

import NonosExtraction.BtiPad
import NonosExtraction.BtiGuard

open Aeneas Aeneas.Std Result
open nonos_x_bti_pad

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction.BtiPad

/-! ### The forwarding functions add nothing -/

theorem the_is_bti_landing_pad_wrapper_is_its_method (a : Std.U32) :
    is_bti_landing_pad a = pad.is_bti_landing_pad a := rfl

/-! ### The landing pads -/

/-- The accepted words are exactly `BTI c` (`0xD503245F`), `BTI j`
    (`0xD503249F`), `BTI jc` (`0xD50324DF`), `PACIASP` (`0xD503233F`) and
    `PACIBSP` (`0xD503237F`). -/
theorem is_bti_landing_pad_is_exactly_the_five_landing_pads (w : Std.U32) :
    is_bti_landing_pad w =
      ok (decide (w.val ∈ [0xD503245F, 0xD503249F, 0xD50324DF, 0xD503233F, 0xD503237F])) := by
  unfold is_bti_landing_pad pad.is_bti_landing_pad
  split
  all_goals first
    | rfl
    | (rename_i h1 h2 h3 h4 h5
       congr 1
       symm
       simp only [decide_eq_false_iff_not, List.mem_cons, List.not_mem_nil, or_false]
       intro h
       rcases h with h | h | h | h | h
       · exact h1 (UScalar.eq_of_val_eq h)
       · exact h2 (UScalar.eq_of_val_eq h)
       · exact h3 (UScalar.eq_of_val_eq h)
       · exact h4 (UScalar.eq_of_val_eq h)
       · exact h5 (UScalar.eq_of_val_eq h))

/-- The NOP (`0xD503201F`), hint 0, and the bare `BTI` (`0xD503241F`) are not
    landing pads. The check used to accept the NOP. -/
theorem nop_and_bare_bti_are_not_landing_pads :
    is_bti_landing_pad 0xD503201F#u32 = ok false ∧
      is_bti_landing_pad 0xD503241F#u32 = ok false := ⟨rfl, rfl⟩

/-- `PACIASP` and `PACIBSP` are landing pads, as the architecture treats them
    as `BTI c`. The check used to reject both. -/
theorem paciasp_and_pacibsp_are_landing_pads :
    is_bti_landing_pad 0xD503233F#u32 = ok true ∧
      is_bti_landing_pad 0xD503237F#u32 = ok true := ⟨rfl, rfl⟩

/-- Every guard but `None` emits a landing pad, and `None` emits a word that is
    not one, so code marked `None` cannot be reached by an indirect branch in a
    guarded page. -/
theorem a_guard_emits_a_landing_pad_exactly_unless_it_is_none
    (g : nonos_x_bti_guard.guard.BtiGuard) :
    ∃ w, nonos_x_bti_guard.btiguard_instruction g = ok w ∧
      is_bti_landing_pad w = ok (match g with | .None => false | _ => true) := by
  cases g <;> exact ⟨_, rfl, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.BtiPad.the_is_bti_landing_pad_wrapper_is_its_method
#print axioms NonosExtraction.BtiPad.is_bti_landing_pad_is_exactly_the_five_landing_pads
#print axioms NonosExtraction.BtiPad.nop_and_bare_bti_are_not_landing_pads
#print axioms NonosExtraction.BtiPad.paciasp_and_pacibsp_are_landing_pads
#print axioms NonosExtraction.BtiPad.a_guard_emits_a_landing_pad_exactly_unless_it_is_none

end NonosExtraction.BtiPad
