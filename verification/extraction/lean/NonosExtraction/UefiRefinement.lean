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
import NonosExtraction.Bits

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

/-! ### The operations read against one another, and against their callers

    The theorems above read each operation on its own and are stated on the
    methods. These are stated on the crate-root functions the manifest counts,
    and most of them read one extracted function against another: containment
    is the subset order on bit positions and agrees with the order union
    defines, union and intersection are the least upper and greatest lower
    bounds for it, the empty set is the identity for union and the only set
    every set contains, the two constructors agree on exactly the words below
    256, and each named predicate reads exactly the bit or bits its flag names.
    Most are stated for every word or every pair, so a mask swapped for a
    neighbour, an argument order reversed, or a union computed as an exclusive
    or makes one of them false.

    Two are about how the kernel uses the type. `set_variable` hands `bits()`
    straight to firmware `SetVariable`, and `delete_variable` passes `NONE`,
    which is `Self(0)` as `empty()` is; UEFI deletes a variable written with no
    attributes, so the empty set has to reach firmware as the zero word. And
    `UefiManager::get_variable` and the secure boot prefill in
    `manager/init.rs` read the attributes `GetVariable` reports into a local and
    then drop them, building every cached `UefiVariable` with
    `DEFAULT_NV_BS_RT`. The last theorem records what the predicates answer on
    that word and on the words firmware reports for the secure boot variables.

    What these cannot establish: anything about the manager itself. Its cache,
    its locks and the firmware call through a raw function pointer are not
    extracted, and neither are `NONE`, `BOOTSERVICE_ACCESS`, `APPEND_WRITE`,
    `DEFAULT_NV_BS_RT`, the operator impls (`BitOr`, `BitAnd`, `BitXor`, `Not`
    and their assigning forms), or the mutating `insert`, `remove`, `toggle` and
    `set`. Where a statement needs one of those words it is built from the
    extracted constants and `union`, or written as a literal and named. The
    attribute words firmware reports are taken from the UEFI specification, not
    from any code here.
-/

private theorem words_are_equal_when_their_bits_are (x y : Std.U32) :
    x = y ↔ ∀ i, x.val.testBit i = y.val.testBit i :=
  ⟨fun h _ => h ▸ rfl, fun h => UScalar.eq_of_val_eq (Nat.eq_of_testBit_eq h)⟩

private theorem carrying_a_mask_is_carrying_its_bits (a b : Std.U32) :
    (a &&& b = b) ↔ ∀ i, b.val.testBit i = true → a.val.testBit i = true := by
  rw [words_are_equal_when_their_bits_are]
  simp only [UScalar.val_and, Nat.testBit_and]
  constructor
  · intro h i hb
    have := h i
    rw [hb, Bool.and_true] at this
    exact this
  · intro h i
    cases hb : b.val.testBit i
    · simp
    · simp [h i hb]

/-- A set contains another exactly when every bit of the other is a bit of it.
    Containment reads its receiver as the larger set: `a.contains(b)` asks
    whether `a` holds all of `b`. A version that compared `self & other` with
    `self` would answer the reverse question and still type check. -/
theorem holding_a_set_is_holding_each_of_its_bits (a b : Std.U32) :
    ∃ r, variableattributes_contains a b = ok r ∧
      (r = true ↔ ∀ i, b.val.testBit i = true → a.val.testBit i = true) := by
  unfold variableattributes_contains holds
  simp only [Std.lift, bind_tc_ok]
  exact ⟨_, rfl, by rw [decide_eq_true_iff]; exact carrying_a_mask_is_carrying_its_bits a b⟩

/-- Containment is the order union defines: `a` contains `b` exactly when adding
    `b` to `a` changes nothing. Two extracted functions compute the same
    relation in two ways, and they agree on every pair. -/
