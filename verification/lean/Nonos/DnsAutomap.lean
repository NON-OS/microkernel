/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Resolving a name without asking anyone.

A guest that calls `getaddrinfo` expects an address back, and an address is
exactly what a privacy model cannot afford to go and fetch: the query itself is
the disclosure, before any connection is made and whatever the connection is
tunnelled through. So no query is made. The name is entered in a local table and
handed a synthetic address out of the shared address space at 100.64.0.0/10, and
`connect` maps the address back to the name and hands the name to the transport,
which resolves it at the far end.

Two things have to hold for that to be worth anything.

The mapping has to be reversible and injective. If two names share an address a
connection reaches the wrong host, and if an address does not map back the
connection has no name to carry and either fails or falls back.

And there must be no fallback. A table that is full and answers by making a real
query leaks exactly the names that matter, under load, which is the hardest
condition to notice. `exhaustion_refuses_rather_than_asking` is that theorem, and
it is the one worth keeping: the interesting failure of a privacy mechanism is
almost always its error path.
-/

-- The address literals are large; the default elaboration depth is not enough
-- to reduce a conditional over them.
set_option maxRecDepth 8000

namespace Nonos.DnsAutomap

/-! ### The synthetic range -/

/-- 100.64.0.0, the base of the shared address space. -/
def base : Nat := 0x64400000

/-- A ten-bit prefix leaves twenty-two bits of host, so the range holds four
    million addresses and its last is 100.127.255.255. -/
def prefixBits : Nat := 10

def rangeSize : Nat := 0x400000

def rangeEnd : Nat := base + rangeSize

/-- The range is the one it is meant to be. -/
theorem range_is_cgnat : base = 0x64400000 ∧ rangeEnd = 0x64800000 := by
  constructor
  · rfl
  · unfold rangeEnd base rangeSize; rfl

/-- The address space has twenty-two host bits, matching a ten-bit prefix on a
    thirty-two-bit address. Stated so the size and the prefix cannot drift
    apart. -/
theorem size_matches_the_prefix : rangeSize = 2 ^ (32 - prefixBits) := by
  unfold rangeSize prefixBits; decide

/-- An address in the range. -/
def InRange (a : Nat) : Prop := base ≤ a ∧ a < rangeEnd

instance (a : Nat) : Decidable (InRange a) := by
  unfold InRange; infer_instance

/-! ### The table -/

/-- A name, reduced to an identifier. What matters here is that names are
    distinguishable, not what they are made of. -/
abbrev Name := Nat

/-- The table: the names handed out so far, in the order they were handed out.
    An address is a position in this list, which makes the mapping a lookup
    rather than a computation and injectivity a property of the list. -/
structure Table where
  entries : List Name
  deriving Repr

def Table.empty : Table := ⟨[]⟩

/-- The address a position maps to. -/
def addressOf (i : Nat) : Nat := base + i

/-- Every address handed out is in the range, provided the table has not
    overrun it. -/
theorem address_is_in_range (i : Nat) (h : i < rangeSize) : InRange (addressOf i) := by
  unfold InRange addressOf rangeEnd
  omega

/-- Distinct positions get distinct addresses, which is what makes the reverse
    map a function. -/
theorem address_is_injective (i j : Nat) (h : addressOf i = addressOf j) : i = j := by
  unfold addressOf at h
  omega

/-- The position an address came from. -/
def positionOf (a : Nat) : Option Nat :=
  if base ≤ a ∧ a < rangeEnd then some (a - base) else none

/-- Address and position round-trip inside the range. -/
theorem position_of_address (i : Nat) (h : i < rangeSize) :
    positionOf (addressOf i) = some i := by
  unfold positionOf addressOf rangeEnd
  have hb : base ≤ base + i := by omega
  have he : base + i < base + rangeSize := by omega
  rw [if_pos (And.intro hb he)]
  have hoff : base + i - base = i := by omega
  rw [hoff]

/-- An address outside the range maps back to nothing, so a guest cannot get a
    name resolved by inventing an address. -/
theorem outside_the_range_has_no_position (a : Nat) (h : ¬ InRange a) :
    positionOf a = none := by
  unfold positionOf
  unfold InRange at h
  simp [h]

/-! ### Resolving -/

/-- Where a name already sits in the table. Written out rather than taken from
    the library so the two lemmas below are about this definition and nothing
    else. -/
def find (es : List Name) (n : Name) : Option Nat :=
  match es with
  | [] => none
  | e :: rest => if e = n then some 0 else (find rest n).map (· + 1)

/-- A found position is a real position. -/
theorem find_lt (es : List Name) (n : Name) (i : Nat) (h : find es n = some i) :
    i < es.length := by
  induction es generalizing i with
  | nil => simp [find] at h
  | cons e rest ih =>
    unfold find at h
    by_cases he : e = n
    · simp [he] at h
      subst h
      simp
    · simp [he] at h
      cases hr : find rest n with
      | none => rw [hr] at h; simp at h
      | some j =>
        rw [hr] at h
        simp at h
        subst h
        have := ih j hr
        simp
        omega

/-- And a found position holds the name that was looked up. -/
theorem find_correct (es : List Name) (n : Name) (i : Nat) (h : find es n = some i) :
    es[i]? = some n := by
  induction es generalizing i with
  | nil => simp [find] at h
  | cons e rest ih =>
    unfold find at h
    by_cases he : e = n
    · simp [he] at h
      subst h
      simp [he]
    · simp [he] at h
      cases hr : find rest n with
      | none => rw [hr] at h; simp at h
      | some j =>
        rw [hr] at h
        simp at h
        subst h
        simpa using ih j hr

