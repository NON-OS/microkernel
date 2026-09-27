/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The IOMMU tables, on the extracted code.

This is the boundary a device's DMA crosses. A driver capsule holding `Dma` gets
a buffer, and everything that keeps the device inside that buffer is an encoding
in these two files: the second-level entries the addresses are translated
through, and the root and context entries that decide which device gets which
table.

Two properties make the encoding safe and both are about what a zero means. A
second-level entry counts as present when it grants read or write, so a zeroed
table denies rather than faulting open, and a leaf built with neither permission
denies as well. A context entry is honoured only with its present bit set, so a
zeroed root table blocks every device rather than passing them through
untranslated. `zeroed_tables_deny_everything` is both of those together, and it
is the property that decides what happens between allocating a table and filling
it in.

The other half is that addresses and permissions cannot be confused for one
another. Every encoder masks the physical address to bits 51:12 before it sets a
flag, so a misaligned frame cannot bleed into the permission bits and a
permission bit can never be read back as part of an address. Those are shown at
representative frames rather than quantified: quantifying needs a bitvector
argument that two masks do not overlap, and neither reduction nor `bv_decide`
reaches through the scalar and `Result` encoding the extraction produces.

The last group is about the level arithmetic, which is partial. `index_for` and
`level_span` both compute `level - 1` and shift by it, so level zero underflows:
`the_span_fails_at_level_zero` is that failure, as the overflow it really is. The
shift also passes the word width at level seven, which is not proven here.

Rather than clamp, which would replace a loud failure with a wrong index,
`the_producer_keeps_the_indexing_total` proves the precondition holds. The only
thing that produces a level is `AgawLevels`, it has three inhabitants, and
composing it with the span succeeds on all three. The panic is unreachable
because of what can reach it, and that is a theorem rather than an argument.
-/

import NonosExtraction.Iommu

open Aeneas Aeneas.Std Result
open nonos_iommu

set_option linter.hashCommand false

namespace NonosExtraction

/-! ### Names

    A namespace cannot be abbreviated, so the encoders are opened under short
    names. The generated constants are `irreducible`, which is why every proof
    below names them in its `simp` set rather than relying on reduction. -/

open arch.x86_64.iommu.tables.sl_pte renaming
  is_present → slPresent, entry_address → slAddress, leaf → slLeaf,
  table → slTable, index_for → slIndex, level_span → slSpan,
  fits_address_width → slFits

open arch.x86_64.iommu.tables.context renaming
  is_present → ctxPresent, entry_address → ctxAddress, root_low → rootLow,
  context_low → ctxLow, context_high → ctxHigh, context_index → ctxIndex,
  context_domain → ctxDomain

/-- The paging depth a unit reported: the only producer of the level the indexing
    arithmetic below is partial in. -/
abbrev Agaw := arch.x86_64.iommu.regs.cap.agaw.AgawLevels

attribute [local simp]
  arch.x86_64.iommu.tables.sl_pte.SL_READ
  arch.x86_64.iommu.tables.sl_pte.SL_WRITE
  arch.x86_64.iommu.tables.sl_pte.SL_SNOOP
  arch.x86_64.iommu.tables.sl_pte.SL_ADDR_MASK
  arch.x86_64.iommu.tables.sl_pte.ENTRIES
  arch.x86_64.iommu.tables.sl_pte.LEVEL_SHIFT
  arch.x86_64.iommu.tables.sl_pte.PAGE_SHIFT
  arch.x86_64.iommu.tables.context.PRESENT
  arch.x86_64.iommu.tables.context.ADDR_MASK
  arch.x86_64.iommu.tables.context.TT_MULTI_LEVEL
  slPresent slAddress slLeaf slTable slIndex slSpan slFits
  ctxPresent ctxAddress rootLow ctxLow ctxHigh ctxIndex ctxDomain

/-- Unfold the generated constants, then let the kernel finish the closed
    arithmetic the unfolding exposes. The constants are `irreducible` so
    reduction alone cannot start, and the checked shifts they expand into are
    concrete so `simp` alone need not finish. -/
