/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Who may revoke (`src/syscall/microkernel/capability/grant_rule.rs`,
`revoke_decision`). A process may drop its own capabilities; reaching another
needs Admin and being init or the target's parent; a protected service
(compositor, input router, policy, the net stack) answers to init alone. The
theorems give the rule's exact shape, its consequences for the three kinds of
caller, and that it only ever narrows the old rule (Admin alone).
-/

namespace Nonos.RevokeBound

/-- `revoke_decision`. -/
def decision (same admin init parent prot : Bool) : Bool :=
  if same then true
  else if !admin then false
  else if prot then init
  else init || parent

/-- The rule before the bound: Admin alone, for any target. -/
def oldDecision (admin : Bool) : Bool := admin

theorem decision_iff (same admin init parent prot : Bool) :
    decision same admin init parent prot = true ↔
      same = true ∨ (admin = true ∧ (prot = true → init = true) ∧ (init = true ∨ parent = true)) := by
  cases same <;> cases admin <;> cases init <;> cases parent <;> cases prot <;> simp [decision]

theorem self_always (admin init parent prot : Bool) :
    decision true admin init parent prot = true := rfl

theorem no_admin_only_self (init parent prot : Bool) :
    decision false false init parent prot = false := by
  cases prot <;> rfl

/-- A stranger with Admin, neither init nor the parent, revokes nothing. -/
theorem stranger_refused (prot : Bool) : decision false true false false prot = false := by
  cases prot <;> rfl

/-- Only init reaches a protected service. -/
theorem protected_only_init (parent : Bool) : decision false true false parent true = false := rfl

theorem init_reaches_all (prot parent : Bool) : decision false true true parent prot = true := by
  cases prot <;> rfl

/-- A parent reaches its ordinary child. -/
theorem parent_reaches_child : decision false true false true false = true := rfl

/-- **The bound only narrows.** Every revoke of another process the new rule
    admits, the old rule admitted. -/
theorem narrows_old (admin init parent prot : Bool) :
    decision false admin init parent prot = true → oldDecision admin = true := by
  cases admin <;> cases init <;> cases parent <;> cases prot <;> simp [decision, oldDecision]

/-- **What the old rule allowed.** Any Admin holder could revoke from a
    protected service it neither started nor is init. -/
theorem old_rule_reached_protected :
    oldDecision true = true ∧ decision false true false false true = false := ⟨rfl, rfl⟩

/-- The decision does not depend on the target's parent when the caller is
    init. -/
theorem init_ignores_parent (prot p q : Bool) :
    decision false true true p prot = decision false true true q prot := by
  cases prot <;> rfl

end Nonos.RevokeBound
