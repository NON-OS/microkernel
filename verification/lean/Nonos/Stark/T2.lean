/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

T2: a capsule executes only if its measurement is in the enrolled set.

Stated over a verifier, so the same sentence can be put to the kernel's
current one and to the one that should replace it. Against the private-leaf
verifier it is false, and `t2_fails_private_leaf` is the forgery: an enrolled
leaf and its path, drawn under the forged capsule's own context, admit a
capsule whose measurement is in no tree. Opening the leaf publicly, and
requiring it to be the capsule's measurement, is what makes it true
(`t2_holds_public_leaf`). Nothing here assumes the hash: the failure needs no
collision, and the repair needs none either, since the leaf is no longer
chosen by the prover.
-/

import Nonos.Stark.Attest

namespace Nonos.Stark.T2

open Nonos.Stark.Merkle Nonos.Stark.Attest

variable {α : Type}

/-- A capsule as the gate sees it: the measurement of its image, and the
    context its spawn derives the challenge from. -/
structure Capsule (α : Type) where
  measurement : α
  ctx : Nat

/-- A measurement is enrolled when some path carries it to the root. -/
def inTree (f : α → α → α) (root : α) (m : α) : Prop :=
  ∃ p : List (Step α), recompute f m p = root

/-- T2 for a verifier `acc`: whatever it admits has its own measurement in the
    tree the kernel trusts. -/
def T2 (acc : Attestation α → α → Capsule α → Prop) (f : α → α → α) (root : α) : Prop :=
  ∀ (c : Capsule α) (a : Attestation α), acc a root c → inTree f root c.measurement

/-- The kernel's verifier under NZKSTRK1: the leaf is the prover's to choose. -/
def privateLeaf (f : α → α → α) (bind : Nat → Nat) : Attestation α → α → Capsule α → Prop :=
  fun a root c => accepts f bind a root c.ctx

/-- The verifier T2 needs, and NZKSTRK2 runs: the opened leaf is the capsule's
    own measurement. -/
def publicLeaf (f : α → α → α) (bind : Nat → Nat) : Attestation α → α → Capsule α → Prop :=
  fun a root c => a.leaf = c.measurement ∧ accepts f bind a root c.ctx

/-- T2 is false for the private-leaf verifier: one enrolled leaf is enough to
    admit any capsule, enrolled or not. -/
theorem t2_fails_private_leaf (f : α → α → α) (bind : Nat → Nat) (root leaf : α)
    (path : List (Step α)) (hroot : recompute f leaf path = root)
    (c : Capsule α) (hc : ¬ inTree f root c.measurement) :
    ¬ T2 (privateLeaf f bind) f root := by
  intro h
  let a : Attestation α := ⟨leaf, path, bind c.ctx⟩
  have hacc : privateLeaf f bind a root c := And.intro hroot rfl
  exact hc (h c a hacc)

/-- T2 holds for the public-leaf verifier. -/
theorem t2_holds_public_leaf (f : α → α → α) (bind : Nat → Nat) (root : α) :
    T2 (publicLeaf f bind) f root := by
  intro c a h
  have h' : a.leaf = c.measurement ∧
      (recompute f a.leaf a.path = root ∧ a.challenge = bind c.ctx) := h
  exact ⟨a.path, by rw [← h'.1]; exact h'.2.1⟩

end Nonos.Stark.T2
