/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The boot profile's capability trim
(`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs`).

On an Air-Gapped, Safe Mode or Recovery boot no program keeps Network (bit 2);
on every boot Debug is withheld from a secret holder. `caps` is applied where
every verified capsule starts, so these theorems say what any capsule can start
with on such a boot.
-/

import Nonos.SecretLog

namespace Nonos.ProfileTrim

open Nonos.WordSet Nonos.SecretLog

def network : Nat := 2 ^ 2

/-- `profile_gate::caps` for a boot whose profile allows the network or not. -/
def trim (caps : Nat) (networkAllowed dev : Bool) : Nat :=
  let c := unlogged caps dev
  if networkAllowed then c else excess c network

theorem trim_within (caps : Nat) (n dev : Bool) : Within (trim caps n dev) caps := by
  unfold trim
  split
  · exact unlogged_within caps dev
  · exact within_trans (within_excess _ _) (unlogged_within caps dev)

/-- **No network on an offline boot.** -/
theorem offline_has_no_network (caps : Nat) (dev : Bool) : bit (trim caps false dev) 2 = false := by
  simp only [trim, Bool.false_eq_true, if_false]
  exact excess_disjoint _ _ _ (by decide)

/-- With the network allowed, the trim is the secret rule alone. -/
theorem online_is_unlogged (caps : Nat) (dev : Bool) : trim caps true dev = unlogged caps dev := by
  simp [trim]

/-- An offline boot is never more generous than an online one. -/
theorem offline_within_online (caps : Nat) (dev : Bool) :
    Within (trim caps false dev) (trim caps true dev) := by
  simp only [trim, Bool.false_eq_true, if_false, if_true]
  exact within_excess _ _

/-- Network is the only bit an offline boot takes beyond the online trim. -/
theorem offline_differs_only_in_network (caps : Nat) (dev : Bool) (i : Nat) (hi : i ≠ 2) :
    bit (trim caps false dev) i = bit (trim caps true dev) i := by
  simp only [trim, Bool.false_eq_true, if_false, if_true]
  rw [bit_excess]
  have : bit network i = false := by
    unfold bit network
    rw [Nat.testBit_two_pow]; simp [Ne.symm hi]
  rw [this]; cases bit (unlogged caps dev) i <;> rfl

/-- Secret bits pass through the trim untouched, online or off. -/
theorem trim_keeps_secrets (caps : Nat) (n dev : Bool) (i : Nat) (hi : bit secrets i = true) :
    bit (trim caps n dev) i = bit caps i := by
  have h8 : i ≠ 8 := by intro h; subst h; revert hi; decide
  have h2 : i ≠ 2 := by intro h; subst h; revert hi; decide
  cases n
  · rw [offline_differs_only_in_network caps dev i h2]
    simp only [trim, if_true]
    exact only_debug_removed caps dev i h8
  · simp only [trim, if_true]
    exact only_debug_removed caps dev i h8

end Nonos.ProfileTrim
