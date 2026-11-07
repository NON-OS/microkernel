/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The syscall gate over reachable states. A handler runs only for a capsule
whose own, current token's allowlist permits the call; the allowlist is the
one `allowlist::for_token` derived from the token's word, and that word is
the capsule's held word, so the decision is the generated cap table on
exactly the authority the capsule has.
-/

import Formal.Proofs.Capabilities

namespace Formal

theorem gate_on_held {P : Platform} {g : GrantRule} {s : State} (hr : Reachable P g s) {p : Pid}
    {nr : Nat} (hg : gate P s p nr = true) :
    ∃ c, s.caps p = some c ∧ inAbi nr = true ∧ c.token.subject = p ∧ c.epoch ≤ c.token.epoch ∧
      capTableAllows c.held (P.launcher p) nr = true := by
  obtain ⟨c, hc, h2, h3⟩ := gate_some hg
  have ht := (capInv_reachable s hr p c hc).1
  rw [ht] at h3
  simp only [allowed, mint, derive, Bool.and_eq_true] at h3
  simp only [tokenOk, Bool.and_eq_true, beq_iff_eq, decide_eq_true_eq] at h2
  exact ⟨c, hc, h3.1, h2.1, h2.2, h3.2⟩

/-- The table check behind `only_attest_doc_negates`, as one boolean. -/
def negationsOnlyAttestDoc : Bool :=
  Data.capTable.all fun e => !e.2.hasNot || e.1 == Data.sysMkAttestDoc

set_option maxRecDepth 20000 in
theorem negationsOnlyAttestDoc_true : negationsOnlyAttestDoc = true := by decide

/-- Requirements in the shipped table that contain a negation: only
    MkAttestDoc (`can_attest_doc`, refused to a holder of Network). Every
    other syscall's requirement is monotone in the token. -/
theorem only_attest_doc_negates :
    ∀ e ∈ Data.capTable, e.2.hasNot = true → e.1 = Data.sysMkAttestDoc := by
  intro e he hn
  have := List.all_eq_true.1 negationsOnlyAttestDoc_true e he
  simp only [hn, Bool.not_true, Bool.false_or, beq_iff_eq] at this
  exact this

end Formal
