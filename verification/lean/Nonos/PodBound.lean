/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Which types may cross the user boundary as raw bytes.

`read_user_value` and `write_user_value` are generic over `T: Copy`, and `Copy`
is the wrong bound. It says a value may be duplicated by copying its bytes
between two places the compiler knows about. It does not say that every byte
pattern of that size is a valid `T`, and it does not say that a `T` has no
padding. Both are required here, because one side of this copy is memory a guest
controls and the other is memory the kernel does.

Reading is the soundness half. A type with invalid bit patterns, a `bool`, an
enumeration, a `NonZeroU32`, a reference, gets one constructed out of guest bytes,
and from then on the compiler is entitled to assume something that is not true.
`invalid_pattern_is_reachable` is that, at the smallest type that has the
property.

Writing is the disclosure half. Padding bytes are not part of a value, so what
goes into them is whatever was in that stack slot, and the copy publishes it.
`padding_is_not_determined_by_the_value` says the written bytes are not a
function of the value alone, which is the definition of a leak, and
`plain_layout_writes_only_the_value` says what it takes to close it.

The bound the two together ask for is a marker trait asserting both, applied at
every call site. There are forty-seven, so the trait is the only way to get them
all; a review cannot.
-/

namespace Nonos.PodBound

/-! ### Layouts -/

/-- What matters about a type at this boundary: how many bytes it occupies, which
    of those bytes carry the value, and which patterns are values of it. -/
structure Layout where
  size : Nat
  /-- The offsets that carry the value. An offset below `size` and absent from
      this list is padding. -/
  dataOffsets : List Nat
  /-- Whether a byte pattern is a value of the type. -/
  valid : List Nat → Bool

/-- Every byte carries value: the type has no padding. -/
def NoPadding (l : Layout) : Prop := ∀ i, i < l.size → i ∈ l.dataOffsets

/-- Every pattern of the right length is a value: the type has no invalid bit
    patterns. -/
def Inhabits (l : Layout) : Prop := ∀ bs, bs.length = l.size → l.valid bs = true

/-- The bound the copy actually needs, which `Copy` does not imply either half
    of. -/
def Plain (l : Layout) : Prop := NoPadding l ∧ Inhabits l

/-! ### Reading from the guest -/

/-- A read reinterprets the bytes it copied. It performs no check, which is the
    whole of its cost. -/
def readAccepts (_l : Layout) (_bs : List Nat) : Bool := true

/-- Whether the value the read produced is one the type can hold. -/
def Sound (l : Layout) (bs : List Nat) : Prop := l.valid bs = true

/-- `bool`: one byte, no padding, and exactly two of its two hundred and
    fifty-six patterns are values. -/
def boolLayout : Layout where
  size := 1
  dataOffsets := [0]
  valid := fun bs => bs == [0] || bs == [1]

theorem bool_has_no_padding : NoPadding boolLayout := by
  intro i hi
  unfold boolLayout at hi
  simp only at hi
  have : i = 0 := by omega
  subst this
  unfold boolLayout
  simp

/-- And is not inhabited by every pattern, so it fails the bound. -/
theorem bool_is_not_inhabited : ¬ Inhabits boolLayout := by
  intro h
  have := h [2] (by unfold boolLayout; rfl)
  unfold boolLayout at this
  simp at this

/-- A read under the `Copy` bound accepts a pattern that is not a value of the
    type, so the value it hands back is one the compiler has already assumed
    cannot exist. -/
theorem invalid_pattern_is_reachable :
    ∃ bs, readAccepts boolLayout bs = true ∧ ¬ Sound boolLayout bs := by
  refine ⟨[2], rfl, ?_⟩
  intro h
  unfold Sound boolLayout at h
  simp at h

/-- Under the right bound every accepted read is sound, for every pattern the
    guest can supply. This is what the marker trait buys, and it buys it without
    a runtime check. -/
theorem plain_read_is_always_sound (l : Layout) (h : Plain l) (bs : List Nat)
    (hlen : bs.length = l.size) : Sound l bs :=
  h.2 bs hlen

/-- And the check the read is not doing is exactly `valid`, so a type that fails
    the bound needs one. A read of a non-plain type is either checked or
    unsound; there is no third option. -/
