/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The bug the grant bound fixed. Before `grant_rule::grant_decision`,
`sys_cap_grant` let a capsule holding Admin OR any bits it held into any live
capsule's word, with nothing bounding that by the target's manifest
(`GrantRule.callerOnly`). Under that rule "no capsule ever holds a capability
its manifest did not grant" was false as soon as the image let a capsule hold
Admin. The trace below is the witness: two signed capsules are admitted, the
one with Admin grants Admin to the other, and the other holds a bit its
manifest never named. Every step passes the real gate and admission decision.
`fixed_rule_refuses_the_old_witness` shows the shipped rule
(`GrantRule.targetEnvelope`) refuses the same grant.
-/

import Formal.Proofs.Gate

namespace Formal.AdminGrant

open Formal

/-- A platform whose primitives accept: the witness is about the grant path,
    not about forging a signature. Ceiling and profile keep bits 0 to 9. -/
def platform : Platform where
  sigVerify := fun _ _ _ _ => true
  keyRevoked := fun _ => false
  imageCeiling := 1023
  profileKeep := 1023
  launcher := fun _ => false
  parentOf := fun _ => 0
  protectedSvc := fun _ => false

def cert : Cert := ⟨1023, [⟨1, 1, 0⟩, ⟨2, 3, 0⟩]⟩

/-- A manifest signed under both algorithms the policy requires. -/
def request (required : Caps) : Request :=
  ⟨⟨required, 0, [⟨1, 1, 0⟩, ⟨3, 2, 0⟩], 0⟩, cert, 0, true, true⟩

/-- Admin (bit 9) and IPC (bit 3). -/
def adminWord : Caps := 2 ^ Data.capAdmin ||| 2 ^ Data.capIPC
/-- IPC alone. -/
def ipcWord : Caps := 2 ^ Data.capIPC

def c1 : Capsule := Capsule.fresh platform 1 (request adminWord) adminWord
def c2 : Capsule := Capsule.fresh platform 2 (request ipcWord) ipcWord

def s1 : State := { State.init with caps := upd State.init.caps 1 (some c1) }
def s2 : State := { s1 with caps := upd s1.caps 2 (some c2) }
def s3 : State := { s2 with caps := upd s2.caps 2 (some (c2.grant 2 (2 ^ Data.capAdmin))) }

theorem admission1 : admission platform (request adminWord) = some adminWord := by decide
theorem admission2 : admission platform (request ipcWord) = some ipcWord := by decide

set_option maxRecDepth 20000 in
theorem gate_grant : gate platform s2 1 Data.sysMkCapGrant = true := by decide

theorem reach3 : Reachable platform .callerOnly s3 := by
  have r1 : Reachable platform .callerOnly s1 := Reachable.step Reachable.init (Step.spawn rfl admission1)
  have r2 : Reachable platform .callerOnly s2 := Reachable.step r1 (Step.spawn rfl admission2)
  refine Reachable.step r2 (Step.call (h := .capGrant 2 (2 ^ Data.capAdmin)) gate_grant rfl ?_)
  exact Effect.capGrant (cp := c1) (ct := c2) rfl (by decide) (by decide) rfl trivial

/-- **The old grant rule exceeded the manifest.** Under `callerOnly` there
    is a platform and a reachable state in which a capsule holds a capability
    outside everything its manifest declared. -/
theorem old_rule_grant_exceeds_admitted :
    ∃ (P : Platform) (s : State) (p : Pid) (c : Capsule),
      Reachable P .callerOnly s ∧ s.caps p = some c ∧
        ¬ Within c.held (c.request.manifest.required ||| c.request.manifest.optional) :=
  ⟨platform, s3, 2, c2.grant 2 (2 ^ Data.capAdmin), reach3, rfl, by decide⟩

/-- The shipped rule refuses that grant from the same state: Admin is
    outside capsule 2's envelope. -/
theorem fixed_rule_refuses_the_old_witness :
    ¬ ∃ s', Step platform .targetEnvelope s2
      (.call 1 Data.sysMkCapGrant (.capGrant 2 (2 ^ Data.capAdmin))) s' := by
  rintro ⟨s', st⟩
  cases st with
  | call _ _ eff =>
      cases eff with
      | @capGrant _ _ _ ct _ _ _ ht hallow =>
          have : ct = c2 := by
            have h := ht
            simp only [s2, upd_same, Option.some.injEq] at h
            exact h.symm
          subst this
          have h : ¬ Within (2 ^ Data.capAdmin) c2.envelope := by decide
          exact h hallow

end Formal.AdminGrant
