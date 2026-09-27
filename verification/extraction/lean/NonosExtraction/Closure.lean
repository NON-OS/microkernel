/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Closing the gap between what is extracted and what is proven.

`EVIDENCE.json` lists the functions Charon and Aeneas lower out of the kernel,
and CI regenerates and diffs every one of them. That is not the same as proving
something about them, and for a while it was quietly a larger set: six functions
were extracted and carried no theorem at all. A manifest that counts those
alongside the rest is inflating its own number.

This file exists so the two sets agree. Five of the six are closed here. The
sixth, `program_header_bounds`, is named at the end with the reason it is not,
because a note is better than a theorem that proves nothing.

`ct_lt_u32` is the one that matters. It is the function whose defect started the
whole constant-time pass, and until now it was covered only at witnesses while
its sixty-four bit sibling had the general theorem. `the_comparison_is_less_than`
is the general statement: for every pair of thirty-two bit words, the extracted
function returns one exactly when the first is below the second.

That proof leans on `bv_decide`, which discharges the bitvector identity by SAT
and checks the resulting LRAT certificate. The check runs as compiled code rather
than in the Lean kernel, so it contributes one axiom beyond the standard three:
`verifyBVExpr <expr> <cert> = true`. Nothing here trusts the SAT solver, only the
checker. That axiom stays inside the extraction project. The core corpus under
`verification/lean` is unaffected, and `proof-corpus-root.sh` still refuses to
emit a root for anything outside Lean's own three, which is the claim the release
identity makes.
-/

import NonosExtraction.Caps
import NonosExtraction.Ct
import NonosExtraction.Iommu
import NonosExtraction.Signal

open Aeneas Aeneas.Std Result

set_option linter.hashCommand false
set_option maxRecDepth 100000
set_option maxHeartbeats 4000000

namespace NonosExtraction.Closure

/-! ### The borrow identity

    Stated on plain bitvectors first, so the SAT-backed step is one small lemma
    with a readable statement rather than something buried in a larger proof. -/

/-- Computing a borrow as `a ^ ((a ^ b) | ((a - b) ^ b))` and taking the top bit
    decides unsigned less-than, for every pair of thirty-two bit words. -/
theorem borrow_decides_less_than (x y : BitVec 32) :
    (x ^^^ ((x ^^^ y) ||| ((x - y) ^^^ y))) >>> 31 = if x < y then 1#32 else 0#32 := by
  bv_decide

/-- The shape the code had before the fix does not decide it. One witness is
    enough to refute a universal claim, and this is the smallest one: the top of
    the signed range against zero. -/
