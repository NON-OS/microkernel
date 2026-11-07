/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your option)
any later version. See <https://www.gnu.org/licenses/>.

What each modelled handler costs the caller, read from the shipped table,
and how fast a revoke bites. Both follow from the gate theorem and the
generated data; neither restates a requirement by hand.
-/

import Formal.Proofs.Table
import Formal.Proofs.Capabilities

namespace Formal

/-- The capability the table demands for every syscall that runs a handler
    with this effect. -/
def Handler.cap : Handler → Option Nat
  | .other => none
  | .capGrant .. => some Data.capAdmin
  | .capRevoke .. => none
  | .mmap .. => some Data.capMemory
  | .munmap _ => some Data.capMemory
  | .ipcSend .. => some Data.capIPC
  | .surfaceRegister .. => some Data.capGraphicsSurfaceCreate
  | .surfaceShare .. => some Data.capGraphicsSurfaceCreate
  | .surfaceAttach .. => some Data.capGraphicsSurfaceMap
  | .surfaceRelease _ => some Data.capGraphicsSurfaceCreate

/-- Every number in `l` has a table entry that needs bit `i`. -/
def numsNeed (l : List Nat) (i : Nat) : Bool :=
  l.all fun nr =>
    match lookupReq nr with
    | some r => r.needs i
    | none => false

/-- The table agrees with `Handler.cap` for this handler. -/
def Handler.check (h : Handler) : Bool :=
  match h.syscalls, h.cap with
  | some l, some i => numsNeed l i
  | _, _ => true

set_option maxRecDepth 20000 in
theorem Handler.check_true (h : Handler) : h.check = true := by
  cases h <;> simp only [Handler.check, Handler.syscalls, Handler.cap] <;> decide

theorem numsNeed_mem {l : List Nat} {i nr : Nat} (h : numsNeed l i = true) (hm : nr ∈ l) :
    ∃ r, lookupReq nr = some r ∧ r.needs i = true := by
  have := List.all_eq_true.1 h nr hm
  revert this
  cases hl : lookupReq nr with
  | none => intro h; cases h
  | some r => intro h; exact ⟨r, rfl, h⟩

/-- A table entry that needs bit `i` passes only on a word holding `i`. -/
theorem capTableAllows_needs {w : Caps} {l : Bool} {nr i : Nat} {r : Req}
    (hl : lookupReq nr = some r) (hn : r.needs i = true)
    (ha : capTableAllows w l nr = true) : has w i = true := by
  unfold capTableAllows at ha
  rw [hl] at ha
  exact Req.needs_sound i w l r hn ha

/-- **Every modelled handler costs a capability.** In every reachable state,
    a handler that grants or revokes runs only for a holder of Admin, one
    that maps or unmaps memory only for a holder of Memory, an IPC send only
    for a holder of IPC, and the surface calls only for a holder of the
    matching graphics capability. The bit is read from the generated table,
    not written here. -/
theorem handler_requires_capability {P : Platform} {g : GrantRule} {s s' : State}
    (hr : Reachable P g s) {p : Pid} {nr : Nat} {h : Handler} (st : Step P g s (.call p nr h) s')
    {i : Nat} (hi : h.cap = some i) :
    ∃ c, s.caps p = some c ∧ has c.held i = true := by
  cases st with
  | call hg hrun _ =>
  obtain ⟨c, hc, _, _, _, hallow⟩ := gate_on_held hr hg
  refine ⟨c, hc, ?_⟩
  have hchk := Handler.check_true h
  cases hs : h.syscalls with
  | none => cases h <;> simp [Handler.syscalls] at hs <;> simp [Handler.cap] at hi
  | some l =>
      unfold Handler.check at hchk
      rw [hs, hi] at hchk
      unfold Handler.runsUnder at hrun
      rw [hs] at hrun
      have hm : nr ∈ l := by simpa using hrun
      obtain ⟨r, hl, hn⟩ := numsNeed_mem hchk hm
      exact capTableAllows_needs hl hn hallow

/-- The state a revoke of `mask` from `t` leaves: `t`'s word lacks every bit
    of `mask`, and every syscall whose entry needs such a bit is refused to
    `t` from that state on, until a grant gives the bit back. -/
theorem revoked_state {P : Platform} {g : GrantRule} {s' : State} (hr : Reachable P g s')
    {t : Pid} {ct : Capsule} {mask : Caps} (hc : s'.caps t = some (ct.revoke t mask))
    {i : Nat} (hi : has mask i = true) :
    has (ct.revoke t mask).held i = false ∧
      ∀ n r, lookupReq n = some r → r.needs i = true → gate P s' t n = false := by
  have hheld : has (ct.revoke t mask).held i = false := by
    simp only [Capsule.revoke, has_excess, hi, Bool.not_true, Bool.and_false]
  refine ⟨hheld, fun n r hl hn => ?_⟩
  cases hg : gate P s' t n with
  | false => rfl
  | true =>
      obtain ⟨c, hc', _, _, _, ha⟩ := gate_on_held hr hg
      rw [hc] at hc'
      cases hc'
      rw [capTableAllows_needs hl hn ha] at hheld
      cases hheld

/-- **A revoke takes effect on the next call.** After `sys_cap_revoke`
    removes `mask` from `t`, `t` holds none of those bits, and every syscall
    whose table entry needs one of them is refused to `t` in the very next
    state: the revocation epoch is bumped and the token re-minted with its
    list in the same step, so no stale authority survives it. -/
theorem revoke_takes_effect {P : Platform} {g : GrantRule} {s s' : State}
    (hr : Reachable P g s) {p : Pid} {nr : Nat} {t : Pid} {mask : Caps}
    (st : Step P g s (.call p nr (.capRevoke t mask)) s') {i : Nat} (hi : has mask i = true) :
    ∃ c', s'.caps t = some c' ∧ has c'.held i = false ∧
      ∀ n r, lookupReq n = some r → r.needs i = true → gate P s' t n = false := by
  have hr' : Reachable P g s' := Reachable.step hr st
  cases st with
  | call _ _ eff =>
      cases eff with
      | @capRevoke _ _ _ ct _ _ _ =>
          have hc : (upd s.caps t (some (ct.revoke t mask))) t = some (ct.revoke t mask) :=
            upd_same _ _ _
          exact ⟨_, hc, revoked_state hr' hc hi⟩

/-- The same for the kernel's own quarantine revoke
    (`process::core::isolation`). -/
theorem quarantine_takes_effect {P : Platform} {g : GrantRule} {s s' : State}
    (hr : Reachable P g s) {t : Pid} {mask : Caps}
    (st : Step P g s (.quarantine t mask) s') {i : Nat} (hi : has mask i = true) :
    ∃ c', s'.caps t = some c' ∧ has c'.held i = false ∧
      ∀ n r, lookupReq n = some r → r.needs i = true → gate P s' t n = false := by
  have hr' : Reachable P g s' := Reachable.step hr st
  cases st with
  | @quarantine _ _ ct _ =>
      have hc : (upd s.caps t (some (ct.revoke t mask))) t = some (ct.revoke t mask) :=
        upd_same _ _ _
      exact ⟨_, hc, revoked_state hr' hc hi⟩

end Formal