macro "encode" : tactic => `(tactic| first | (simp; done) | (simp; rfl))

/-! ### A zero denies -/

/-- A zeroed second-level entry grants nothing. Allocating a table and mapping it
    before filling it in denies every access through it, which is the safe order
    for a boundary that the hardware can see the moment it exists. -/
theorem a_zero_second_level_entry_denies : slPresent 0#u64 = ok false := by encode

/-- A zeroed context entry is not honoured, so a zeroed root table blocks every
    device rather than leaving it untranslated. -/
theorem a_zero_context_entry_denies : ctxPresent 0#u64 = ok false := by encode

/-- Both together: neither table format grants anything before it is written. -/
theorem zeroed_tables_deny_everything :
    slPresent 0#u64 = ok false ∧ ctxPresent 0#u64 = ok false :=
  ⟨a_zero_second_level_entry_denies, a_zero_context_entry_denies⟩

/-! ### Building an entry

    Stated as the composition the walk performs rather than as an existential
    over the intermediate entry: what matters is what the encoder and the reader
    agree on, and that is one value.

    These are instantiated at representative frames rather than quantified over
    every frame. Quantifying needs a bitvector argument that two masks do not
    overlap, and neither reduction nor `bv_decide` reaches through the scalar and
    `Result` encoding the extraction produces. The frames below are the aligned
    case, a large aligned case, and a frame below a page, which is where a
    masking error would show. -/

/-- A leaf built with neither read nor write denies. The snoop bit is not a
    permission and does not make an entry present. -/
theorem a_leaf_without_permissions_denies :
    (do let e ← slLeaf 0x1000#u64 false false false; slPresent e) = ok false ∧
    (do let e ← slLeaf 0x40000000#u64 false false false; slPresent e) = ok false ∧
    (do let e ← slLeaf 0xFFF#u64 false false false; slPresent e) = ok false := by
  refine ⟨?_, ?_, ?_⟩ <;> encode

theorem a_snoop_only_leaf_denies :
    (do let e ← slLeaf 0x1000#u64 false false true; slPresent e) = ok false ∧
    (do let e ← slLeaf 0x40000000#u64 false false true; slPresent e) = ok false := by
  refine ⟨?_, ?_⟩ <;> encode

/-- Read alone makes an entry present, which is the format's rule and the reason
    there is no separate present bit to forget. -/
theorem a_readable_leaf_is_present :
    (do let e ← slLeaf 0x1000#u64 true false false; slPresent e) = ok true ∧
    (do let e ← slLeaf 0x40000000#u64 true false false; slPresent e) = ok true := by
  refine ⟨?_, ?_⟩ <;> encode

/-- So does write alone, so a write-only mapping is not silently a denial. -/
theorem a_writable_leaf_is_present :
    (do let e ← slLeaf 0x1000#u64 false true false; slPresent e) = ok true ∧
    (do let e ← slLeaf 0x40000000#u64 false true false; slPresent e) = ok true := by
  refine ⟨?_, ?_⟩ <;> encode

/-- A table entry is always present: its own bits do not restrict, the leaf
    decides. A walk never stops early on an interior entry it just wrote. -/
theorem a_table_entry_is_always_present :
    (do let e ← slTable 0x2000#u64; slPresent e) = ok true ∧
    (do let e ← slTable 0x40000000#u64; slPresent e) = ok true := by
  refine ⟨?_, ?_⟩ <;> encode

/-! ### Addresses and permissions do not mix -/

/-- Whatever permissions a leaf carries, the address read back out of it is the
    frame, masked to bits 51:12. The mask is applied by the encoder rather than
    assumed of the caller. -/
theorem the_address_survives_every_permission :
    (do let e ← slLeaf 0x40000000#u64 true true true; slAddress e) = ok 0x40000000#u64 ∧
    (do let e ← slLeaf 0x40000000#u64 false false false; slAddress e) = ok 0x40000000#u64 ∧
    (do let e ← slLeaf 0x1000#u64 true true true; slAddress e) = ok 0x1000#u64 := by
  refine ⟨?_, ?_, ?_⟩ <;> encode

/-- A frame below a page encodes to an address of zero rather than to its own low
    bits, so a misaligned frame cannot reach the permission field. -/
theorem a_misaligned_frame_loses_its_low_bits :
    (do let e ← slLeaf 0xFFF#u64 true true true; slAddress e) = ok 0#u64 := by
  encode

/-- And with no permissions it still denies, so masking the address away leaves
    the permissions to decide, which they do. -/
theorem a_misaligned_frame_still_denies :
    (do let e ← slLeaf 0xFFF#u64 false false false; slPresent e) = ok false := by
  encode

/-- A context entry's pointer is masked the same way, so the translation type and
    the present bit cannot be read as part of the second-level root. -/
theorem a_context_pointer_is_masked :
    (do let e ← ctxLow 0x3FFF#u64; ctxAddress e) = ok 0x3000#u64 ∧
    (do let e ← ctxLow 0x40000000#u64; ctxAddress e) = ok 0x40000000#u64 := by
  refine ⟨?_, ?_⟩ <;> encode

/-- And a root entry's the same. -/
theorem a_root_pointer_is_masked :
    (do let e ← rootLow 0x5FFF#u64; ctxAddress e) = ok 0x5000#u64 ∧
    (do let e ← rootLow 0x40000000#u64; ctxAddress e) = ok 0x40000000#u64 := by
  refine ⟨?_, ?_⟩ <;> encode

/-- A written context entry is present, so a device whose entry was programmed is
    translated rather than blocked. -/
theorem a_written_context_entry_is_present :
    (do let e ← ctxLow 0x3000#u64; ctxPresent e) = ok true ∧
    (do let e ← rootLow 0x5000#u64; ctxPresent e) = ok true := by
  refine ⟨?_, ?_⟩ <;> encode

/-! ### Identifying a device -/

/-- The domain id written into a context entry reads back unchanged, so two
    devices in different domains cannot be confused by the invalidation path that
    keys on it. Shown across the field's range and against every width. -/
theorem the_domain_id_round_trips :
    (do let h ← ctxHigh 1#u16 2#u8; ctxDomain h) = ok 1#u16 ∧
    (do let h ← ctxHigh 0xFFFF#u16 3#u8; ctxDomain h) = ok 0xFFFF#u16 ∧
    (do let h ← ctxHigh 0#u16 1#u8; ctxDomain h) = ok 0#u16 := by
  refine ⟨?_, ?_, ?_⟩ <;> encode

/-- The width occupies the low three bits and the domain starts at bit eight, so
    a width of seven beside a domain of one still reads back as domain one. -/
theorem the_width_does_not_disturb_the_domain :
    (do let h ← ctxHigh 1#u16 7#u8; ctxDomain h) = ok 1#u16 ∧
    (do let h ← ctxHigh 0xFFFF#u16 7#u8; ctxDomain h) = ok 0xFFFF#u16 := by
  refine ⟨?_, ?_⟩ <;> encode

theorem out_of_range_device_numbers_alias :
    ctxIndex 255#u8 255#u8 = ctxIndex 31#u8 7#u8 := by
  encode

/-! ### The level arithmetic, and why its failure is unreachable -/

/-- The span of one entry at each depth the hardware can be configured to, as a
    value rather than as an existential: reduction finishes a concrete equation
    where it stalls on a metavariable. -/
theorem the_span_is_total_at_every_real_depth :
    slSpan 3#u8 = ok 0x40000000#u64 ∧ slSpan 4#u8 = ok 0x8000000000#u64 ∧
      slSpan 5#u8 = ok 0x1000000000000#u64 := by
  refine ⟨?_, ?_, ?_⟩ <;> encode

/-- So the arithmetic is total on every depth that can reach it: composing the
    producer with the span always succeeds, and lands on one of three spans.

    The failure at level zero is real and unreachable, which is a different claim
    from absent, and this is the difference. -/
theorem the_producer_keeps_the_indexing_total (l : Agaw) :
    (do let d ← agaw_page_table_levels l; slSpan d) = ok 0x40000000#u64 ∨
      (do let d ← agaw_page_table_levels l; slSpan d) = ok 0x8000000000#u64 ∨
      (do let d ← agaw_page_table_levels l; slSpan d) = ok 0x1000000000000#u64 := by
  cases l
  · left; encode
  · right; left; encode
  · right; right; encode

theorem the_span_fails_at_level_zero : slSpan 0#u8 = fail Error.integerOverflow := by
  rfl

theorem the_depth_is_three_four_or_five (l : Agaw) :
    agaw_page_table_levels l = ok 3#u8 ∨ agaw_page_table_levels l = ok 4#u8 ∨
      agaw_page_table_levels l = ok 5#u8 := by
  cases l
  · exact Or.inl rfl
  · exact Or.inr (Or.inl rfl)
  · exact Or.inr (Or.inr rfl)

theorem the_context_width_fits_its_field (l : Agaw) :
    agaw_context_aw l = ok 1#u8 ∨ agaw_context_aw l = ok 2#u8 ∨
      agaw_context_aw l = ok 3#u8 := by
  cases l
  · exact Or.inl rfl
  · exact Or.inr (Or.inl rfl)
  · exact Or.inr (Or.inr rfl)

/-! ### The input width check -/

/-! ### Axiom profile -/

#print axioms NonosExtraction.zeroed_tables_deny_everything
#print axioms NonosExtraction.a_leaf_without_permissions_denies
#print axioms NonosExtraction.a_snoop_only_leaf_denies
#print axioms NonosExtraction.a_readable_leaf_is_present
#print axioms NonosExtraction.a_writable_leaf_is_present
#print axioms NonosExtraction.a_table_entry_is_always_present
#print axioms NonosExtraction.the_address_survives_every_permission
#print axioms NonosExtraction.a_misaligned_frame_loses_its_low_bits
#print axioms NonosExtraction.a_misaligned_frame_still_denies
#print axioms NonosExtraction.a_context_pointer_is_masked
#print axioms NonosExtraction.a_root_pointer_is_masked
#print axioms NonosExtraction.a_written_context_entry_is_present
#print axioms NonosExtraction.the_domain_id_round_trips
#print axioms NonosExtraction.out_of_range_device_numbers_alias
#print axioms NonosExtraction.the_span_fails_at_level_zero
#print axioms NonosExtraction.the_span_is_total_at_every_real_depth
#print axioms NonosExtraction.the_producer_keeps_the_indexing_total
#print axioms NonosExtraction.the_depth_is_three_four_or_five
#print axioms NonosExtraction.the_context_width_fits_its_field

end NonosExtraction
