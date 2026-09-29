/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The ELF loader's bounds checks, on the extracted code.

An ELF image is attacker-controlled bytes, so the checks between it and the
loader's reads are the ones worth proving over the code rather than over a model.
Three are extracted.

`in_range` decides whether a relocation target lies wholly inside a segment. Its
own header says it was factored out of `in_segment` so it holds no image and could
be checked against `Nonos.Bounds`, which is a hand-written model; the theorems
below are the same properties over the function. Both ends are formed with
`checked_add`, so the interesting case is the one a naive check gets wrong: a range
whose end wraps past the top of the address space. `a_wrapping_range_is_refused`
is that, for every address and size that wrap, not at a witness.

`is_supported` is the relocation-type allowlist. A type absent from it is refused,
which is what keeps the applier from acting on a relocation kind nobody wrote code
for.

`program_header_bounds` is proven in `ElfBoundsRefinement`: an accepted non-empty
table ends inside the image, and on every header whose offset fits a `usize` the
function returns the verdict of the `Nonos.ElfPhdr` model. Those theorems carry the
four opaque standard-library calls the function makes, so they live in a file of
their own and the profile below stays at Lean's three. The host test
`kernel_proofs::elf_tests::program_header_table_never_overflows_or_escapes_the_file`
still runs beside them.
-/

import NonosExtraction.Elf

open Aeneas Aeneas.Std Result
open nonos_elf

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction

open elf.reloc.apply.range renaming in_range → inRange
open elf.reloc.utils.supported renaming is_supported → relocSupported

/-! ### Overflow is refusal, not acceptance -/

/-- A segment whose end wraps is never in range, whatever is asked of it. A check
    that computed `start + seg_size` unguarded would compare against a small
    number and accept most of the address space. -/
theorem a_wrapping_segment_is_refused (addr size start seg_size : Std.U64)
    (h : U64.checked_add start seg_size = none) :
    inRange addr size start seg_size = ok false := by
  simp [elf.reloc.apply.range.in_range, Std.lift, h]

/-- And a request whose own end wraps is refused, which is the case an attacker
    controls directly: a relocation at a high address with a large size. -/
theorem a_wrapping_request_is_refused (addr size start seg_size : Std.U64)
    (hs : U64.checked_add start seg_size = some (0#u64))
    (h : U64.checked_add addr size = none) :
    inRange addr size start seg_size = ok false := by
  simp [elf.reloc.apply.range.in_range, Std.lift, hs, h]

/-- The witness, so the refusal is visible as a value rather than only as a
    hypothesis: the last byte of the address space with a size of one. -/
theorem the_top_of_the_address_space_is_refused :
    inRange 0xFFFFFFFFFFFFFFFF#u64 1#u64 0#u64 0xFFFFFFFFFFFFFFFF#u64 = ok false := by
  rfl

/-! ### What acceptance means -/

/-- An accepted range starts at or after the segment, so nothing below the segment
    is ever written. -/
theorem an_accepted_range_starts_inside (addr size start seg_size : Std.U64)
    (h : inRange addr size start seg_size = ok true) : start.val ≤ addr.val := by
  simp only [elf.reloc.apply.range.in_range, Std.lift, bind_tc_ok] at h
  split at h
  · simp at h
  · split at h
    · simp at h
    · split at h
      · rename_i hge
        scalar_tac
      · simp at h

/-- The whole of a segment is in range, so the check is not vacuously strict. -/
theorem the_whole_segment_is_in_range (start seg_size : Std.U64) (e : Std.U64)
    (h : U64.checked_add start seg_size = some e) :
    inRange start seg_size start seg_size = ok true := by
  simp [elf.reloc.apply.range.in_range, Std.lift, h]

/-- An empty request at the very end of a segment is accepted, which is the
    boundary an off-by-one puts on the wrong side. -/
theorem an_empty_request_at_the_end_is_accepted :
    inRange 0x2000#u64 0#u64 0x1000#u64 0x1000#u64 = ok true := by
  rfl

/-- One byte past the end is refused. -/
theorem one_byte_past_the_end_is_refused :
    inRange 0x2000#u64 1#u64 0x1000#u64 0x1000#u64 = ok false := by
  rfl

/-- And one byte below the start is refused, so the check guards both ends rather
    than only the upper one. -/
theorem one_byte_below_the_start_is_refused :
    inRange 0xFFF#u64 1#u64 0x1000#u64 0x1000#u64 = ok false := by
  rfl

/-- The ordinary case, inside on both sides. -/
theorem an_interior_range_is_accepted :
    inRange 0x1800#u64 8#u64 0x1000#u64 0x1000#u64 = ok true := by
  rfl

/-- An empty segment admits only an empty request at its own address, so a
    zero-length segment cannot be written through. -/
theorem an_empty_segment_admits_only_nothing :
    inRange 0x1000#u64 0#u64 0x1000#u64 0#u64 = ok true ∧
    inRange 0x1000#u64 1#u64 0x1000#u64 0#u64 = ok false := by
  refine ⟨rfl, rfl⟩

/-! ### The relocation allowlist -/

/-- The kinds the applier handles are accepted. -/
theorem the_handled_relocations_are_accepted :
    relocSupported 0#u32 = ok true ∧      -- R_X86_64_NONE
    relocSupported 1#u32 = ok true ∧      -- R_X86_64_64
    relocSupported 8#u32 = ok true ∧      -- R_X86_64_RELATIVE
    relocSupported 37#u32 = ok true := by -- R_X86_64_IRELATIVE
  refine ⟨rfl, rfl, rfl, rfl⟩

/-- And a kind nobody wrote code for is refused rather than silently treated as
    one that was. The thread-local kinds and an arbitrary out-of-range value are
    both outside the list. -/
theorem an_unhandled_relocation_is_refused :
    relocSupported 18#u32 = ok false ∧      -- R_X86_64_TPOFF64
    relocSupported 20#u32 = ok false ∧      -- R_X86_64_DTPOFF64
    relocSupported 999#u32 = ok false ∧
    relocSupported 0xFFFFFFFF#u32 = ok false := by
  refine ⟨rfl, rfl, rfl, rfl⟩

/-- The allowlist is total: every type gets an answer, so an unknown kind cannot
    make the check itself fail and be mistaken for a pass. -/
theorem the_allowlist_is_total (t : Std.U32) : ∃ b, relocSupported t = ok b := by
  simp only [elf.reloc.utils.supported.is_supported, Std.lift, bind_tc_ok]
  split <;> exact ⟨_, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.a_wrapping_segment_is_refused
#print axioms NonosExtraction.a_wrapping_request_is_refused
#print axioms NonosExtraction.the_top_of_the_address_space_is_refused
#print axioms NonosExtraction.an_accepted_range_starts_inside
#print axioms NonosExtraction.the_whole_segment_is_in_range
#print axioms NonosExtraction.an_empty_request_at_the_end_is_accepted
#print axioms NonosExtraction.one_byte_past_the_end_is_refused
#print axioms NonosExtraction.one_byte_below_the_start_is_refused
#print axioms NonosExtraction.an_interior_range_is_accepted
#print axioms NonosExtraction.an_empty_segment_admits_only_nothing
#print axioms NonosExtraction.the_handled_relocations_are_accepted
#print axioms NonosExtraction.an_unhandled_relocation_is_refused
#print axioms NonosExtraction.the_allowlist_is_total

end NonosExtraction
