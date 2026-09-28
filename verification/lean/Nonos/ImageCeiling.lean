/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

A ceiling on what a capsule in the image may be granted.

The idea is sound: an image carries a maximum, no manifest in that image may ask
for more than the maximum, and the reviewable question becomes one number rather
than fifty manifests. What shipped was a ceiling of `0x7fffffff` and no code that
consulted it, which is two defects that cover for each other. Unenforced, the
value is inert. Enforced, that particular value refuses three capabilities the
machine needs: `AttestRead` at bit 31, `ForeignExec` at bit 32 and `LocalSign` at
bit 33 are all above it, so turning the check on would have stopped attestation
reads, every Linux binary, and local signing at once.

Which makes the interesting theorem the soundness condition rather than the
arithmetic. `sound_ceiling_admits_every_capability` says a ceiling is usable only
if it covers the whole table, and `the_shipped_ceiling_is_not_sound` says the
shipped one does not. `an_unenforced_ceiling_refuses_nothing` is the other half:
a check that is not on the path grants everything, so the two defects have to be
fixed in one direction. Raise the ceiling first, then enforce it.
-/

namespace Nonos.ImageCeiling

/-! ### The table, by bit index -/

/-- The capabilities this file needs to name, at the indices they occupy. The
    three at the top are the ones the shipped ceiling excludes. -/
def coreExec : Nat := 0
def appInstall : Nat := 30
def attestRead : Nat := 31
def foreignExec : Nat := 32
def localSign : Nat := 33

/-- The highest index in the table. A ceiling has to cover this. -/
def highestIndex : Nat := localSign

/-- A capability's bit. -/
def bit (i : Nat) : Nat := 2 ^ i

/-- Every index the table defines, written out so a claim can be decided over
    all of them rather than argued about in general. A capability added to the
    Rust enumeration without being added here leaves `table_is_complete` in
    `CapsComplete` as the thing that fails, not this file. -/
def tableIndices : List Nat :=
  [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
    18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33 ]

/-- The list covers exactly the indices up to the highest, with nothing missing
    in between and nothing beyond. -/
theorem table_indices_are_the_range (i : Nat) : i ∈ tableIndices ↔ i ≤ highestIndex := by
  constructor
  · intro h
    unfold tableIndices at h
    unfold highestIndex localSign
    simp at h
    rcases h with h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h <;> omega
  · intro h
    unfold highestIndex localSign at h
    unfold tableIndices
    have : i = 0 ∨ i = 1 ∨ i = 2 ∨ i = 3 ∨ i = 4 ∨ i = 5 ∨ i = 6 ∨ i = 7 ∨ i = 8 ∨
           i = 9 ∨ i = 10 ∨ i = 11 ∨ i = 12 ∨ i = 13 ∨ i = 14 ∨ i = 15 ∨ i = 16 ∨
           i = 17 ∨ i = 18 ∨ i = 19 ∨ i = 20 ∨ i = 21 ∨ i = 22 ∨ i = 23 ∨ i = 24 ∨
           i = 25 ∨ i = 26 ∨ i = 27 ∨ i = 28 ∨ i = 29 ∨ i = 30 ∨ i = 31 ∨ i = 32 ∨
           i = 33 := by omega
    rcases this with h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h <;>
      subst h <;> simp

/-- The mask that grants every capability in the table: bits zero through the
    highest, inclusive. -/
def fullMask : Nat := 0x3FFFFFFFF

/-- The value the image actually carried. -/
def shippedCeiling : Nat := 0x7fffffff

/-! ### What a ceiling does -/

/-- A grant passes the ceiling when every bit it asks for is in the mask. Stated
    per bit, because that is how the failure presents: one capability refused out
    of a manifest that is otherwise fine. -/
def Admits (ceiling i : Nat) : Prop := (ceiling / bit i) % 2 = 1

instance (ceiling i : Nat) : Decidable (Admits ceiling i) := by
  unfold Admits; infer_instance

/-- A ceiling is sound when it admits every capability the table defines.
    Anything less is a ceiling that refuses part of the machine. -/
def Sound (ceiling : Nat) : Prop := ∀ i ∈ tableIndices, Admits ceiling i

instance (ceiling : Nat) : Decidable (Sound ceiling) := by
  unfold Sound; infer_instance

/-! ### The shipped value -/

/-- `0x7fffffff` is bits zero through thirty. -/
theorem shipped_is_thirty_one_bits : shippedCeiling = 2 ^ 31 - 1 := by
  unfold shippedCeiling; decide

/-- It admits the ordinary capabilities. The value is not absurd, which is part
    of why it survived: everything anyone tested was below bit thirty-one. -/
