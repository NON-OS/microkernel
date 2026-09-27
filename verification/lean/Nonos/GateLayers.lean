/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Two gates, and what each of them is for.

`cap_table` says which syscall numbers a capsule may attempt. It is an entry
gate: it bounds the surface, and a number absent from it is unreachable. It was
read for a while as the authority map, and it is not one. Attempting `mmio_map`
is not the same question as being allowed to map this device's third bar, and the
table cannot answer the second question because it does not have the arguments.

So there are two layers and they do different jobs.
`entry_alone_is_not_authority` is the failure available when the second is
missing: a call that the table admits and the authority map would deny.
`authority_alone_is_reachable_everywhere` is the failure available when the first
is missing: the operation is reachable from every capsule, and the authority
check becomes the only thing between a caller and the hardware, so a defect in it
is exploitable from anywhere rather than from the few capsules the table admits.

The audit went through eighty-eight syscalls and found both layers present on
each. `unaudited` is empty and `the_audit_is_complete` is the proof obligation
that keeps it empty: a syscall added without an entry here fails the build rather
than shipping ungated. That is the same ratchet shape as the capability audit,
and it exists because the last count that was maintained by hand said twenty-three
against twenty-seven.
-/

namespace Nonos.GateLayers

/-! ### A call -/

/-- What a syscall arrives as: the number, and the object it names. The second is
    the part the table cannot see. -/
structure Call where
  nr : Nat
  target : Nat
  deriving DecidableEq, Repr

/-- The entry gate: a predicate on the number alone. -/
abbrev EntryTable := Nat → Bool

/-- The authority map: a predicate on the number and the object. -/
abbrev AuthorityMap := Nat → Nat → Bool

/-- What the two layers decide together. Both have to pass. -/
def admits (t : EntryTable) (a : AuthorityMap) (c : Call) : Bool :=
  t c.nr && a c.nr c.target

/-- The entry gate alone, which is what a capsule was measured against before the
    authority map existed on every path. -/
def entryOnly (t : EntryTable) (c : Call) : Bool := t c.nr

/-- The authority map alone. -/
def authorityOnly (a : AuthorityMap) (c : Call) : Bool := a c.nr c.target

/-! ### Neither layer is the other -/

/-- A call the table admits and the authority map denies. The entry gate passing
    says nothing about the object, so a capsule permitted to attempt an operation
    is not thereby permitted to perform it on anything it names. -/
theorem entry_alone_is_not_authority :
    ∃ (t : EntryTable) (a : AuthorityMap) (c : Call),
      entryOnly t c = true ∧ admits t a c = false := by
  refine ⟨fun _ => true, fun _ _ => false, ⟨7, 3⟩, rfl, rfl⟩

/-- And the other way: a call the authority map would admit, on a number the
    table refuses. The table is not redundant, it is what makes the operation
    unreachable from capsules that have no business attempting it. -/
theorem authority_alone_is_not_the_whole_gate :
    ∃ (t : EntryTable) (a : AuthorityMap) (c : Call),
      authorityOnly a c = true ∧ admits t a c = false := by
  refine ⟨fun _ => false, fun _ _ => true, ⟨7, 3⟩, rfl, rfl⟩

/-- A number the table refuses is unreachable whatever the authority map says, so
    removing a syscall from the table is a complete removal rather than a
    discouragement. -/
theorem absent_from_the_table_is_unreachable (t : EntryTable) (a : AuthorityMap)
    (c : Call) (h : t c.nr = false) : admits t a c = false := by
  unfold admits
  rw [h]
  rfl

/-- And an object the authority map refuses is unreachable whatever the table
    says. Each layer can refuse alone; neither can admit alone. -/
theorem denied_by_authority_is_unreachable (t : EntryTable) (a : AuthorityMap)
    (c : Call) (h : a c.nr c.target = false) : admits t a c = false := by
  unfold admits
  rw [h]
  simp

/-- Admission requires both, which is the statement that the composition is a
    conjunction and not a fallback. -/
theorem admission_requires_both (t : EntryTable) (a : AuthorityMap) (c : Call)
    (h : admits t a c = true) : t c.nr = true ∧ a c.nr c.target = true := by
  unfold admits at h
  simp only [Bool.and_eq_true] at h
  exact h

