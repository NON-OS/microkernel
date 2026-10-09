/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The spawn admission invariant: a capsule runs only if its attestation verified.
The kernel gate `verify_capsule_attestation` returns `Ok` for a capsule exactly
when its proof passes, and the spawn path admits a capsule only on that `Ok`, so
whatever the sequence of spawns, every admitted capsule was attested. The gate
is `#[must_use]`, so its result cannot be dropped; `Nonos.Stark.Attest` proves
what a passing proof means (an enrolled leaf bound to that capsule). This module
proves the admission side: nothing unattested ever enters the admitted set.
`userland/kernel_proofs` and the attestation host tests discharge the gate
decision on the real code.
-/

namespace Nonos.Spawn

/-- The gate's decision for a capsule: true when its attestation verified. -/
abbrev Attested := Nat → Bool

/-- One spawn: a capsule is admitted only if it is attested, otherwise the
    spawn is rejected and the admitted set is unchanged. -/
def admit (att : Attested) (admitted : List Nat) (cap : Nat) : List Nat :=
  if att cap then cap :: admitted else admitted

/-- A whole run of spawns from an initial admitted set. -/
def run (att : Attested) (admitted : List Nat) : List Nat → List Nat
  | [] => admitted
  | cap :: rest => run att (admit att admitted cap) rest

/-- One admission preserves the invariant: if every currently admitted capsule
    is attested, so is every capsule after admitting one more. -/
theorem admit_preserves (att : Attested) (admitted : List Nat) (cap : Nat)
    (h : ∀ c ∈ admitted, att c = true) :
    ∀ c ∈ admit att admitted cap, att c = true := by
  intro c hc
  unfold admit at hc
  by_cases hcap : att cap = true
  · rw [hcap] at hc
    simp at hc
    rcases hc with hc | hc
    · rw [hc]; exact hcap
    · exact h c hc
  · simp [hcap] at hc
    exact h c hc

/-- The invariant holds along any run: from an all-attested start, every
    admitted capsule stays attested. -/
theorem run_preserves (att : Attested) (trace : List Nat) :
    ∀ (admitted : List Nat), (∀ c ∈ admitted, att c = true) →
      ∀ c ∈ run att admitted trace, att c = true := by
  induction trace with
  | nil => intro admitted h c hc; exact h c hc
  | cons cap rest ih =>
    intro admitted h c hc
    exact ih (admit att admitted cap) (admit_preserves att admitted cap h) c hc

/-- No unattested capsule ever runs: after any sequence of spawns from an empty
    start, every admitted capsule was attested. The attacker chooses the spawn
    order and never gets an unattested capsule admitted. -/
theorem only_attested_capsules_run (att : Attested) (trace : List Nat) (c : Nat)
    (h : c ∈ run att [] trace) : att c = true :=
  run_preserves att trace [] (by intro c hc; exact absurd hc (List.not_mem_nil c)) c h

/-! ### What the gate actually decides

The theorems above take `att` as given, so they propagate an invariant without
saying where it comes from. `attest_gate` supplies it. It has three branches and
one configuration in every build: the features that once turned a refusal into
a log line are deleted, so no build admits a capsule its proof did not admit. -/

/-- The three ways `attest_gate` ends, branch for branch: an empty attestation
    trailer, a trailer `verify_capsule_attestation` accepted, and one it
    rejected. -/
inductive Outcome where
  | noTrailer
  | verified
  | rejected
  deriving DecidableEq, Repr

/-- The gate's decision, mirroring `attest_gate`: only a trailer that verified
    is admitted. -/
def gate : Outcome → Bool
  | .verified => true
  | .noTrailer => false
  | .rejected => false

/-- The attestation predicate the gate supplies, given what each capsule's
    trailer verified to. -/
def attOf (outcome : Nat → Outcome) : Attested :=
  fun c => gate (outcome c)

/-- A capsule is admitted only when its attestation verified. -/
theorem enforcing_admits_only_verified (o : Outcome) (h : gate o = true) :
    o = .verified := by
  cases o <;> simp [gate] at h ⊢

/-- A missing trailer is refused. Worth naming because it is the branch that
    runs before any verification happens, and the one an unsigned capsule
    takes. -/
theorem enforcing_refuses_a_missing_trailer : gate .noTrailer = false := rfl

/-- A failed proof is refused. -/
theorem enforcing_refuses_a_failed_proof : gate .rejected = false := rfl

/-- **The gate property.** In every build, after any sequence of spawns, every
    capsule that ran had a trailer that verified. Unlike
    `only_attested_capsules_run` this does not take the predicate as given: it
    is the gate's own decision, so a branch returning `Ok` where it should
    refuse makes this false. -/
theorem enforcing_run_admits_only_verified (outcome : Nat → Outcome)
    (trace : List Nat) (c : Nat) (h : c ∈ run (attOf outcome) [] trace) :
    outcome c = .verified :=
  enforcing_admits_only_verified (outcome c)
    (only_attested_capsules_run (attOf outcome) trace c h)

end Nonos.Spawn