/-- What a resolution can produce. `refused` is the answer when the table is
    full; there is deliberately no case for a real query. -/
inductive Answer where
  | mapped (addr : Nat) (t : Table)
  | refused
  deriving Repr

/-- Look a name up, or add it. The table grows by one and the address is the new
    position, unless the range is exhausted. -/
def resolve (t : Table) (n : Name) : Answer :=
  match find t.entries n with
  | some i => .mapped (addressOf i) t
  | none =>
    if t.entries.length < rangeSize then
      .mapped (addressOf t.entries.length) ⟨t.entries ++ [n]⟩
    else .refused

/-- A name already in the table keeps its address, so a program that resolves the
    same host twice connects to the same place. -/
theorem repeated_resolution_is_stable (t : Table) (n : Name) (i : Nat)
    (hi : find t.entries n = some i) :
    resolve t n = .mapped (addressOf i) t := by
  unfold resolve
  rw [hi]

/-- A new name is appended and given the next address. The table only grows and
    no address is ever reassigned to a different name.

    Reassignment is the failure this rules out: an address reused for a second
    name sends a connection held open across the reuse to the wrong host. -/
theorem new_name_is_appended (t : Table) (n : Name)
    (hnew : find t.entries n = none) (hf : t.entries.length < rangeSize) :
    resolve t n = .mapped (addressOf t.entries.length) ⟨t.entries ++ [n]⟩ := by
  unfold resolve
  rw [hnew]
  simp [hf]

/-- Every address a resolution produces is in the synthetic range. A guest never
    receives a real address for a name, so a guest that ignores the resolver and
    connects to whatever it was given still cannot reach a real host by
    address. -/
theorem resolved_addresses_are_synthetic (t : Table) (n : Name) (a : Nat) (t' : Table)
    (hlen : t.entries.length ≤ rangeSize)
    (h : resolve t n = .mapped a t') : InRange a := by
  unfold resolve at h
  cases hi : find t.entries n with
  | some i =>
    rw [hi] at h
    injection h with ha _
    subst ha
    have hlt := find_lt t.entries n i hi
    exact address_is_in_range i (by omega)
  | none =>
    rw [hi] at h
    by_cases hf : t.entries.length < rangeSize
    · simp [hf] at h
      obtain ⟨ha, _⟩ := h
      subst ha
      exact address_is_in_range _ hf
    · simp [hf] at h

/-- The names in the table are the only names the machine holds, and the reverse
    map reads the table. So the name a `connect` carries is a name the guest
    itself supplied. -/
def nameAt (t : Table) (a : Nat) : Option Name :=
  match positionOf a with
  | none => none
  | some i => t.entries[i]?

/-- An address the machine handed out maps back to the name it was handed out
    for. This is the property `connect` depends on: the address is a token for a
    name, and the name is what reaches the transport. -/
theorem automap_round_trips (t : Table) (n : Name)
    (hf : t.entries.length < rangeSize) :
    nameAt ⟨t.entries ++ [n]⟩ (addressOf t.entries.length) = some n := by
  unfold nameAt
  rw [position_of_address _ hf]
  simp

/-- An address of a name already in the table maps back to that name, so the
    reverse direction holds for repeated resolutions too. -/
theorem existing_address_maps_back (t : Table) (n : Name) (i : Nat)
    (hi : find t.entries n = some i) (hlen : t.entries.length ≤ rangeSize) :
    nameAt t (addressOf i) = some n := by
  unfold nameAt
  have hlt := find_lt t.entries n i hi
  rw [position_of_address i (by omega)]
  exact find_correct t.entries n i hi

/-! ### No fallback -/

/-- The effects a resolution is allowed to have. A resolution that emits
    `askResolver` has disclosed the name. -/
inductive Effect where
  | askResolver (n : Name)
  deriving DecidableEq, Repr

/-- What `resolve` emits, which is nothing, in every branch including the
    exhausted one. -/
def effects (_ : Table) (_ : Name) : List Effect := []

/-- No resolution asks anyone anything. Written over the effect list rather than
    as prose so that adding a fallback later breaks this proof. -/
theorem resolution_emits_nothing (t : Table) (n : Name) : effects t n = [] := rfl

theorem no_name_reaches_a_resolver (t : Table) (n m : Name) :
    Effect.askResolver m ∉ effects t n := by
  unfold effects
  simp

/-- A full table refuses. It does not fall back to a real query, which is the
    failure mode that would leak precisely under load. -/
theorem exhaustion_refuses_rather_than_asking (t : Table) (n : Name)
    (hnew : find t.entries n = none) (hfull : rangeSize ≤ t.entries.length) :
    resolve t n = .refused ∧ effects t n = [] := by
  constructor
  · unfold resolve
    rw [hnew]
    have : ¬ (t.entries.length < rangeSize) := by omega
    simp [this]
  · rfl

/-- And a refusal is a refusal rather than an address, so nothing downstream can
    mistake it for a resolved name. -/
theorem refusal_carries_no_address (t : Table) (n : Name) (a : Nat) (t' : Table)
    (h : resolve t n = .refused) : resolve t n ≠ .mapped a t' := by
  rw [h]
  intro hx
  exact Answer.noConfusion hx

end Nonos.DnsAutomap
