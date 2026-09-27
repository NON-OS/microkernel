/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The UEFI variable attribute set, on the extracted code.

`VariableAttributes` decides what the firmware is asked to do with a variable:
whether it survives a reboot, whether the runtime can reach it, and whether a
write has to be authenticated. Secure boot's own variables are carried in it, so
the predicates below are read on a path where getting a flag wrong is a policy
decision rather than a cosmetic one.

It is a newtype over `u32` and the operations are masks, which means the
interesting properties hold for every value rather than at a handful, and can be
proven that way. Where a statement is about all four billion inputs it says so
by quantifying, not by listing witnesses.

Two things are worth reading closely. `from_bits_truncate` masks to the low eight
bits, which is exactly the eight flags the type defines, so it cannot manufacture
a flag the firmware has no name for. `from_bits` does not mask, so it can, and
`from_bits_admits_undefined_flags` is that stated plainly. Neither is a defect on
its own; the pair is a choice, and a caller reaching for the wrong one gets no
warning from the type.
-/

import NonosExtraction.Uefi

open Aeneas Aeneas.Std Result
open nonos_uefi_attrs

set_option linter.hashCommand false
set_option maxRecDepth 100000
set_option maxHeartbeats 4000000

namespace NonosExtraction.Uefi

open attributes.VariableAttributes renaming
  contains → holds, is_empty → isEmpty, from_bits → ofBits,
  from_bits_truncate → ofBitsTruncated, bits → bitsOf,
  intersection → meet, union → join, empty → none',
  is_non_volatile → isNonVolatile, is_runtime_access → isRuntimeAccess,
  requires_authentication → needsAuth

/-! ### The flag set as a lattice -/

/-- Every set contains itself, so a caller asking whether a variable has the
    attributes it was created with is never told no. -/
theorem containment_is_reflexive (a : Std.U32) : holds a a = ok true := by
  unfold holds
  simp only [Std.lift, bind_tc_ok]
  have : a &&& a = a := by bv_tac 32
  simp [this]

/-- The empty set is contained in every set. -/
theorem the_empty_set_is_contained_everywhere (a : Std.U32) :
    holds a 0#u32 = ok true := by
  unfold holds
  simp only [Std.lift, bind_tc_ok]
  have : a &&& 0#u32 = 0#u32 := by bv_tac 32
  simp [this]

/-- Containment is exactly the mask test, for every pair. -/
theorem containment_is_the_mask (a b : Std.U32) :
    holds a b = ok (decide ((a &&& b) = b)) := by
  unfold holds
  simp only [Std.lift, bind_tc_ok]

/-- The meet is below both of its arguments, so intersecting an attribute set can
    only ever drop flags. -/
theorem the_meet_is_below_both (a b : Std.U32) : meet a b = ok (a &&& b) := by
  unfold meet
  simp only [Std.lift, bind_tc_ok]

/-- And the join is above both, so a union never silently loses one. -/
theorem the_join_is_above_both (a b : Std.U32) : join a b = ok (a ||| b) := by
  unfold join
  simp only [Std.lift, bind_tc_ok]

/-- Anything the meet produces is contained in both arguments. Stated on the
    bitvectors, for every pair, because this is the property a caller relies on
    when it narrows a set before handing it to the firmware. -/
theorem the_meet_is_contained_in_both (a b : BitVec 32) :
    ((a &&& b) &&& a = a &&& b) ∧ ((a &&& b) &&& b = a &&& b) := by
  bv_decide

/-- The join contains both, likewise. -/
theorem the_join_contains_both (a b : BitVec 32) :
    ((a ||| b) &&& a = a) ∧ ((a ||| b) &&& b = b) := by
  bv_decide

/-! ### Emptiness -/

