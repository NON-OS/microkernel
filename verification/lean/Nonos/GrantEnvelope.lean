/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The capability grant rule (`src/syscall/microkernel/capability/grant_rule.rs`)
and the envelope it reads (`src/process/caps_envelope.rs`).

`sys_cap_grant` used to check only the caller: hold Admin, hold every bit of
the mask. A caller that passed could OR any of its bits into any live process.
The rule now also requires the mask to lie inside the target's envelope, the
manifest's required and optional bits under the image ceiling, recorded once
at spawn. The theorems state the decision, show it never lifts a target past
its envelope however many grants arrive, and keep the old rule's
counterexample as a theorem so the bug that was fixed stays on the record.
-/

import Nonos.WordSet

namespace Nonos.GrantEnvelope

open Nonos.WordSet

/-- `may_grant`: the caller holds `admin` and every bit of `mask`. -/
def mayGrant (held admin mask : Nat) : Bool :=
  (held &&& admin) != 0 && decide (Within mask held)

/-- `within_envelope`. -/
def withinEnvelope (mask envelope : Nat) : Bool := decide (Within mask envelope)

/-- `grant_decision`: both processes exist, the caller may grant, and the mask
    is inside the target's envelope. -/
def grantDecision (caller : Option Nat) (envelope : Option Nat) (admin mask : Nat) : Bool :=
  match caller, envelope with
  | some held, some env => mayGrant held admin mask && withinEnvelope mask env
  | _, _ => false

/-- `may_revoke`. -/
def mayRevoke (held admin : Nat) : Bool := (held &&& admin) == admin

/-- The rule before the envelope existed. -/
def oldDecision (caller : Option Nat) (target : Option Nat) (admin mask : Nat) : Bool :=
  match caller, target with
  | some held, some _ => mayGrant held admin mask
  | _, _ => false

theorem missing_caller_refused (env : Option Nat) (admin mask : Nat) :
    grantDecision none env admin mask = false := by
  cases env <;> rfl

theorem missing_target_refused (held : Option Nat) (admin mask : Nat) :
    grantDecision held none admin mask = false := by
  cases held <;> rfl

theorem decision_spec {held env admin mask : Nat}
    (h : grantDecision (some held) (some env) admin mask = true) :
    (held &&& admin) ≠ 0 ∧ Within mask held ∧ Within mask env := by
  simp only [grantDecision, mayGrant, withinEnvelope, Bool.and_eq_true, bne_iff_ne,
    decide_eq_true_eq] at h
  exact ⟨h.1.1, h.1.2, h.2⟩

/-- The decision is exactly its three conditions. -/
theorem decision_iff (held env admin mask : Nat) :
    grantDecision (some held) (some env) admin mask = true ↔
      (held &&& admin) ≠ 0 ∧ Within mask held ∧ Within mask env := by
  constructor
  · exact decision_spec
  · intro ⟨h1, h2, h3⟩
    simp only [grantDecision, mayGrant, withinEnvelope, Bool.and_eq_true, bne_iff_ne,
      decide_eq_true_eq]
    exact ⟨⟨h1, h2⟩, h3⟩

/-- A caller without Admin grants nothing. -/
theorem no_admin_no_grant {held env admin mask : Nat} (h : (held &&& admin) = 0) :
    grantDecision (some held) (some env) admin mask = false := by
  cases hd : grantDecision (some held) (some env) admin mask
  · rfl
  · exact absurd h (decision_spec hd).1

/-- A process with an empty envelope can be granted only the empty mask. -/
theorem empty_envelope_grants_nothing {held admin mask : Nat}
    (h : grantDecision (some held) (some 0) admin mask = true) : mask = 0 :=
  zero_of_within_zero (decision_spec h).2.2

/-- A grant that passes keeps the target inside its envelope. -/
theorem grant_stays_in_envelope {held env admin mask target : Nat}
    (h : grantDecision (some held) (some env) admin mask = true) (ht : Within target env) :
    Within (target ||| mask) env :=
  within_or ht (decision_spec h).2.2

/-- Any sequence of passing grants keeps the target inside its envelope. -/
theorem grants_stay_in_envelope (env admin : Nat) :
    ∀ (masks : List (Nat × Nat)) (target : Nat), Within target env →
      (∀ p ∈ masks, grantDecision (some p.1) (some env) admin p.2 = true) →
      Within (masks.foldl (fun t p => t ||| p.2) target) env
  | [], _, ht, _ => ht
  | p :: ps, t, ht, hall => by
      simp only [List.foldl]
      exact grants_stay_in_envelope env admin ps _
        (grant_stays_in_envelope (hall p (List.Mem.head _)) ht)
        (fun q hq => hall q (List.Mem.tail _ hq))

/-- What a passing grant hands over, the granter already held. -/
theorem grant_from_held {held env admin mask : Nat}
    (h : grantDecision (some held) (some env) admin mask = true) : Within mask held :=
  (decision_spec h).2.1

/-- The new rule refuses everything the old rule refused. -/
theorem new_rule_narrows (c t : Option Nat) (admin mask : Nat) :
    grantDecision c t admin mask = true → oldDecision c t admin mask = true := by
  cases c <;> cases t <;> simp only [grantDecision, oldDecision, Bool.and_eq_true] <;>
    first | exact fun h => h.1 | exact fun h => h

/-- **The bug that was fixed.** Under the old rule an Admin holder granted
    Admin (bit 9) to a target whose envelope is IPC (bit 3) alone. -/
theorem old_rule_exceeds_envelope :
    oldDecision (some (512 ||| 8)) (some 8) 512 512 = true ∧
      ¬ Within (8 ||| 512) 8 := by
  decide

/-- The new rule refuses that grant. -/
theorem new_rule_refuses_it : grantDecision (some (512 ||| 8)) (some 8) 512 512 = false := by
  decide

/-- Revoke asks only for Admin. -/
theorem revoke_iff (held admin : Nat) : mayRevoke held admin = true ↔ (held &&& admin) = admin := by
  simp [mayRevoke]

end Nonos.GrantEnvelope
