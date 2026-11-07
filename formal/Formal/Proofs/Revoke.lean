/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Who may revoke (`grant_rule::revoke_decision`, `sys_cap_revoke`). Before
this rule any Admin holder could clear any bit from any process, so the
least trusted Admin holder could take IPC from the compositor or Network from
the net stack. Now a process may drop its own bits freely; reaching another
needs Admin and being init or the target's parent; and a protected service
answers to init alone.
-/

import Formal.Step

namespace Formal

theorem revokeDecision_spec {same a i par prot : Bool}
    (h : revokeDecision same a i par prot = true) :
    same = true ∨ (a = true ∧ (prot = true → i = true) ∧ (i = true ∨ par = true)) := by
  cases same <;> cases a <;> cases i <;> cases par <;> cases prot <;>
    simp_all [revokeDecision]

/-- Dropping one's own bits needs nothing. -/
theorem self_revoke_allowed (a i par prot : Bool) : revokeDecision true a i par prot = true := rfl

/-- **The revoke bound.** In any state, a revoke by `p` that removes bits from
    a different process `t` runs only if `p` holds Admin and is init or `t`'s
    parent; if `t` owns a protected service, `p` is init. -/
theorem revoke_bound {P : Platform} {g : GrantRule} {s s' : State} {p : Pid} {nr : Nat}
    {t : Pid} {mask : Caps} (st : Step P g s (.call p nr (.capRevoke t mask)) s') (hne : p ≠ t) :
    (∃ cp, s.caps p = some cp ∧ has cp.held Data.capAdmin = true) ∧
    (p = initPid ∨ p = P.parentOf t) ∧ (P.protectedSvc t = true → p = initPid) := by
  cases st with
  | call _ _ eff =>
      cases eff with
      | @capRevoke _ _ cp _ hcp hdec _ =>
          rcases revokeDecision_spec hdec with hs | ⟨ha, hprot, hip⟩
          · simp only [beq_iff_eq] at hs; exact absurd hs hne
          · refine ⟨⟨cp, hcp, ha⟩, ?_, ?_⟩
            · simp only [beq_iff_eq] at hip
              exact hip
            · intro hp
              simpa using hprot hp

/-- Only init can remove bits from a protected service through the syscall. -/
theorem protected_service_answers_to_init {P : Platform} {g : GrantRule} {s s' : State}
    {p : Pid} {nr : Nat} {t : Pid} {mask : Caps}
    (st : Step P g s (.call p nr (.capRevoke t mask)) s') (hne : p ≠ t)
    (hprot : P.protectedSvc t = true) : p = initPid :=
  (revoke_bound st hne).2.2 hprot

end Formal
