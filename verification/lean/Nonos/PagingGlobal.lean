/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

A user page is never GLOBAL (P-06). With CR4.PGE on, a CR3 write keeps the
GLOBAL entries and drops the rest, so a GLOBAL user entry would survive the
switch to the next process and stay readable to it. The kernel's encoders
(`PagePermissions::to_pte_flags`, `PageTableEntry::to_raw`) drop GLOBAL from a
user page; `userland/kernel_proofs` checks both over every input, and the
Verus theorems in `verification/verus/src/page_global.rs` the bit operations.
On aarch64 both encoders set nG on every EL0 page by the same rule, checked
by `userland/arch_paging_proofs`; the model here is arch-neutral.
-/

namespace Nonos.PagingGlobal

/-- What a mapping asks for: a user page or not, GLOBAL or not. -/
structure Request where
  user : Bool
  global : Bool

/-- An entry as the TLB holds it. -/
structure Entry where
  user : Bool
  global : Bool

/-- The encoders' rule: GLOBAL is kept only on a kernel page. -/
def encode (r : Request) : Entry :=
  { user := r.user, global := r.global && !r.user }

/-- What a CR3 write leaves in the TLB: with PGE set, the GLOBAL entries. -/
def survivesSwitch (pge : Bool) (e : Entry) : Bool :=
  pge && e.global

/-- No encoded user entry is GLOBAL. -/
theorem user_never_global (r : Request) (h : r.user = true) :
    (encode r).global = false := by
  simp [encode, h]

/-- A kernel page keeps the GLOBAL it asked for. -/
theorem kernel_keeps_global (r : Request) (h : r.user = false) :
    (encode r).global = r.global := by
  simp [encode, h]

/-- So no user entry outlives the switch to another address space, whatever
    PGE says: the cross-process read the rule exists to rule out. -/
theorem user_entry_never_survives_switch (pge : Bool) (r : Request)
    (h : r.user = true) : survivesSwitch pge (encode r) = false := by
  simp [survivesSwitch, user_never_global r h]

end Nonos.PagingGlobal
