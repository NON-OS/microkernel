/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Getting an interrupt from a pin to a processor.

Three things went wrong on real hardware and each of them is a different link in
the same chain.

The firmware may report that an ISA interrupt arrives on a different global
number than its own, and the override table is how it says so. Reading the pin
number as the global number works on every machine that publishes no overrides,
which includes every machine anyone tested on. `override_wins` and
`identity_without_an_override` are the two halves, and
`a_collapsing_translation_merges_two_devices` is what a translation that is not
injective costs: two devices on one entry, and a handler that runs for the wrong
one.

The destination was written as zero. Zero is a real APIC identifier, the boot
processor's, so nothing failed: every interrupt arrived, at one processor, while
the others idled. `a_fixed_destination_starves_the_others` says it, and it is the
kind of defect that looks like a performance characteristic.

And an entry has to be masked before it is rewritten. The vector and the
destination are separate fields, so an interrupt that arrives between the two
writes is delivered against half an entry. `unmasked_rewrite_has_a_bad_window`
exhibits the intermediate state; `masked_rewrite_has_none` is the fix, and it is
a fix about ordering rather than about values.
-/

namespace Nonos.GsiRouting

/-! ### Translation -/

/-- An entry in the firmware's override table: this ISA interrupt actually
    arrives on that global number. -/
structure Override where
  isa : Nat
  gsi : Nat
  deriving DecidableEq, Repr

/-- Translate an ISA interrupt to a global number. Without an override the two
    agree, which is why reading one as the other is right until it is not. -/
def gsiOf (overrides : List Override) (isa : Nat) : Nat :=
  match overrides with
  | [] => isa
  | o :: rest => if o.isa = isa then o.gsi else gsiOf rest isa

/-- With no overrides the translation is the identity. -/
theorem identity_without_an_override (isa : Nat) : gsiOf [] isa = isa := rfl

/-- An override for this interrupt decides it. -/
theorem override_wins (o : Override) (rest : List Override) :
    gsiOf (o :: rest) o.isa = o.gsi := by
  unfold gsiOf
  simp

/-- An override for a different interrupt does not. -/
theorem other_overrides_are_skipped (o : Override) (rest : List Override) (isa : Nat)
    (h : o.isa ≠ isa) : gsiOf (o :: rest) isa = gsiOf rest isa := by
  show (if o.isa = isa then o.gsi else gsiOf rest isa) = gsiOf rest isa
  simp [h]

/-- The translation is total: every ISA interrupt gets a global number, so there
    is no interrupt the routing silently drops. -/
theorem translation_is_total (overrides : List Override) (isa : Nat) :
    ∃ g, gsiOf overrides isa = g := ⟨_, rfl⟩

/-- Two ISA interrupts translated to one global number share an entry. Whichever
    handler is installed runs for both devices, and one of them is not the one it
    was written for. -/
def Collapses (overrides : List Override) (a b : Nat) : Prop :=
  a ≠ b ∧ gsiOf overrides a = gsiOf overrides b

/-- A table that maps two interrupts onto one number collapses them, and the
    witness is the smallest table that does it. -/
theorem a_collapsing_translation_merges_two_devices :
    Collapses [⟨0, 2⟩, ⟨1, 2⟩] 0 1 := by
  constructor
  · omega
  · unfold gsiOf
    rfl

/-- The identity translation collapses nothing, so the defect is in the override
    table rather than in the idea of overriding. -/
theorem identity_collapses_nothing (a b : Nat) : ¬ Collapses [] a b := by
  intro ⟨hne, heq⟩
  unfold gsiOf at heq
  exact hne heq

/-! ### Vectors -/

/-- The processor reserves the first thirty-two vectors for its own exceptions. -/
def firstUsableVector : Nat := 32

/-- And there are two hundred and fifty-six in total. -/
def vectorCount : Nat := 256

/-- Where a global interrupt number lands in the vector space. -/
def vectorOf (gsi : Nat) : Nat := firstUsableVector + gsi

/-- No interrupt is ever routed onto an exception vector, so a device cannot be
    made to look like a page fault. -/
theorem vector_avoids_the_exception_range (gsi : Nat) :
    firstUsableVector ≤ vectorOf gsi := by
  unfold vectorOf
  omega

/-- And the vector fits, provided the global number does. -/
theorem vector_fits (gsi : Nat) (h : gsi < vectorCount - firstUsableVector) :
    vectorOf gsi < vectorCount := by
  unfold vectorOf firstUsableVector vectorCount
  unfold firstUsableVector vectorCount at h
  omega

/-- Distinct global numbers get distinct vectors, so the vector identifies the
    source. A collapsing translation defeats this before it is reached, which is
    why the two are separate properties. -/
