/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Two page-descriptor backends, and that they mean the same thing.

The shared paging manager states what a mapping should be in one neutral flag
vocabulary, `arch::paging::descriptor::flags`. Each architecture turns that into
its own descriptor bits. On x86_64 the vocabulary is the hardware layout, so the
translation is nearly the identity. On aarch64 almost nothing lines up: write
permission is the *absence* of `AP_READ_ONLY`, user reachability is a set
`AP_EL0`, a page rather than a block is a set `TABLE_OR_PAGE`, and execute
permission is denied by setting `PXN` and `UXN` rather than granted by clearing
a bit. Three of those run opposite to the x86_64 rule.

Opposite polarities that nothing checks are how an earlier revision of the
aarch64 encoder published every kernel page to userspace. So the theorem this
file exists for is `the_backends_agree`: for the flag words the manager actually
emits, both encoders produce descriptors that read back with the same meaning.
The bits differ on almost every one of them and the semantics do not, for leaves,
and for interior table entries since the aarch64 `table` honours its
`user_accessible` argument (`the_backends_agree_on_tables`). Before that it dropped
the argument, and `the_aarch64_table_ignores_the_request` is kept, stated on that
old shape, so the defect stays named and a return to it has a theorem to break.

Everything is stated on the extracted definitions, so these are the encoders the
kernel calls rather than a model of them. The flag words are instantiated rather
than quantified, for the reason given in `IommuRefinement`: proving a mask
property for every `u64` needs a bitvector argument that neither reduction nor
`bv_decide` reaches through the scalar and `Result` encoding. The words below are
the ones the manager emits, plus the absent case for each permission, which is
where a polarity error shows.
-/

import NonosExtraction.Paging

open Aeneas Aeneas.Std Result
open nonos_paging

set_option linter.hashCommand false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000

namespace NonosExtraction

/-! ### Names

    A namespace cannot be abbreviated, so the two backends are opened under short
    names. The generated constants are `irreducible`, which is why `emit` names
    them before letting the kernel finish. -/

open arch.paging.descriptor.x86_64 renaming
  leaf → xLeaf, table → xTable, is_present → xPresent, is_block → xBlock,
  address → xAddress, is_writable → xWritable, is_executable → xExecutable,
  is_user → xUser, table_grants_user → xTableGrantsUser

open arch.paging.descriptor.aarch64.read renaming
  is_present → aPresent, is_block → aBlock, address → aAddress,
  is_writable → aWritable, is_executable → aExecutable, is_user → aUser,
  table_grants_user → aTableGrantsUser

attribute [local simp]
  arch.paging.descriptor.flags.PRESENT
  arch.paging.descriptor.flags.WRITABLE
  arch.paging.descriptor.flags.USER
  arch.paging.descriptor.flags.NO_CACHE
  arch.paging.descriptor.flags.HUGE
  arch.paging.descriptor.flags.GLOBAL
  arch.paging.descriptor.flags.NO_EXECUTE
  arch.paging.descriptor.x86_64.ADDR_MASK
  arch.paging.descriptor.x86_64.FLAGS_MASK
  arch.paging.descriptor.aarch64.bits.ADDR_MASK
  arch.paging.descriptor.aarch64.bits.VALID
  arch.paging.descriptor.aarch64.bits.TABLE_OR_PAGE
  arch.paging.descriptor.aarch64.bits.AP_EL0
  arch.paging.descriptor.aarch64.bits.AP_READ_ONLY
  arch.paging.descriptor.aarch64.bits.SH_INNER
  arch.paging.descriptor.aarch64.bits.AF
  arch.paging.descriptor.aarch64.bits.NOT_GLOBAL
  arch.paging.descriptor.aarch64.bits.PXN
  arch.paging.descriptor.aarch64.bits.UXN
  arch.paging.descriptor.aarch64.bits.APTABLE_NO_EL0
  xLeaf xTable xPresent xBlock xAddress xWritable xExecutable xUser
  xTableGrantsUser
  aPresent aBlock aAddress aWritable aExecutable aUser aTableGrantsUser
  aarch64_leaf aarch64_table
  arch.paging.descriptor.aarch64.build.leaf
  arch.paging.descriptor.aarch64.build.table
  arch.paging.descriptor.aarch64.build.execute_never
  arch.aarch64.mmu.attributes.kind.MemoryType.attr_index

/-- Unfold the generated constants, then let the kernel finish the closed
    arithmetic the unfolding exposes. Named `emit` rather than `encode` because
    `IommuRefinement` already defines the latter in this namespace. -/