/-- A set is reported empty exactly when it holds no flag at all. -/
theorem emptiness_is_zero (a : Std.U32) : isEmpty a = ok (decide (a = 0#u32)) := by
  unfold isEmpty
  rfl

/-- And the constructor for the empty set really is empty. -/
theorem the_empty_constructor_is_empty : none' = ok 0#u32 := by rfl

/-! ### Round tripping through the raw word -/

/-- The bits go in and come back out unchanged, so nothing is lost by carrying an
    attribute set as a word across a firmware call. -/
theorem the_raw_word_round_trips (w : Std.U32) :
    (do let a ← ofBits w; bitsOf a) = ok w := by
  unfold ofBits bitsOf
  simp

/-! ### The two constructors differ, and the difference matters -/

/-- The truncating constructor masks to the low eight bits, which is exactly the
    eight flags this type defines, so it can never produce a flag the firmware
    has no name for. -/
theorem truncation_is_the_low_byte (w : Std.U32) :
    ofBitsTruncated w = ok (w &&& 255#u32) := by
  unfold ofBitsTruncated
  simp only [Std.lift, bind_tc_ok]

/-- And nothing above the eight defined flags survives it. -/
theorem truncation_keeps_only_defined_flags (w : BitVec 32) :
    (w &&& 0xFF#32) &&& 0xFFFFFF00#32 = 0#32 := by
  bv_decide

/-- The eight flags fit inside that mask, so truncation is lossless on anything
    the type can legitimately hold. -/
theorem truncation_loses_nothing_defined (w : BitVec 32)
    (h : w &&& 0xFFFFFF00#32 = 0#32) : w &&& 0xFF#32 = w := by
  bv_decide

/-- The plain constructor does not mask, so it will happily build a set carrying
    bits no flag corresponds to. A caller that reaches for it with a word from
    firmware keeps whatever the firmware put there. -/
theorem from_bits_admits_undefined_flags :
    ofBits 0x100#u32 = ok 0x100#u32 ∧ ofBitsTruncated 0x100#u32 = ok 0#u32 := by
  refine ⟨rfl, ?_⟩
  unfold ofBitsTruncated
  simp only [Std.lift, bind_tc_ok]
  rfl

/-! ### The named predicates agree with their flags -/

theorem non_volatile_is_bit_zero (a : Std.U32) :
    isNonVolatile a = ok (decide ((a &&& 1#u32) = 1#u32)) := by
  unfold isNonVolatile holds attributes.VariableAttributes.NON_VOLATILE
  simp only [Std.lift, bind_tc_ok]

theorem runtime_access_is_bit_two (a : Std.U32) :
    isRuntimeAccess a = ok (decide ((a &&& 4#u32) = 4#u32)) := by
  unfold isRuntimeAccess holds attributes.VariableAttributes.RUNTIME_ACCESS
  simp only [Std.lift, bind_tc_ok]

/-- Authentication is required when either authenticated-write flag is present,
    and the witnesses cover each one alone, both together, and neither. -/
theorem authentication_is_either_flag :
    needsAuth 0x20#u32 = ok true ∧ needsAuth 0x80#u32 = ok true ∧
    needsAuth 0xA0#u32 = ok true ∧ needsAuth 0x00#u32 = ok false ∧
    needsAuth 0x5F#u32 = ok false := by
  refine ⟨?_, ?_, ?_, ?_, ?_⟩ <;>
    (unfold needsAuth holds attributes.VariableAttributes.TIME_BASED_AUTHENTICATED_WRITE_ACCESS
       attributes.VariableAttributes.ENHANCED_AUTHENTICATED_ACCESS
     simp only [Std.lift, bind_tc_ok]
     rfl)

/-! ### The forwarding functions add nothing

    Charon cannot take an inherent method as an entry point, so the crate root
    carries a free function per method and those are what the manifest lists. A
    wrapper that quietly did something other than call through would make every
    theorem above a statement about code nobody runs. Each one is proven to be
    its method, by definition.
-/

theorem the_empty_wrapper_is_its_method :
    variableattributes_empty  = none'  := rfl

theorem the_bits_wrapper_is_its_method (a : Std.U32) :
    variableattributes_bits a = bitsOf a := rfl

theorem the_from_bits_wrapper_is_its_method (a : Std.U32) :
    variableattributes_from_bits a = ofBits a := rfl

theorem the_from_bits_truncate_wrapper_is_its_method (a : Std.U32) :
    variableattributes_from_bits_truncate a = ofBitsTruncated a := rfl

theorem the_contains_wrapper_is_its_method (a b : Std.U32) :
    variableattributes_contains a b = holds a b := rfl

theorem the_is_empty_wrapper_is_its_method (a : Std.U32) :
    variableattributes_is_empty a = isEmpty a := rfl

theorem the_is_non_volatile_wrapper_is_its_method (a : Std.U32) :
    variableattributes_is_non_volatile a = isNonVolatile a := rfl

theorem the_is_runtime_access_wrapper_is_its_method (a : Std.U32) :
    variableattributes_is_runtime_access a = isRuntimeAccess a := rfl

theorem the_requires_authentication_wrapper_is_its_method (a : Std.U32) :
    variableattributes_requires_authentication a = needsAuth a := rfl

theorem the_intersection_wrapper_is_its_method (a b : Std.U32) :
    variableattributes_intersection a b = meet a b := rfl

theorem the_union_wrapper_is_its_method (a b : Std.U32) :
    variableattributes_union a b = join a b := rfl

/-! ### Axiom profile -/

#print axioms NonosExtraction.Uefi.the_empty_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_bits_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_from_bits_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_from_bits_truncate_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_contains_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_is_empty_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_is_non_volatile_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_is_runtime_access_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_requires_authentication_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_intersection_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.the_union_wrapper_is_its_method
#print axioms NonosExtraction.Uefi.containment_is_reflexive
#print axioms NonosExtraction.Uefi.the_empty_set_is_contained_everywhere
#print axioms NonosExtraction.Uefi.containment_is_the_mask
#print axioms NonosExtraction.Uefi.the_meet_is_below_both
#print axioms NonosExtraction.Uefi.the_join_is_above_both
#print axioms NonosExtraction.Uefi.the_meet_is_contained_in_both
#print axioms NonosExtraction.Uefi.the_join_contains_both
#print axioms NonosExtraction.Uefi.emptiness_is_zero
#print axioms NonosExtraction.Uefi.the_empty_constructor_is_empty
#print axioms NonosExtraction.Uefi.the_raw_word_round_trips
#print axioms NonosExtraction.Uefi.truncation_is_the_low_byte
#print axioms NonosExtraction.Uefi.truncation_keeps_only_defined_flags
#print axioms NonosExtraction.Uefi.truncation_loses_nothing_defined
#print axioms NonosExtraction.Uefi.from_bits_admits_undefined_flags
#print axioms NonosExtraction.Uefi.non_volatile_is_bit_zero
#print axioms NonosExtraction.Uefi.runtime_access_is_bit_two
#print axioms NonosExtraction.Uefi.authentication_is_either_flag

end NonosExtraction.Uefi
