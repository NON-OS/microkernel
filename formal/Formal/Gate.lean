/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The syscall gate. Every architecture's trap handler calls
`syscall::entry::handle_syscall`, which refuses a number the ABI registry does
not resolve (ENOSYS) and otherwise calls `contract::dispatch`. That runs
`resolver::resolve` on the caller's token: signature, expiry and revocation,
boot session, ASID binding, revocation epoch, then `cap_table::is_allowed`;
only on success does it reach `handle_syscall_dispatch` and a handler. The
table is generated from the Rust (`Formal.Data.Syscalls`), so the gate below
is the shipped one, not a restatement.
-/

import Formal.State
import Formal.Req
import Formal.Data.Syscalls
import Formal.Data.CapBits
import Formal.Admission

namespace Formal

/-- The requirement `cap_table::is_allowed` applies to `nr`: the first group
    that claims it, or none. -/
def lookupReq (nr : Nat) : Option Req :=
  (Data.capTable.find? (·.1 == nr)).map (·.2)

/-- `SyscallNumber::from_u64` resolves `nr`. -/
def inAbi (nr : Nat) : Bool :=
  Data.abiSyscalls.any (·.1 == nr)

/-- `cap_table::is_allowed`: an unclaimed number is refused. -/
def capTableAllows (perms : Caps) (launcher : Bool) (nr : Nat) : Bool :=
  match lookupReq nr with
  | none => false
  | some r => r.eval perms launcher

/-- `allowlist::derive`: every registered slot asked of the cap table twice,
    once per launcher answer, on the token's word. A number with no registry
    slot is refused. -/
def derive (perms : Caps) : Nat → Bool → Bool :=
  fun nr l => inAbi nr && capTableAllows perms l nr

/-- `allowlist::allowed(list, nr, caller_launches)`. -/
def allowed (list : Nat → Bool → Bool) (nr : Nat) (launches : Bool) : Bool :=
  list nr launches

/-- `process::caps::install_token`: a token for `p` under epoch `e` over the
    word `w`, with its allowlist derived from `w` (`allowlist::for_token`). -/
def mint (p : Pid) (e : Nat) (w : Caps) : Token := ⟨p, e, w, derive w⟩

/-- The resolver's binding checks: the token is the caller's own and was
    minted under the current revocation epoch (`check_asid_binding`,
    `check_revocation_epoch`). -/
def tokenOk (p : Pid) (c : Capsule) : Bool :=
  c.token.subject == p && decide (c.epoch ≤ c.token.epoch)

/-- The whole gate between a trap and a handler: the resolver's binding
    checks, then the bit test against the token's allowlist
    (`allowlist::decide`). -/
def gate (P : Platform) (s : State) (p : Pid) (nr : Nat) : Bool :=
  match s.caps p with
  | none => false
  | some c => tokenOk p c && allowed c.token.list nr (P.launcher p)

theorem gate_some {P : Platform} {s : State} {p : Pid} {nr : Nat} (h : gate P s p nr = true) :
    ∃ c, s.caps p = some c ∧ tokenOk p c = true ∧ allowed c.token.list nr (P.launcher p) = true := by
  unfold gate at h
  cases hc : s.caps p with
  | none => rw [hc] at h; simp at h
  | some c =>
      rw [hc] at h
      simp only [Bool.and_eq_true] at h
      exact ⟨c, rfl, h.1, h.2⟩

end Formal