theorem containment_is_the_order_union_defines (a b : Std.U32) :
    variableattributes_contains a b =
      (do let u ← variableattributes_union a b; ok (decide (u = a))) := by
  unfold variableattributes_contains variableattributes_union holds join
  simp only [Std.lift, bind_tc_ok, ok.injEq, decide_eq_decide]
  rw [carrying_a_mask_is_carrying_its_bits, words_are_equal_when_their_bits_are]
  simp only [UScalar.val_or, Nat.testBit_or]
  constructor
  · intro h i
    cases hb : b.val.testBit i
    · simp
    · simp [h i hb]
  · intro h i hb
    have := h i
    rw [hb, Bool.or_true] at this
    exact this.symm

/-- Union is the least upper bound: a set contains a union exactly when it
    contains both halves. A caller that asks for a compound set of flags gets
    the same answer as asking for each. -/
theorem a_set_contains_a_union_exactly_when_it_contains_both (a b c : Std.U32) :
    (do let u ← variableattributes_union a b; variableattributes_contains c u) =
      (do let x ← variableattributes_contains c a
          let y ← variableattributes_contains c b
          ok (x && y)) := by
  unfold variableattributes_contains variableattributes_union holds join
  simp only [Std.lift, bind_tc_ok, ok.injEq, ← Bool.decide_and, decide_eq_decide]
  rw [carrying_a_mask_is_carrying_its_bits, carrying_a_mask_is_carrying_its_bits,
    carrying_a_mask_is_carrying_its_bits]
  simp only [UScalar.val_or, Nat.testBit_or, Bool.or_eq_true, or_imp, forall_and]

/-- Intersection is the greatest lower bound: the intersection of two sets
    contains a set exactly when both of them do, so narrowing a set before a
    firmware call cannot add a flag that either side lacked. -/
theorem the_intersection_contains_exactly_what_both_contain (a b c : Std.U32) :
    (do let m ← variableattributes_intersection a b; variableattributes_contains m c) =
      (do let x ← variableattributes_contains a c
          let y ← variableattributes_contains b c
          ok (x && y)) := by
  unfold variableattributes_contains variableattributes_intersection holds meet
  simp only [Std.lift, bind_tc_ok, ok.injEq, ← Bool.decide_and, decide_eq_decide]
  rw [carrying_a_mask_is_carrying_its_bits, carrying_a_mask_is_carrying_its_bits,
    carrying_a_mask_is_carrying_its_bits]
  simp only [UScalar.val_and, Nat.testBit_and, Bool.and_eq_true, imp_and, forall_and]