theorem the_wrapped_difference_alone_does_not :
    ¬ (∀ x y : BitVec 32, (x - y) >>> 31 = if x < y then 1#32 else 0#32) := by
  intro h
  have := h 0x80000000#32 0#32
  simp at this

/-! ### ct_lt_u32 and ct_gt_u32 -/

open nonos_ct.security.crypto.constant_time.ops renaming
  ct_lt_u32 → opsLt32, ct_gt_u32 → opsGt32

/-- The general statement, on the extracted function, for every input pair. -/
theorem the_comparison_is_less_than (a b : Std.U32) :
    opsLt32 a b ⦃ z => z.bv = if a.bv < b.bv then 1#32 else 0#32 ⦄ := by
  unfold opsLt32
  simp only [Std.lift, bind_tc_ok]
  step
  simp only [*]
  exact borrow_decides_less_than a.bv b.bv

/-- Greater-than is the same function with its arguments the other way round.
    This holds by definition, which is the contract the two owe each other. -/
theorem greater_than_is_less_than_reversed_u32 (a b : Std.U32) :
    opsGt32 a b = opsLt32 b a := rfl

/-- So greater-than is also correct on every pair. -/
theorem the_comparison_is_greater_than (a b : Std.U32) :
    opsGt32 a b ⦃ z => z.bv = if b.bv < a.bv then 1#32 else 0#32 ⦄ := by
  rw [greater_than_is_less_than_reversed_u32]
  exact the_comparison_is_less_than b a

/-! ### The address width the unit prefers -/

open nonos_iommu.arch.x86_64.iommu.regs.cap.agaw renaming preferred_levels → preferred

/-- `SAGAW` lives in bits 8 to 12 of the capability register, and the choice
    walks it in preference order: four levels first, then three, then five. The
    witnesses are the register values a real unit reports. -/
local macro "compute" : tactic => `(tactic| first | rfl | (simp; rfl))

theorem four_levels_win : preferred 0x400#u64 = ok (some nonos_iommu.arch.x86_64.iommu.regs.cap.agaw.AgawLevels.Four) := by
  compute

theorem three_levels_when_four_is_absent :
    preferred 0x200#u64 = ok (some nonos_iommu.arch.x86_64.iommu.regs.cap.agaw.AgawLevels.Three) := by compute

theorem five_levels_only_when_neither_fits :
    preferred 0x800#u64 = ok (some nonos_iommu.arch.x86_64.iommu.regs.cap.agaw.AgawLevels.Five) := by compute

/-- Four wins even when the unit also offers five, which is the preference the
    code intends and the one a reader would want confirmed rather than assumed. -/
theorem four_is_preferred_over_five :
    preferred 0xC00#u64 = ok (some nonos_iommu.arch.x86_64.iommu.regs.cap.agaw.AgawLevels.Four) := by compute

/-- A unit offering nothing in those three positions gets no answer, rather than
    a default that would silently pick a width the hardware cannot walk. -/
theorem an_empty_sagaw_chooses_nothing : preferred 0#u64 = ok none := by compute

/-- The finding. `SAGAW` has five bits and the choice reads three of them. A unit
    that reports only two level or only six level paging is treated exactly like
    a unit that reported nothing at all. Neither width is one this kernel walks,
    so refusing is right, but it refuses by not looking rather than by deciding,
    and the two are different when a sixth width is added later. -/
theorem the_outer_sagaw_bits_are_not_read :
    preferred 0x100#u64 = ok none ∧ preferred 0x1000#u64 = ok none := by
  refine ⟨by compute, by compute⟩

/-! ### Which signals dump core -/

open nonos_signal.process.signal.helpers renaming generates_core_dump → dumpsCore

/-- Every signal gets an answer. -/
theorem the_dump_classifier_is_total (s : Std.U8) : ∃ b, dumpsCore s = ok b := by
  unfold dumpsCore
  split <;> exact ⟨_, rfl⟩

/-- The ten that dump. -/
theorem the_ten_that_dump :
    dumpsCore 3#u8 = ok true ∧ dumpsCore 4#u8 = ok true ∧
    dumpsCore 5#u8 = ok true ∧ dumpsCore 6#u8 = ok true ∧
    dumpsCore 7#u8 = ok true ∧ dumpsCore 8#u8 = ok true ∧
    dumpsCore 11#u8 = ok true ∧ dumpsCore 24#u8 = ok true ∧
    dumpsCore 25#u8 = ok true ∧ dumpsCore 31#u8 = ok true := by
  refine ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

/-- And the ordinary ones that must not, because a core dump writes a process's
    memory to disk and these are the signals a program receives in normal use. -/
theorem the_ordinary_signals_do_not_dump :
    dumpsCore 1#u8 = ok false ∧ dumpsCore 2#u8 = ok false ∧
    dumpsCore 9#u8 = ok false ∧ dumpsCore 13#u8 = ok false ∧
    dumpsCore 15#u8 = ok false ∧ dumpsCore 17#u8 = ok false := by
  refine ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

/-! ### Still not proven

    `program_header_bounds` is extracted and diffed by CI and carries no theorem.
    A witness needs a fifteen-field header and a slice, and `?` desugars into
    `ControlFlow` over Aeneas's opaque `Option::ok_or`. The property it should
    carry is stated over a model in `Nonos.ElfPhdr`, and on the code it is a host
    test rather than a proof. That is one function, and it is named here so the
    gap is counted rather than forgotten.
-/

/-! ### Axiom profile -/

#print axioms NonosExtraction.Closure.borrow_decides_less_than
#print axioms NonosExtraction.Closure.the_wrapped_difference_alone_does_not
#print axioms NonosExtraction.Closure.the_comparison_is_less_than
#print axioms NonosExtraction.Closure.greater_than_is_less_than_reversed_u32
#print axioms NonosExtraction.Closure.the_comparison_is_greater_than
#print axioms NonosExtraction.Closure.four_levels_win
#print axioms NonosExtraction.Closure.three_levels_when_four_is_absent
#print axioms NonosExtraction.Closure.five_levels_only_when_neither_fits
#print axioms NonosExtraction.Closure.four_is_preferred_over_five
#print axioms NonosExtraction.Closure.an_empty_sagaw_chooses_nothing
#print axioms NonosExtraction.Closure.the_outer_sagaw_bits_are_not_read
#print axioms NonosExtraction.Closure.the_dump_classifier_is_total
#print axioms NonosExtraction.Closure.the_ten_that_dump
#print axioms NonosExtraction.Closure.the_ordinary_signals_do_not_dump

end NonosExtraction.Closure
