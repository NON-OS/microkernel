/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Which address space is running, on a machine with more than one processor.

`PagingManager` kept the current address space in a field. On one processor a
field is the truth. On several it is the truth about whichever processor wrote it
last, and an invalidation that selects by it invalidates entries belonging to
another processor's address space while leaving its own behind.

The theorems say why a single field cannot be made to work rather than why the
one we had was wrong. `no_single_value_tracks_two_spaces` is the whole argument:
two processors running different spaces have no common current space, so any
single-valued reading of "the current space" is wrong on at least one of them.
Everything after that is the consequence on the invalidation path, and
`per_cpu_read_is_exact` is the shape that has none of it.

The last group is about PCID. With PCID off, loading CR3 discards the whole
translation cache, so a full reload is sound no matter what the field said: the
unsoundness lives specifically in the selective path, and turning PCID on is what
makes the selective path load-bearing. That is worth having written down before
anyone turns it on.
-/

namespace Nonos.PerCpuAsid

/-- A cached translation: the address space it was established under, and the
    page it translates. -/
structure Entry where
  asid : Nat
  page : Nat
  deriving DecidableEq

/-- The machine: what each processor is running, and what each processor has
    cached. A processor's cache is its own, which is the fact the single field
    was in conflict with. -/
structure Machine where
  running : Nat → Nat
  cached : Nat → List Entry

/-- An entry is stale on a processor when it belongs to an address space that
    processor is no longer running. -/
def Stale (m : Machine) (cpu : Nat) (e : Entry) : Prop := e.asid ≠ m.running cpu

/-! ### A single field cannot describe several processors -/

/-- Two processors running different address spaces have no common current
    address space.

    This is the defect in one line, and it does not depend on any detail of the
    invalidation path: a field holding one number is wrong on at least one of the
    two processors, whichever number it holds. -/
theorem no_single_value_tracks_two_spaces (m : Machine) (a b : Nat)
    (h : m.running a ≠ m.running b) :
    ∀ v, ¬ (v = m.running a ∧ v = m.running b) := by
  intro v ⟨ha, hb⟩
  exact h (ha ▸ hb ▸ rfl)

/-- Reading the address space out of per-processor storage is exact by
    construction: the answer is indexed by the processor asking. -/
def percpuRunning (m : Machine) (cpu : Nat) : Nat := m.running cpu

theorem per_cpu_read_is_exact (m : Machine) (cpu : Nat) :
    percpuRunning m cpu = m.running cpu := rfl

/-- A global field is exact only where it happens to agree, so its correctness
    is a property of the schedule rather than of the code. -/
theorem global_read_is_exact_only_by_luck (m : Machine) (field cpu : Nat)
    (h : field = m.running cpu) : field = percpuRunning m cpu := h

/-! ### What the invalidation does with it -/

/-- Drop every cached entry for one address space. -/
def flushAsid (es : List Entry) (asid : Nat) : List Entry :=
  es.filter (fun e => e.asid != asid)

/-- Nothing for the flushed space survives. -/
theorem flushed_space_is_gone (es : List Entry) (asid : Nat) (e : Entry)
    (h : e ∈ flushAsid es asid) : e.asid ≠ asid := by
  unfold flushAsid at h
  have := (List.mem_filter.mp h).2
  simp at this
  exact this

/-- And nothing else is touched, so a selective flush is not a full flush by
    another name. -/
theorem flush_keeps_other_spaces (es : List Entry) (asid : Nat) (e : Entry)
    (hmem : e ∈ es) (h : e.asid ≠ asid) : e ∈ flushAsid es asid := by
  unfold flushAsid
  refine List.mem_filter.mpr ⟨hmem, ?_⟩
  simp [h]

/-- A flush keyed on a value that is not this processor's address space leaves
    this processor's own stale entry in place.

    The entry is a translation for a page the address space no longer owns, and
    the processor will keep using it. -/
theorem wrong_key_leaves_the_stale_entry (m : Machine) (cpu field : Nat) (e : Entry)
    (hmem : e ∈ m.cached cpu) (_hstale : Stale m cpu e) (hne : e.asid ≠ field) :
    e ∈ flushAsid (m.cached cpu) field :=
  flush_keeps_other_spaces _ _ e hmem hne

/-- Keyed on this processor's own address space, no stale entry of that space
    survives. The correct key makes the flush do its job. -/
theorem right_key_clears_the_space (m : Machine) (cpu : Nat) (e : Entry)
    (h : e ∈ flushAsid (m.cached cpu) (m.running cpu)) : e.asid ≠ m.running cpu :=
  flushed_space_is_gone _ _ e h

/-- The two together: on a machine where two processors run different spaces,
    a single field is the wrong key for one of them, whichever value it holds.

    Two stale entries, one per processor, each belonging to the space the *other*
    processor is running. A field holding one value clears at most one of them,
    and the entry it leaves behind is a live translation into a space its
    processor has left. -/
theorem single_field_flush_is_unsound (m : Machine) (a b : Nat) (ea eb : Entry)
    (hmema : ea ∈ m.cached a) (hmemb : eb ∈ m.cached b)
    (hsa : Stale m a ea) (hsb : Stale m b eb)
    (hea : ea.asid = m.running b) (heb : eb.asid = m.running a)
    (hdiff : m.running a ≠ m.running b) :
    ∀ field, (ea ∈ flushAsid (m.cached a) field ∧ Stale m a ea) ∨
             (eb ∈ flushAsid (m.cached b) field ∧ Stale m b eb) := by
  intro field
  by_cases h : ea.asid = field
  · /-
     * The field names the first entry's space, which is the space the second
     * processor is running. So it is not the second processor's key, and the
     * second entry survives there.
     -/
    have hne : eb.asid ≠ field := by
      intro hx
      apply hdiff
      rw [← heb, hx, ← h, hea]
    exact Or.inr ⟨flush_keeps_other_spaces _ _ eb hmemb hne, hsb⟩
  · exact Or.inl ⟨flush_keeps_other_spaces _ _ ea hmema h, hsa⟩

/-! ### PCID, and why the reload path was sound anyway -/

/-- A CR3 reload with PCID disabled: the whole cache goes. -/
def reloadCr3 (_ : List Entry) : List Entry := []

/-- After a reload nothing is cached, so nothing stale is cached. The field's
    value does not enter this statement, which is why the reload path was not
    affected by the defect. -/
theorem reload_leaves_nothing (es : List Entry) (e : Entry) :
    e ∉ reloadCr3 es := by
  unfold reloadCr3
  simp

/-- And therefore no stale entry survives a reload, on any processor, whatever
    the manager thought was running. -/
theorem reload_is_sound_regardless (m : Machine) (cpu : Nat) (e : Entry) :
    ¬ (e ∈ reloadCr3 (m.cached cpu) ∧ Stale m cpu e) := by
  intro ⟨hm, _⟩
  exact reload_leaves_nothing _ e hm

/-- With PCID on, a reload no longer discards other spaces, so the selective
    path becomes the only thing keeping the cache honest and the key has to be
    right. Modelled as a reload that keeps every entry of another space. -/
def reloadCr3Pcid (es : List Entry) (asid : Nat) : List Entry :=
  es.filter (fun e => e.asid != asid)

/-- Under PCID a reload is exactly the selective flush, so it inherits the
    selective flush's dependence on the key: with PCID on, the address space
    read has to be the per-processor one or the reload leaves stale entries
    behind as well. -/
theorem pcid_reload_is_the_selective_flush (es : List Entry) (asid : Nat) :
    reloadCr3Pcid es asid = flushAsid es asid := rfl

end Nonos.PerCpuAsid