/-- Non-volatility is bit 0 of the word, and nothing else. -/
theorem non_volatility_is_bit_zero_of_the_word (a : Std.U32) :
    variableattributes_is_non_volatile a = ok (a.val.testBit 0) := by
  unfold variableattributes_is_non_volatile isNonVolatile holds
    attributes.VariableAttributes.NON_VOLATILE
  simp only [Std.lift, bind_tc_ok]
  rw [Bits.reads_bit_eq a 1#u32 0 rfl]

/-- Runtime access is bit 2 of the word, and nothing else; in particular
    `BOOTSERVICE_ACCESS`, bit 1, does not count. -/
theorem runtime_access_is_bit_two_of_the_word (a : Std.U32) :
    variableattributes_is_runtime_access a = ok (a.val.testBit 2) := by
  unfold variableattributes_is_runtime_access isRuntimeAccess holds
    attributes.VariableAttributes.RUNTIME_ACCESS
  simp only [Std.lift, bind_tc_ok]
  rw [Bits.reads_bit_eq a 4#u32 2 rfl]

/-- Authentication is required exactly when bit 5
    (`TIME_BASED_AUTHENTICATED_WRITE_ACCESS`) or bit 7
    (`ENHANCED_AUTHENTICATED_ACCESS`) is set, for every word. This replaces the
    five witnesses of `authentication_is_either_flag` with the closed form. -/
theorem authentication_is_bit_five_or_bit_seven (a : Std.U32) :
    variableattributes_requires_authentication a =
      ok (a.val.testBit 5 || a.val.testBit 7) := by
  unfold variableattributes_requires_authentication needsAuth holds
    attributes.VariableAttributes.TIME_BASED_AUTHENTICATED_WRITE_ACCESS
    attributes.VariableAttributes.ENHANCED_AUTHENTICATED_ACCESS
  simp only [Std.lift, bind_tc_ok]
  rw [Bits.reads_bit_eq a 32#u32 5 rfl, Bits.reads_bit_eq a 128#u32 7 rfl]
  cases a.val.testBit 5 <;> rfl

/-- Records a gap, or at least a choice that should be visible. The older
    counter-based flag, `AUTHENTICATED_WRITE_ACCESS` (bit 4, `0x10`), is not
    counted: a set carrying only it is answered as needing no authentication,
    while the same set with bit 5 added is answered as needing it. UEFI 2.3.1
    deprecates that flag, so this may be intended, but firmware can still report
    it on an existing variable. No kernel caller asks `requires_authentication`
    today. -/
theorem the_counter_based_authenticated_write_flag_is_not_counted :
    variableattributes_requires_authentication 0x10#u32 = ok false ∧
    variableattributes_requires_authentication 0x30#u32 = ok true := by
  unfold variableattributes_requires_authentication needsAuth holds
    attributes.VariableAttributes.TIME_BASED_AUTHENTICATED_WRITE_ACCESS
    attributes.VariableAttributes.ENHANCED_AUTHENTICATED_ACCESS
  exact ⟨rfl, rfl⟩

/-- A set is empty exactly when every set contains it, and exactly when it is
    what `empty` builds. Emptiness looks at the whole word, not only the eight
    defined flags: a word carrying only an undefined bit is not empty, because
    the empty set does not contain it. -/
theorem a_set_is_empty_exactly_when_every_set_contains_it (a : Std.U32) :
    (variableattributes_is_empty a = ok true ↔ ∀ c, variableattributes_contains c a = ok true) ∧
    (variableattributes_is_empty a = ok true ↔ variableattributes_empty = ok a) := by
  unfold variableattributes_is_empty isEmpty variableattributes_contains holds
    variableattributes_empty none'
  simp only [Std.lift, bind_tc_ok, ok.injEq, decide_eq_true_eq]
  refine ⟨⟨fun h c => ?_, fun h => ?_⟩, ⟨fun h => h.symm, fun h => h.symm⟩⟩
  · subst h
    apply UScalar.eq_of_val_eq
    simp
  · have hv := congrArg UScalar.val (h 0#u32)
    simp only [UScalar.val_and] at hv
    apply UScalar.eq_of_val_eq
    rw [← hv]
    simp

/-- The empty set is the identity for union and carries none of the named
    flags. `Default` returns `empty()`, so a set a caller starts from and builds
    up by union asks the firmware for exactly what was added to it. -/
theorem the_empty_set_is_the_identity_for_union_and_holds_no_flag (a : Std.U32) :
    (do let e ← variableattributes_empty; variableattributes_union a e) = ok a ∧
    (do let e ← variableattributes_empty; variableattributes_is_empty e) = ok true ∧
    (do let e ← variableattributes_empty; variableattributes_is_non_volatile e) = ok false ∧
    (do let e ← variableattributes_empty; variableattributes_is_runtime_access e) = ok false ∧
    (do let e ← variableattributes_empty;
        variableattributes_requires_authentication e) = ok false := by
  unfold variableattributes_empty none' variableattributes_union join
  simp only [Std.lift, bind_tc_ok, non_volatility_is_bit_zero_of_the_word,
    runtime_access_is_bit_two_of_the_word, authentication_is_bit_five_or_bit_seven]
  refine ⟨?_, rfl, rfl, rfl, rfl⟩
  congr 1
  apply UScalar.eq_of_val_eq
  simp

/-- `delete_variable` calls `set_variable` with `NONE`, which is `Self(0)` as
    `empty()` is, and `set_variable` passes `bits()` to firmware unchanged. UEFI
    deletes a variable written with no attributes, so this word has to be zero:
    a `bits` that forced a flag on, or an `empty` that carried one, would turn
    every deletion into a write. -/
theorem an_empty_set_reaches_firmware_as_the_zero_word :
    (do let e ← variableattributes_empty; variableattributes_bits e) = ok 0#u32 := rfl

/-- The word `bits` returns for a truncated set is the low byte of the word it
    was built from, for every word: 255 passes unchanged and 256 comes back as
    zero, so a set built by truncation can never hand `SetVariable` a bit at or
    above bit 8. No kernel caller builds a set this way today; the manager's
    writes use the named constants or the set its own caller passed. -/
theorem a_truncated_word_reaches_firmware_as_its_low_byte (w : Std.U32) :
    ∃ r, (do let a ← variableattributes_from_bits_truncate w; variableattributes_bits a) = ok r ∧
      r.val = w.val % 256 := by
  unfold variableattributes_from_bits_truncate ofBitsTruncated variableattributes_bits bitsOf
  simp only [Std.lift, bind_tc_ok]
  exact ⟨_, rfl, Bits.land_low_mask w 255#u32 8 rfl⟩

/-- The two constructors agree on a word exactly when it is below 256, that is,
    exactly when it carries no bit the type has no flag for. The boundary is
    sharp: they agree at 255 and differ at 256. -/
theorem the_two_constructors_agree_exactly_on_words_below_256 (w : Std.U32) :
    variableattributes_from_bits w = variableattributes_from_bits_truncate w ↔ w.val < 256 := by
  have hw : (w &&& 255#u32).val = w.val % 2 ^ 8 := Bits.land_low_mask w 255#u32 8 rfl
  unfold variableattributes_from_bits ofBits variableattributes_from_bits_truncate ofBitsTruncated
  simp only [Std.lift, bind_tc_ok, ok.injEq]
  constructor
  · intro h
    have := congrArg UScalar.val h
    rw [hw] at this
    omega
  · intro h
    apply UScalar.eq_of_val_eq
    rw [hw]
    omega

/-- Truncation changes no test of a defined flag: for every word, and every set
    of flags below 256, the truncated word contains the set exactly when the
    original does. The bound is exact, because at 256, the first bit with no
    flag, the two answers differ. -/
theorem truncation_changes_no_test_of_a_defined_flag :
    (∀ w b : Std.U32, b.val < 256 →
      (do let t ← variableattributes_from_bits_truncate w; variableattributes_contains t b) =
        variableattributes_contains w b) ∧
    (do let t ← variableattributes_from_bits_truncate 0x100#u32
        variableattributes_contains t 0x100#u32) = ok false ∧
    variableattributes_contains 0x100#u32 0x100#u32 = ok true := by
  refine ⟨fun w b hb => ?_, ?_, rfl⟩
  · have hw : (w &&& 255#u32).val = w.val % 2 ^ 8 := Bits.land_low_mask w 255#u32 8 rfl
    have hb8 : ∀ i, b.val.testBit i = true → i < 8 := by
      intro i hi
      have hmod : b.val = b.val % 2 ^ 8 := (Nat.mod_eq_of_lt hb).symm
      rw [hmod, Nat.testBit_mod_two_pow] at hi
      simp only [Bool.and_eq_true, decide_eq_true_eq] at hi
      exact hi.1
    unfold variableattributes_from_bits_truncate ofBitsTruncated variableattributes_contains holds
    simp only [Std.lift, bind_tc_ok, ok.injEq, decide_eq_decide]
    rw [carrying_a_mask_is_carrying_its_bits, carrying_a_mask_is_carrying_its_bits, hw]
    simp only [Nat.testBit_mod_two_pow, Bool.and_eq_true, decide_eq_true_eq]
    constructor
    · intro h i hi
      exact (h i hi).2
    · intro h i hi
      exact ⟨hb8 i hi, h i hi⟩
  · unfold variableattributes_from_bits_truncate ofBitsTruncated
    simp only [Std.lift, bind_tc_ok]
    rfl

/-- Truncation changes none of the three named predicates, for every word. -/
theorem truncation_changes_no_named_predicate (w : Std.U32) :
    (do let t ← variableattributes_from_bits_truncate w; variableattributes_is_non_volatile t) =
      variableattributes_is_non_volatile w ∧
    (do let t ← variableattributes_from_bits_truncate w; variableattributes_is_runtime_access t) =
      variableattributes_is_runtime_access w ∧
    (do let t ← variableattributes_from_bits_truncate w;
        variableattributes_requires_authentication t) =
      variableattributes_requires_authentication w := by
  have hw : (w &&& 255#u32).val = w.val % 2 ^ 8 := Bits.land_low_mask w 255#u32 8 rfl
  unfold variableattributes_from_bits_truncate ofBitsTruncated
  simp only [Std.lift, bind_tc_ok, non_volatility_is_bit_zero_of_the_word,
    runtime_access_is_bit_two_of_the_word, authentication_is_bit_five_or_bit_seven, hw,
    Nat.testBit_mod_two_pow]
  simp

/-- A single-flag predicate is disjunctive over union and conjunctive over
    intersection. A test of two bits at once would fail the first: bit 0 from
    one side and bit 1 from the other would pass after a union and fail on
    each side alone. -/
theorem non_volatility_and_runtime_access_follow_union_and_intersection (a b : Std.U32) :
    (do let u ← variableattributes_union a b; variableattributes_is_non_volatile u) =
      (do let x ← variableattributes_is_non_volatile a
          let y ← variableattributes_is_non_volatile b
          ok (x || y)) ∧
    (do let m ← variableattributes_intersection a b; variableattributes_is_non_volatile m) =
      (do let x ← variableattributes_is_non_volatile a
          let y ← variableattributes_is_non_volatile b
          ok (x && y)) ∧
    (do let u ← variableattributes_union a b; variableattributes_is_runtime_access u) =
      (do let x ← variableattributes_is_runtime_access a
          let y ← variableattributes_is_runtime_access b
          ok (x || y)) ∧
    (do let m ← variableattributes_intersection a b; variableattributes_is_runtime_access m) =
      (do let x ← variableattributes_is_runtime_access a
          let y ← variableattributes_is_runtime_access b
          ok (x && y)) := by
  unfold variableattributes_union variableattributes_intersection join meet
  simp only [Std.lift, bind_tc_ok, non_volatility_is_bit_zero_of_the_word,
    runtime_access_is_bit_two_of_the_word, UScalar.val_or, UScalar.val_and, Nat.testBit_or,
    Nat.testBit_and]
  exact ⟨trivial, trivial, trivial, trivial⟩

/-- Adding flags never removes an authentication requirement: a union needs
    authentication exactly when either side does. Intersection is not the dual,
    and the witness shows it: a set needing time-based authentication and one
    needing enhanced authentication intersect in a set that needs neither, so a
    caller that narrows two authenticated sets cannot assume the result is
    still authenticated. -/
theorem authentication_survives_union_but_not_intersection (a b : Std.U32) :
    (do let u ← variableattributes_union a b; variableattributes_requires_authentication u) =
      (do let x ← variableattributes_requires_authentication a
          let y ← variableattributes_requires_authentication b
          ok (x || y)) ∧
    (variableattributes_requires_authentication 0x20#u32 = ok true ∧
      variableattributes_requires_authentication 0x80#u32 = ok true ∧
      (do let m ← variableattributes_intersection 0x20#u32 0x80#u32
          variableattributes_requires_authentication m) = ok false) := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · unfold variableattributes_union join
    simp only [Std.lift, bind_tc_ok, authentication_is_bit_five_or_bit_seven, UScalar.val_or,
      Nat.testBit_or, ok.injEq]
    cases a.val.testBit 5 <;> cases b.val.testBit 5 <;> cases a.val.testBit 7 <;>
      cases b.val.testBit 7 <;> rfl
  all_goals
    try unfold variableattributes_intersection meet
    unfold variableattributes_requires_authentication needsAuth holds
      attributes.VariableAttributes.TIME_BASED_AUTHENTICATED_WRITE_ACCESS
      attributes.VariableAttributes.ENHANCED_AUTHENTICATED_ACCESS
    rfl

/-- Records what the manager's cache answers. `UefiManager::get_variable` and
    the secure boot prefill in `manager/init.rs` read a variable's attributes
    from `GetVariable` into a local, drop them, and cache the variable with
    `DEFAULT_NV_BS_RT`, which the kernel defines as
    `NON_VOLATILE | BOOTSERVICE_ACCESS | RUNTIME_ACCESS`. It is built here from
    the extracted constants and `union`, with `BOOTSERVICE_ACCESS` written as
    its value `2`. On that word the set is non-volatile, runtime accessible and
    needs no authentication. The UEFI specification gives `SecureBoot` and
    `SetupMode` boot service and runtime access only, `0x06`, which is
    volatile, and gives `PK`, `KEK`, `db` and `dbx` those plus non-volatile and
    time-based authenticated write, `0x27`, which needs authentication. So
    `UefiVariable::is_non_volatile` on a cached `SecureBoot` answers `true`
    where the word firmware reported answers `false`, and a check of
    `requires_authentication` on a cached `PK` would answer `false` where
    firmware's word answers `true`. -/
theorem the_cached_attribute_word_contradicts_the_firmware_word :
    (do let a ← variableattributes_union attributes.VariableAttributes.NON_VOLATILE 2#u32
        let a ← variableattributes_union a attributes.VariableAttributes.RUNTIME_ACCESS
        let nv ← variableattributes_is_non_volatile a
        let rt ← variableattributes_is_runtime_access a
        let auth ← variableattributes_requires_authentication a
        ok (nv, rt, auth)) = ok (true, true, false) ∧
    variableattributes_is_non_volatile 0x06#u32 = ok false ∧
    variableattributes_requires_authentication 0x27#u32 = ok true := by
  unfold variableattributes_union join variableattributes_is_non_volatile isNonVolatile
    variableattributes_is_runtime_access isRuntimeAccess
    variableattributes_requires_authentication needsAuth holds
    attributes.VariableAttributes.NON_VOLATILE attributes.VariableAttributes.RUNTIME_ACCESS
    attributes.VariableAttributes.TIME_BASED_AUTHENTICATED_WRITE_ACCESS
    attributes.VariableAttributes.ENHANCED_AUTHENTICATED_ACCESS
  exact ⟨rfl, rfl, rfl⟩

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
#print axioms NonosExtraction.Uefi.words_are_equal_when_their_bits_are
#print axioms NonosExtraction.Uefi.carrying_a_mask_is_carrying_its_bits
#print axioms NonosExtraction.Uefi.holding_a_set_is_holding_each_of_its_bits
#print axioms NonosExtraction.Uefi.containment_is_the_order_union_defines
#print axioms NonosExtraction.Uefi.a_set_contains_a_union_exactly_when_it_contains_both
#print axioms NonosExtraction.Uefi.the_intersection_contains_exactly_what_both_contain
#print axioms NonosExtraction.Uefi.non_volatility_is_bit_zero_of_the_word
#print axioms NonosExtraction.Uefi.runtime_access_is_bit_two_of_the_word
#print axioms NonosExtraction.Uefi.authentication_is_bit_five_or_bit_seven
#print axioms NonosExtraction.Uefi.the_counter_based_authenticated_write_flag_is_not_counted
#print axioms NonosExtraction.Uefi.a_set_is_empty_exactly_when_every_set_contains_it
#print axioms NonosExtraction.Uefi.the_empty_set_is_the_identity_for_union_and_holds_no_flag
#print axioms NonosExtraction.Uefi.an_empty_set_reaches_firmware_as_the_zero_word
#print axioms NonosExtraction.Uefi.a_truncated_word_reaches_firmware_as_its_low_byte
#print axioms NonosExtraction.Uefi.the_two_constructors_agree_exactly_on_words_below_256
#print axioms NonosExtraction.Uefi.truncation_changes_no_test_of_a_defined_flag
#print axioms NonosExtraction.Uefi.truncation_changes_no_named_predicate
#print axioms NonosExtraction.Uefi.non_volatility_and_runtime_access_follow_union_and_intersection
#print axioms NonosExtraction.Uefi.authentication_survives_union_but_not_intersection
#print axioms NonosExtraction.Uefi.the_cached_attribute_word_contradicts_the_firmware_word

end NonosExtraction.Uefi
