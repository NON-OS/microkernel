/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Debug withheld from a secret holder
(`src/kernel_core/process_spawn/capsule_spawn/runner/secret_log.rs`).

A capsule that holds a key, raw input or a device secret would leak it through
a debug channel, so off the development profile the spawn path removes Debug
(bit 8) from any word holding one of the secret bits (21, 28, 35, 36). The
theorems: the rule only removes, it removes Debug and nothing else, it acts
exactly on secret holders, and it is idempotent.
-/

import Nonos.WordSet

namespace Nonos.SecretLog

open Nonos.WordSet

def debug : Nat := 2 ^ 8
def secrets : Nat := 2 ^ 21 ||| 2 ^ 28 ||| 2 ^ 35 ||| 2 ^ 36

/-- `unlogged(caps, dev)`. -/
def unlogged (caps : Nat) (dev : Bool) : Nat :=
  if dev || (caps &&& secrets) == 0 then caps else excess caps debug

theorem unlogged_within (caps : Nat) (dev : Bool) : Within (unlogged caps dev) caps := by
  unfold unlogged
  split
  · exact within_refl _
  · exact within_excess _ _

theorem dev_keeps_all (caps : Nat) : unlogged caps true = caps := by
  simp [unlogged]

theorem no_secret_keeps_all (caps : Nat) (dev : Bool) (h : (caps &&& secrets) = 0) :
    unlogged caps dev = caps := by
  simp [unlogged, h]

theorem secret_holder_loses_debug (caps : Nat) (h : (caps &&& secrets) ≠ 0) :
    bit (unlogged caps false) 8 = false := by
  simp only [unlogged, Bool.false_or, h, beq_iff_eq, if_false]
  exact excess_disjoint _ _ _ (by decide)

/-- Every bit other than Debug survives. -/
theorem only_debug_removed (caps : Nat) (dev : Bool) (i : Nat) (hi : i ≠ 8) :
    bit (unlogged caps dev) i = bit caps i := by
  unfold unlogged
  split
  · rfl
  · rw [bit_excess]
    have : bit debug i = false := by
      unfold bit debug
      rw [Nat.testBit_two_pow]
      simp [Ne.symm hi]
    rw [this]; cases bit caps i <;> rfl

/-- The secret bits themselves are never removed. -/
theorem secrets_kept (caps : Nat) (dev : Bool) :
    (unlogged caps dev &&& secrets) = (caps &&& secrets) := by
  apply Nat.eq_of_testBit_eq
  intro i
  show bit _ i = bit _ i
  rw [bit_and, bit_and]
  by_cases hi : i = 8
  · subst hi
    have : bit secrets 8 = false := by decide
    rw [this]; cases bit _ 8 <;> cases bit caps 8 <;> rfl
  · rw [only_debug_removed caps dev i hi]

theorem unlogged_idem (caps : Nat) (dev : Bool) :
    unlogged (unlogged caps dev) dev = unlogged caps dev := by
  apply Nat.eq_of_testBit_eq
  intro i
  show bit _ i = bit _ i
  by_cases hi : i = 8
  · subst hi
    cases dev
    · by_cases hs : (caps &&& secrets) = 0
      · rw [no_secret_keeps_all caps false hs, no_secret_keeps_all caps false hs]
      · have hs' : (unlogged caps false &&& secrets) ≠ 0 := by rw [secrets_kept]; exact hs
        rw [secret_holder_loses_debug _ hs', secret_holder_loses_debug _ hs]
    · simp [dev_keeps_all]
  · rw [only_debug_removed _ dev i hi]

end Nonos.SecretLog
