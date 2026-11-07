/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The resolver chain (`src/syscall/contract/resolver/resolve.rs`).

Before any handler, `resolve` runs five checks on the caller's token in a fixed
order: the token's MAC, expiry and revocation; the boot session; the ASID
binding; the revocation epoch; and the syscall allowlist. The first failure
names the error. The theorems: the call is admitted exactly when all five
pass, whatever their order; the error is the first failing check; a token
minted before the last revoke, for another address space or for another boot,
is refused.
-/

namespace Nonos.ResolverChain

inductive Err where
  | token
  | session
  | asid
  | epoch
  | syscall
  deriving DecidableEq, Repr

structure Ctx where
  currentAsid : Nat
  bootNonce : Option Nat
  capsuleEpoch : Nat

structure Tok where
  macOk : Bool
  notExpired : Bool
  notRevoked : Bool
  session : Nat
  asid : Nat
  epoch : Nat

def checkToken (t : Tok) : Bool := t.macOk && t.notExpired && t.notRevoked
def checkSession (t : Tok) (c : Ctx) : Bool := c.bootNonce == some t.session
def checkAsid (t : Tok) (c : Ctx) : Bool := t.asid == c.currentAsid
def checkEpoch (t : Tok) (c : Ctx) : Bool := decide (c.capsuleEpoch ≤ t.epoch)

/-- `resolve`, with `allowed` the allowlist decision for the call. -/
def resolve (t : Tok) (c : Ctx) (allowed : Bool) : Except Err Unit :=
  if !checkToken t then .error .token
  else if !checkSession t c then .error .session
  else if !checkAsid t c then .error .asid
  else if !checkEpoch t c then .error .epoch
  else if !allowed then .error .syscall
  else .ok ()

theorem resolve_ok_iff (t : Tok) (c : Ctx) (a : Bool) :
    resolve t c a = .ok () ↔
      checkToken t = true ∧ checkSession t c = true ∧ checkAsid t c = true ∧
        checkEpoch t c = true ∧ a = true := by
  unfold resolve
  cases checkToken t <;> cases checkSession t c <;> cases checkAsid t c <;>
    cases checkEpoch t c <;> cases a <;> simp

/-- A token minted under an older epoch than the capsule's is refused. -/
theorem stale_epoch_refused (t : Tok) (c : Ctx) (a : Bool) (h : t.epoch < c.capsuleEpoch) :
    resolve t c a ≠ .ok () := by
  rw [Ne, resolve_ok_iff]
  intro ⟨_, _, _, he, _⟩
  simp [checkEpoch] at he
  omega

/-- A token bound to another address space is refused. -/
theorem foreign_asid_refused (t : Tok) (c : Ctx) (a : Bool) (h : t.asid ≠ c.currentAsid) :
    resolve t c a ≠ .ok () := by
  rw [Ne, resolve_ok_iff]
  intro ⟨_, _, ha, _, _⟩
  simp [checkAsid] at ha
  exact h ha

/-- Before the boot session nonce is latched, nothing is admitted. -/
theorem no_session_refused (t : Tok) (c : Ctx) (a : Bool) (h : c.bootNonce = none) :
    resolve t c a ≠ .ok () := by
  rw [Ne, resolve_ok_iff]
  intro ⟨_, hs, _, _, _⟩
  simp [checkSession, h] at hs

/-- A token from another boot is refused. -/
theorem other_boot_refused (t : Tok) (c : Ctx) (a : Bool) (n : Nat) (h : c.bootNonce = some n)
    (hn : t.session ≠ n) : resolve t c a ≠ .ok () := by
  rw [Ne, resolve_ok_iff]
  intro ⟨_, hs, _, _, _⟩
  simp [checkSession, h] at hs
  exact hn hs.symm

/-- A forged, expired or revoked token is refused before anything else is read. -/
theorem bad_token_first (t : Tok) (c : Ctx) (a : Bool) (h : checkToken t = false) :
    resolve t c a = .error .token := by
  simp [resolve, h]

/-- The allowlist is consulted only for a token that passed every binding check. -/
theorem allowlist_last (t : Tok) (c : Ctx)
    (h : checkToken t = true ∧ checkSession t c = true ∧ checkAsid t c = true ∧ checkEpoch t c = true) :
    resolve t c false = .error .syscall := by
  obtain ⟨h1, h2, h3, h4⟩ := h
  simp [resolve, h1, h2, h3, h4]

end Nonos.ResolverChain
