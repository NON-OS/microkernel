/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The image ceiling gate (`src/security/image_ceiling/rule.rs`).

`over(requested, ceiling)` is the part of a request the image forbids, and
`gate` admits a request exactly when that part is empty. The spawn path runs
it on the install word; the name of every forbidden bit goes to the serial log.
-/

import Nonos.WordSet

namespace Nonos.ImageCeilingGate

open Nonos.WordSet

/-- `over`: `requested & !ceiling`. -/
def over (requested ceiling : Nat) : Nat := excess requested ceiling

/-- `gate`: `Ok` when nothing is over, `Err(excess)` otherwise. -/
def gate (requested ceiling : Nat) : Except Nat Unit :=
  if over requested ceiling = 0 then .ok () else .error (over requested ceiling)

theorem gate_ok_iff (r c : Nat) : gate r c = .ok () ↔ Within r c := by
  unfold gate Within over
  split <;> simp_all

theorem over_within_request (r c : Nat) : Within (over r c) r := within_excess r c

/-- What the gate names is forbidden: no reported bit is in the ceiling. -/
theorem over_outside_ceiling (r c i : Nat) (h : bit (over r c) i = true) : bit c i = false := by
  unfold over at h
  rw [bit_excess] at h
  cases hc : bit c i
  · rfl
  · rw [hc] at h; simp at h

/-- A refused request is refused with exactly its excess. -/
theorem gate_error (r c : Nat) (h : ¬ Within r c) : gate r c = .error (over r c) := by
  unfold gate
  rw [if_neg (show ¬ over r c = 0 from h)]

/-- Raising the ceiling never refuses what a lower one admitted. -/
theorem gate_monotone {r c c' : Nat} (hc : Within c c') (h : gate r c = .ok ()) :
    gate r c' = .ok () :=
  (gate_ok_iff r c').2 (within_trans ((gate_ok_iff r c).1 h) hc)

/-- A smaller request passes wherever a larger one does. -/
theorem gate_antitone {r r' c : Nat} (hr : Within r' r) (h : gate r c = .ok ()) :
    gate r' c = .ok () :=
  (gate_ok_iff r' c).2 (within_trans hr ((gate_ok_iff r c).1 h))

theorem gate_zero (c : Nat) : gate 0 c = .ok () := (gate_ok_iff 0 c).2 (within_zero c)

end Nonos.ImageCeilingGate