theorem vectors_are_distinct (a b : Nat) (h : vectorOf a = vectorOf b) : a = b := by
  unfold vectorOf at h
  omega

/-! ### Destination -/

/-- An entry as programmed: which vector, which processor, and whether it is
    masked. -/
structure Entry where
  vector : Nat
  destination : Nat
  masked : Bool
  deriving DecidableEq, Repr

/-- Whether a processor receives this entry's interrupts. -/
def deliversTo (e : Entry) (apicId : Nat) : Bool :=
  !e.masked && e.destination == apicId

/-- With the destination written as a constant, only the processor holding that
    identifier is ever interrupted. -/
def fixedDestination (vector : Nat) : Entry := ⟨vector, 0, false⟩

/-- The boot processor receives everything. -/
theorem the_boot_processor_receives_everything (v : Nat) :
    deliversTo (fixedDestination v) 0 = true := rfl

/-- And no other processor receives anything, for any vector. Nothing is lost, so
    nothing looks broken; the work simply does not spread. -/
theorem a_fixed_destination_starves_the_others (v apicId : Nat) (h : apicId ≠ 0) :
    deliversTo (fixedDestination v) apicId = false := by
  unfold deliversTo fixedDestination
  simp
  omega

/-- A destination chosen per device can reach any processor, which is the property
    the constant was standing in for. -/
theorem a_chosen_destination_reaches_any_processor (v apicId : Nat) :
    deliversTo ⟨v, apicId, false⟩ apicId = true := by
  unfold deliversTo
  simp

/-- A masked entry delivers to nobody, whatever its destination says. -/
theorem masked_delivers_to_nobody (v dest apicId : Nat) :
    deliversTo ⟨v, dest, true⟩ apicId = false := rfl

/-! ### Rewriting an entry -/

/-- Programming an entry is two writes, because the vector and the destination are
    separate fields. -/
def writeVector (e : Entry) (v : Nat) : Entry := { e with vector := v }
def writeDestination (e : Entry) (d : Nat) : Entry := { e with destination := d }

/-- The state between the two writes. -/
def midRewrite (e : Entry) (v : Nat) : Entry := writeVector e v

/-- An entry left unmasked has a window in which it carries the new vector and the
    old destination. An interrupt arriving in that window is delivered to the
    processor that was supposed to stop receiving it, against a handler that was
    installed for a different device. -/
theorem unmasked_rewrite_has_a_bad_window (v v' d d' : Nat)
    (hv : v ≠ v') (hd : d ≠ d') :
    deliversTo (midRewrite ⟨v, d, false⟩ v') d = true ∧
      (midRewrite ⟨v, d, false⟩ v').vector = v' := by
  constructor
  · unfold deliversTo midRewrite writeVector
    simp
  · rfl

/-- Masked first, the window delivers nothing, so there is no state in which a
    half-written entry is live. The fix is that the mask is written before the
    fields and cleared after, not that the fields are written in a better
    order. -/
theorem masked_rewrite_has_no_window (v v' d : Nat) :
    deliversTo (midRewrite ⟨v, d, true⟩ v') d = false := rfl

/-- And the entry that comes out of a masked rewrite is the one intended, once the
    mask is cleared. Masking costs nothing but the ordering. -/
theorem masked_rewrite_ends_correct (v v' d d' : Nat) :
    { (writeDestination (writeVector ⟨v, d, true⟩ v') d') with masked := false } =
      ⟨v', d', false⟩ := rfl

/-! ### The chain, end to end -/

/-- What an interrupt's route is: translate, choose a vector, pick a processor. -/
def route (overrides : List Override) (isa dest : Nat) : Entry :=
  ⟨vectorOf (gsiOf overrides isa), dest, false⟩

/-- A routed interrupt reaches the processor it was routed to, and lands above the
    exception range. -/
theorem a_routed_interrupt_arrives (overrides : List Override) (isa dest : Nat) :
    deliversTo (route overrides isa dest) dest = true ∧
      firstUsableVector ≤ (route overrides isa dest).vector := by
  constructor
  · unfold deliversTo route
    simp
  · unfold route
    exact vector_avoids_the_exception_range _

/-- Two interrupts that the override table does not collapse get different
    entries, so their handlers do not run for each other. -/
theorem uncollapsed_interrupts_get_different_vectors (overrides : List Override)
    (a b dest : Nat) (h : ¬ Collapses overrides a b) (hne : a ≠ b) :
    (route overrides a dest).vector ≠ (route overrides b dest).vector := by
  unfold route
  simp only
  intro hx
  have := vectors_are_distinct _ _ hx
  exact h ⟨hne, this⟩

end Nonos.GsiRouting