macro "emit" : tactic => `(tactic| first | (simp; done) | (simp; rfl))

/-! ### The flag words the manager emits

    Named so a theorem below reads as the mapping it is about rather than as a
    number. The neutral bits are PRESENT 1, WRITABLE 2, USER 4, NO_CACHE 16,
    HUGE 128 and NO_EXECUTE at bit 63. -/

/-- A kernel page, readable and writable, not executable. The ordinary data
    mapping. -/
abbrev kernelData : Std.U64 := 0x8000000000000003#u64

/-- A kernel page, read only and executable. The ordinary text mapping. -/
abbrev kernelText : Std.U64 := 0x1#u64

/-- A user page, readable and writable, not executable. -/
abbrev userData : Std.U64 := 0x8000000000000007#u64

/-- A user page, read only and executable. -/
abbrev userText : Std.U64 := 0x5#u64

/-- Nothing at all: the flag word for an absent mapping. -/
abbrev absent : Std.U64 := 0x0#u64

/-! ### Presence -/

/-- Neither backend calls an absent mapping present, so a descriptor built from a
    zero flag word denies rather than faulting open. -/
theorem absence_is_absence :
    (do let e ← xLeaf 0x1000#u64 absent; xPresent e) = ok false ∧
    (do let e ← aarch64_leaf 0x1000#u64 absent; aPresent e) = ok false := by
  refine ⟨?_, ?_⟩ <;> emit

/-- And both call a present mapping present, on each of the four words. -/
theorem presence_is_presence :
    (do let e ← xLeaf 0x1000#u64 kernelData; xPresent e) = ok true ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelData; aPresent e) = ok true ∧
    (do let e ← xLeaf 0x1000#u64 userText; xPresent e) = ok true ∧
    (do let e ← aarch64_leaf 0x1000#u64 userText; aPresent e) = ok true := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> emit

/-! ### User reachability, in both polarities

    This is the property whose polarity was inverted. On x86_64 a user page has
    `USER` set; on aarch64 it has `AP_EL0` set, which is a different bit reached
    by a different branch. A kernel page must come out unreachable from EL0 and
    ring 3 on both. -/

/-- A kernel mapping is not user reachable, on either architecture. -/
theorem a_kernel_page_never_reaches_userspace :
    (do let e ← xLeaf 0x1000#u64 kernelData; xUser e) = ok false ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelData; aUser e) = ok false ∧
    (do let e ← xLeaf 0x1000#u64 kernelText; xUser e) = ok false ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelText; aUser e) = ok false := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> emit

/-- And a user mapping is, so the encoders are gates rather than walls. -/
theorem a_user_page_reaches_userspace :
    (do let e ← xLeaf 0x1000#u64 userData; xUser e) = ok true ∧
    (do let e ← aarch64_leaf 0x1000#u64 userData; aUser e) = ok true := by
  refine ⟨?_, ?_⟩ <;> emit

/-! ### Write permission, in both polarities

    On x86_64 writable is `WRITABLE` set. On aarch64 it is `AP_READ_ONLY`
    *clear*, so the encoder sets a bit to take the permission away. -/

theorem write_permission_agrees :
    (do let e ← xLeaf 0x1000#u64 kernelData; xWritable e) = ok true ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelData; aWritable e) = ok true ∧
    (do let e ← xLeaf 0x1000#u64 kernelText; xWritable e) = ok false ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelText; aWritable e) = ok false := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> emit

/-! ### Execute permission, in both polarities

    On x86_64 execution is denied by `NO_EXECUTE`. On aarch64 it is denied by
    `PXN` and `UXN`, two bits, and the builder has to set the right one for the
    privilege level the page belongs to. -/

theorem execute_permission_agrees :
    (do let e ← xLeaf 0x1000#u64 kernelText; xExecutable e) = ok true ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelText; aExecutable e) = ok true ∧
    (do let e ← xLeaf 0x1000#u64 kernelData; xExecutable e) = ok false ∧
    (do let e ← aarch64_leaf 0x1000#u64 kernelData; aExecutable e) = ok false := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> emit

/-! ### The theorem the file exists for -/

/-- For each flag word the manager emits, the two backends produce descriptors
    that read back identically: same presence, same write permission, same user
    reachability, same execute permission.

    The bits differ on almost every one of them. Three of the four rules run in
    opposite directions. This is the statement that the arch boundary is a
    translation and not a reimplementation, and it is the statement an inverted
    polarity breaks. -/
theorem the_backends_agree :
    ((do let e ← xLeaf 0x1000#u64 kernelData; xPresent e) =
       (do let e ← aarch64_leaf 0x1000#u64 kernelData; aPresent e)) ∧
    ((do let e ← xLeaf 0x1000#u64 kernelData; xWritable e) =
       (do let e ← aarch64_leaf 0x1000#u64 kernelData; aWritable e)) ∧
    ((do let e ← xLeaf 0x1000#u64 kernelData; xUser e) =
       (do let e ← aarch64_leaf 0x1000#u64 kernelData; aUser e)) ∧
    ((do let e ← xLeaf 0x1000#u64 kernelData; xExecutable e) =
       (do let e ← aarch64_leaf 0x1000#u64 kernelData; aExecutable e)) := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> emit

/-- The same, on a user mapping, which is the direction the historical defect
    got wrong. -/
theorem the_backends_agree_on_a_user_page :
    ((do let e ← xLeaf 0x1000#u64 userData; xPresent e) =
       (do let e ← aarch64_leaf 0x1000#u64 userData; aPresent e)) ∧
    ((do let e ← xLeaf 0x1000#u64 userData; xUser e) =
       (do let e ← aarch64_leaf 0x1000#u64 userData; aUser e)) ∧
    ((do let e ← xLeaf 0x1000#u64 userData; xWritable e) =
       (do let e ← aarch64_leaf 0x1000#u64 userData; aWritable e)) := by
  refine ⟨?_, ?_, ?_⟩ <;> emit

/-- And on executable text, where the aarch64 side has to pick between two
    execute-never bits. -/
theorem the_backends_agree_on_text :
    ((do let e ← xLeaf 0x1000#u64 userText; xExecutable e) =
       (do let e ← aarch64_leaf 0x1000#u64 userText; aExecutable e)) ∧
    ((do let e ← xLeaf 0x1000#u64 kernelText; xExecutable e) =
       (do let e ← aarch64_leaf 0x1000#u64 kernelText; aExecutable e)) := by
  refine ⟨?_, ?_⟩ <;> emit

/-! ### Addresses stay out of the flags -/

/-- Both encoders mask the physical address before they set a flag, so a
    misaligned frame cannot reach the permission bits. On a frame below a page
    the address reads back as zero on both. -/
theorem a_misaligned_frame_loses_its_low_bits_in_the_descriptor :
    (do let e ← xLeaf 0xFFF#u64 kernelData; xAddress e) = ok 0#u64 ∧
    (do let e ← aarch64_leaf 0xFFF#u64 kernelData; aAddress e) = ok 0#u64 := by
  refine ⟨?_, ?_⟩ <;> emit

/-- An aligned frame survives on both, so the masking costs nothing on a correct
    call. -/
theorem an_aligned_frame_round_trips :
    (do let e ← xLeaf 0x40000000#u64 kernelData; xAddress e) = ok 0x40000000#u64 ∧
    (do let e ← aarch64_leaf 0x40000000#u64 kernelData; aAddress e) =
      ok 0x40000000#u64 := by
  refine ⟨?_, ?_⟩ <;> emit

/-! ### Table entries -/

/-- x86_64 honours the request: an interior entry built without user access is
    not read as granting it. -/
theorem the_x86_table_honours_the_request :
    (do let e ← xTable 0x2000#u64 false; xTableGrantsUser e) = ok false ∧
    (do let e ← xTable 0x2000#u64 true; xTableGrantsUser e) = ok true := by
  refine ⟨?_, ?_⟩ <;> emit

/-- aarch64 honours it too. A table built without user access sets APTable[0],
    the bit `table_grants_user` reads, so the refusal carries to the walk and to
    the usercopy check that reads the same bit. -/
theorem the_aarch64_table_honours_the_request :
    (do let e ← aarch64_table 0x2000#u64 false; aTableGrantsUser e) = ok false ∧
    (do let e ← aarch64_table 0x2000#u64 true; aTableGrantsUser e) = ok true := by
  refine ⟨?_, ?_⟩ <;> emit

/-- So the backends agree on interior entries as well as on leaves, for both
    answers to the request. -/
theorem the_backends_agree_on_tables :
    ((do let e ← xTable 0x2000#u64 false; xTableGrantsUser e) =
       (do let e ← aarch64_table 0x2000#u64 false; aTableGrantsUser e)) ∧
    ((do let e ← xTable 0x2000#u64 true; xTableGrantsUser e) =
       (do let e ← aarch64_table 0x2000#u64 true; aTableGrantsUser e)) := by
  refine ⟨?_, ?_⟩ <;> emit

/-! ### The shape the aarch64 table had

    Kept so the defect stays named and a regression to it has a theorem to
    contradict, the device `CtRefinement` uses for `oldShortcut`. -/

/-- The body `table` had before it honoured the request, as Aeneas extracted it:
    the address, `VALID` and `TABLE_OR_PAGE`, and nothing that reads
    `user_accessible`. -/
def oldAarch64Table (pa : Std.U64) (_user_accessible : Bool) : Result Std.U64 := do
  let i ← lift (pa &&& arch.paging.descriptor.aarch64.bits.ADDR_MASK)
  let i1 ← arch.paging.descriptor.aarch64.bits.VALID
  let i2 ← lift (i ||| i1)
  let i3 ← arch.paging.descriptor.aarch64.bits.TABLE_OR_PAGE
  ok (i2 ||| i3)

attribute [local simp] oldAarch64Table

/-- That shape did not honour it, and this was the leaf polarity defect one level
    up.

    It took its second argument as `_user_accessible` and dropped it, and
    `table_grants_user` reads `APTABLE_NO_EL0`, a bit that body never set. So every
    interior entry it built granted EL0 access, whatever the caller asked for, and
    the two calls below were indistinguishable.

    An interior entry that does not restrict EL0 does not by itself publish a
    kernel page, because the leaf still has to permit it and
    `a_kernel_page_never_reaches_userspace` says it does not. It removed the
    second of the two barriers. -/
theorem the_aarch64_table_ignores_the_request :
    (do let e ← oldAarch64Table 0x2000#u64 false; aTableGrantsUser e) = ok true ∧
    (do let e ← oldAarch64Table 0x2000#u64 true; aTableGrantsUser e) = ok true ∧
    (do let e ← oldAarch64Table 0x2000#u64 false; aTableGrantsUser e) =
      (do let e ← oldAarch64Table 0x2000#u64 true; aTableGrantsUser e) := by
  refine ⟨?_, ?_, ?_⟩ <;> emit

/-- Under that shape the backends disagreed on interior entries, which is why
    `the_backends_agree` was stated about leaves only. -/
theorem the_backends_disagree_on_tables :
    (do let e ← xTable 0x2000#u64 false; xTableGrantsUser e) ≠
      (do let e ← oldAarch64Table 0x2000#u64 false; aTableGrantsUser e) := by
  rw [show (do let e ← xTable 0x2000#u64 false; xTableGrantsUser e) = ok false from
        by emit,
      show (do let e ← oldAarch64Table 0x2000#u64 false; aTableGrantsUser e) = ok true from
        by emit]
  simp

/-- And the table the kernel builds is not that shape: on the request that
    matters they answer differently. -/
theorem the_table_is_not_the_old_shape :
    (do let e ← aarch64_table 0x2000#u64 false; aTableGrantsUser e) ≠
      (do let e ← oldAarch64Table 0x2000#u64 false; aTableGrantsUser e) := by
  rw [show (do let e ← aarch64_table 0x2000#u64 false; aTableGrantsUser e) = ok false from
        by emit,
      show (do let e ← oldAarch64Table 0x2000#u64 false; aTableGrantsUser e) = ok true from
        by emit]
  simp

/-- Both table encoders produce a present entry, so a walk does not stop early on
    an interior entry it just wrote. -/
theorem a_table_entry_is_present :
    (do let e ← xTable 0x2000#u64 false; xPresent e) = ok true ∧
    (do let e ← aarch64_table 0x2000#u64 false; aPresent e) = ok true := by
  refine ⟨?_, ?_⟩ <;> emit

/-! ### Axiom profile -/

#print axioms NonosExtraction.absence_is_absence
#print axioms NonosExtraction.presence_is_presence
#print axioms NonosExtraction.a_kernel_page_never_reaches_userspace
#print axioms NonosExtraction.a_user_page_reaches_userspace
#print axioms NonosExtraction.write_permission_agrees
#print axioms NonosExtraction.execute_permission_agrees
#print axioms NonosExtraction.the_backends_agree
#print axioms NonosExtraction.the_backends_agree_on_a_user_page
#print axioms NonosExtraction.the_backends_agree_on_text
#print axioms NonosExtraction.a_misaligned_frame_loses_its_low_bits_in_the_descriptor
#print axioms NonosExtraction.an_aligned_frame_round_trips
#print axioms NonosExtraction.the_x86_table_honours_the_request
#print axioms NonosExtraction.the_aarch64_table_honours_the_request
#print axioms NonosExtraction.the_backends_agree_on_tables
#print axioms NonosExtraction.the_aarch64_table_ignores_the_request
#print axioms NonosExtraction.the_backends_disagree_on_tables
#print axioms NonosExtraction.the_table_is_not_the_old_shape
#print axioms NonosExtraction.a_table_entry_is_present

end NonosExtraction