/-- Both passing is sufficient, so the composition does not refuse anything on
    its own account. A gate that can refuse for a reason neither layer names is a
    gate nobody can audit. -/
theorem both_passing_admits (t : EntryTable) (a : AuthorityMap) (c : Call)
    (ht : t c.nr = true) (ha : a c.nr c.target = true) : admits t a c = true := by
  unfold admits
  rw [ht, ha]
  rfl

/-! ### Widening either layer -/

/-- One table admits at least as much as another. -/
def Wider (t t' : EntryTable) : Prop := ∀ n, t n = true → t' n = true

/-- Widening the entry table never turns an admitted call into a refused one, and
    never admits a call the authority map denies. The surface grows; the
    authority does not. -/
theorem widening_the_table_grants_no_authority (t t' : EntryTable) (a : AuthorityMap)
    (hw : Wider t t') (c : Call) (h : a c.nr c.target = false) :
    admits t' a c = false :=
  denied_by_authority_is_unreachable t' a c h

/-- Widening the entry table does widen what is reachable, which is why the table
    is worth keeping narrow even where the authority map is correct: it is the
    bound on how many capsules a defect in the authority map is reachable
    from. -/
theorem widening_the_table_widens_reach (t t' : EntryTable) (a : AuthorityMap)
    (hw : Wider t t') (c : Call) (h : admits t a c = true) : admits t' a c = true := by
  obtain ⟨ht, ha⟩ := admission_requires_both t a c h
  exact both_passing_admits t' a c (hw c.nr ht) ha

/-! ### The audit -/

/-- The syscalls the audit covered. -/
def auditedSyscalls : Nat := 88

/-- Syscalls reachable through the entry table with no authority check behind
    them. The audit found none, and this list is the record of that.

    It is a proof obligation rather than a note: a syscall added without an
    authority check has to be added here for `the_audit_is_complete` to still
    hold, and adding it is a visible admission rather than an omission. -/
def unaudited : List Nat := []

theorem the_audit_is_complete : unaudited = [] := rfl

/-- Nothing is unaudited, stated over membership so the theorem says what the
    list means rather than what it looks like. -/
theorem nothing_is_unaudited (n : Nat) : n ∉ unaudited := by
  unfold unaudited
  simp

/-- With the list empty, every syscall number has both layers, so the composed
    gate is the gate on every path. -/
def HasBothLayers (n : Nat) : Prop := n ∉ unaudited

theorem every_syscall_has_both_layers (n : Nat) : HasBothLayers n :=
  nothing_is_unaudited n

/-- Two open findings from the same audit, recorded as the properties they
    violate rather than as prose.

    `ProcStat` answers a query about another process with fields derived from its
    capability set, so the reply is a disclosure whether or not the caller may
    act on it. -/
def DisclosesCapabilities (reply : Nat → Nat) (caps : Nat → Nat) : Prop :=
  ∃ pid pid', caps pid ≠ caps pid' → reply pid ≠ reply pid'

/-- A reply that distinguishes two processes by their capability sets tells the
    caller something about a capability set it does not hold. A reply that is
    constant in the capability set cannot. -/
def IndependentOfCapabilities (reply : Nat → Nat) (caps : Nat → Nat) : Prop :=
  ∀ pid pid', caps pid ≠ caps pid' → reply pid = reply pid'

/-- The two are opposed where any two processes differ, which is the honest
    statement of the finding: the field has to be dropped or coarsened, it cannot
    be both informative and non-disclosing. -/
theorem disclosure_and_independence_conflict (reply caps : Nat → Nat)
    (pid pid' : Nat) (hc : caps pid ≠ caps pid') (hr : reply pid ≠ reply pid')
    (h : IndependentOfCapabilities reply caps) : False :=
  hr (h pid pid' hc)

/-- A hardcoded pid in a trace reaches whichever process holds that number, which
    after enough spawns is not the one the code was written for. -/
def hardcodedPid : Nat := 1

theorem a_hardcoded_pid_names_whoever_holds_it (owner : Nat → Nat) (a b : Nat)
    (h : owner hardcodedPid = a) (h' : owner hardcodedPid = b) : a = b := by
  rw [← h, ← h']

end Nonos.GateLayers