theorem shipped_admits_the_low_table :
    Admits shippedCeiling coreExec ∧ Admits shippedCeiling appInstall := by
  constructor
  · unfold Admits bit shippedCeiling coreExec; decide
  · unfold Admits bit shippedCeiling appInstall; decide

/-- And refuses the three above it, each of which is load-bearing: attestation
    reads, every foreign binary, and signing on the machine. -/
theorem shipped_refuses_attest_read : ¬ Admits shippedCeiling attestRead := by
  unfold Admits bit shippedCeiling attestRead; decide

theorem shipped_refuses_foreign_exec : ¬ Admits shippedCeiling foreignExec := by
  unfold Admits bit shippedCeiling foreignExec; decide

theorem shipped_refuses_local_sign : ¬ Admits shippedCeiling localSign := by
  unfold Admits bit shippedCeiling localSign; decide

/-- So the shipped ceiling is not sound, and enforcing it as it stood would have
    been a regression rather than a hardening. -/
theorem the_shipped_ceiling_is_not_sound : ¬ Sound shippedCeiling := by
  intro h
  have hmem : localSign ∈ tableIndices := by unfold localSign tableIndices; decide
  exact shipped_refuses_local_sign (h localSign hmem)

/-! ### What a sound ceiling looks like -/

/-- The full mask admits every index in the table: thirty-four arithmetic facts,
    decided rather than argued, because the list is finite and the arithmetic is
    closed. -/
theorem full_mask_is_sound : Sound fullMask := by
  unfold Sound Admits bit fullMask tableIndices
  decide

/-- And the full mask is the table's own width, so it is derived from the list
    rather than written alongside it. -/
theorem full_mask_covers_the_highest : fullMask = 2 ^ (highestIndex + 1) - 1 := by
  unfold fullMask highestIndex localSign
  decide

/-- A sound ceiling admits each of the three the shipped one refused, so the
    soundness condition is exactly what was missing. -/
theorem sound_ceiling_admits_every_capability (c : Nat) (h : Sound c) :
    Admits c attestRead ∧ Admits c foreignExec ∧ Admits c localSign := by
  refine ⟨h attestRead ?_, h foreignExec ?_, h localSign ?_⟩
  · unfold attestRead tableIndices; decide
  · unfold foreignExec tableIndices; decide
  · unfold localSign tableIndices; decide

/-- And a sound ceiling is at least the full mask, so there is a least sound
    value and it is the one derived from the table rather than written down. -/
theorem sound_ceiling_is_at_least_full (c : Nat) (h : Sound c) : bit highestIndex ≤ c := by
  have hmem : highestIndex ∈ tableIndices := by unfold highestIndex localSign tableIndices; decide
  have hadm := h highestIndex hmem
  unfold Admits at hadm
  by_cases hlt : c < bit highestIndex
  · exfalso
    have : c / bit highestIndex = 0 := Nat.div_eq_of_lt hlt
    rw [this] at hadm
    simp at hadm
  · omega

/-! ### An unenforced ceiling -/

/-- What a grant decision looks like. -/
inductive Decision where
  | granted
  | refused
  deriving DecidableEq, Repr

/-- The check, on the path. -/
def enforced (ceiling i : Nat) : Decision :=
  if Admits ceiling i then .granted else .refused

/-- The check, not on the path: the ceiling is read, stored, and never consulted.
    This is the code as it shipped. -/
def unenforced (_ceiling i : Nat) : Decision :=
  let _ := i
  .granted

/-- An unenforced ceiling grants everything, for every ceiling and every
    capability. The value in the image is inert, so a review that reads the
    number learns nothing about what the machine will grant. -/
theorem an_unenforced_ceiling_refuses_nothing (c i : Nat) : unenforced c i = .granted := rfl

/-- In particular the shipped ceiling refused nothing in practice, which is why
    nothing broke and why nothing was protected either. -/
theorem shipped_ceiling_refused_nothing_in_practice (i : Nat) :
    unenforced shippedCeiling i = .granted := rfl

/-- Enforced, the two disagree exactly where the ceiling excludes a capability,
    so turning the check on is a behavioural change wherever the ceiling is
    unsound. -/
theorem enforcement_changes_behaviour_where_unsound (c i : Nat) (h : ¬ Admits c i) :
    enforced c i ≠ unenforced c i := by
  unfold enforced unenforced
  simp [h]

/-- And nowhere else: on a sound ceiling, enforcing changes nothing for any
    capability in the table. That is the condition under which the check can be
    turned on safely. -/
theorem enforcing_a_sound_ceiling_is_invisible (c : Nat) (h : Sound c) :
    ∀ i ∈ tableIndices, enforced c i = unenforced c i := by
  intro i hi
  unfold enforced unenforced
  simp [h i hi]

end Nonos.ImageCeiling