theorem non_plain_read_needs_a_check (l : Layout) (h : ¬ Inhabits l) :
    ∃ bs, bs.length = l.size ∧ ¬ Sound l bs := by
  unfold Inhabits at h
  by_cases hex : ∃ bs, bs.length = l.size ∧ l.valid bs ≠ true
  · obtain ⟨bs, hlen, hv⟩ := hex
    exact ⟨bs, hlen, hv⟩
  · exfalso
    apply h
    intro bs hlen
    by_cases hv : l.valid bs = true
    · exact hv
    · exact absurd ⟨bs, hlen, hv⟩ hex

/-! ### Writing to the guest -/

/-- One byte of what a write puts on the wire. Value bytes come from the value;
    everything else comes from whatever was in that memory. -/
def writeByte (l : Layout) (value pad : Nat → Nat) (i : Nat) : Nat :=
  if i ∈ l.dataOffsets then value i else pad i

/-- With no padding, the bytes written are a function of the value alone: two
    writes of the same value from different stack slots publish the same
    bytes. -/
theorem plain_layout_writes_only_the_value (l : Layout) (h : NoPadding l)
    (value pad pad' : Nat → Nat) :
    ∀ i, i < l.size → writeByte l value pad i = writeByte l value pad' i := by
  intro i hi
  unfold writeByte
  rw [if_pos (h i hi)]
  rw [if_pos (h i hi)]

/-- With padding, they are not. The same value written twice publishes different
    bytes, and the difference is whatever the kernel last left there. -/
theorem padding_is_not_determined_by_the_value (l : Layout) (i : Nat)
    (hi : i < l.size) (hpad : i ∉ l.dataOffsets) :
    ∃ pad pad', writeByte l (fun _ => 0) pad i ≠ writeByte l (fun _ => 0) pad' i := by
  refine ⟨fun _ => 0, fun _ => 1, ?_⟩
  simp [writeByte, hpad]

/-- A layout with padding fails the bound, so the two halves of `Plain` are not
    independent claims about unrelated risks: the same trait rules out both the
    invalid read and the padding write. -/
theorem padding_fails_the_bound (l : Layout) (i : Nat) (hi : i < l.size)
    (hpad : i ∉ l.dataOffsets) : ¬ Plain l := by
  intro ⟨hnp, _⟩
  exact hpad (hnp i hi)

/-- A layout with a three-byte value in a four-byte slot: the shape of every
    structure whose fields do not fill their alignment. -/
def paddedLayout : Layout where
  size := 4
  dataOffsets := [0, 1, 2]
  valid := fun _ => true

theorem padded_layout_leaks :
    ∃ pad pad', writeByte paddedLayout (fun _ => 0) pad 3 ≠
                writeByte paddedLayout (fun _ => 0) pad' 3 := by
  refine padding_is_not_determined_by_the_value paddedLayout 3 ?_ ?_
  · show (3 : Nat) < 4
    omega
  · show (3 : Nat) ∉ [0, 1, 2]
    decide

/-- Its bytes are valid on the read side, so nothing about a read of it looks
    wrong. A type can pass one half of the bound and fail the other, which is
    why the trait has to assert both. -/
theorem padded_layout_reads_soundly : Inhabits paddedLayout := by
  intro bs _
  unfold paddedLayout
  rfl

theorem padded_layout_is_not_plain : ¬ Plain paddedLayout := by
  intro ⟨hnp, _⟩
  have := hnp 3 (by show (3 : Nat) < 4; omega)
  show False
  revert this
  show (3 : Nat) ∈ [0, 1, 2] → False
  decide

/-! ### The call sites -/

/-- The number of places the generic copy is instantiated. A bound enforced by
    the type system holds at all of them; a review holds at the ones it read. -/
def callSites : Nat := 47

/-- The bound, applied at a site. -/
def SiteIsSafe (l : Layout) : Prop := Plain l

/-- A trait bound discharges every site at once: if the instantiated type carries
    the bound, every site is safe, whatever the site does. -/
theorem the_bound_discharges_every_site (l : Layout) (h : Plain l) :
    SiteIsSafe l := h

/-- And one site instantiated at a type without the bound is one unsound copy,
    whatever the other forty-six do. -/
theorem one_bad_site_is_enough (l : Layout) (h : ¬ Plain l) : ¬ SiteIsSafe l := h

end Nonos.PodBound
