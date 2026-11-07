/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What a process created by the kernel inherits
(`src/process/core/table/inherit.rs`, `ambient.rs`).

A process the kernel creates outside the manifest path starts with its
parent's word cut down to the ambient set, CoreExec, IPC and Memory; pid 1
starts with the ambient set itself; a process whose parent is gone starts with
nothing. The ambient set is checked at compile time to hold none of the
forbidden bits: Admin, Debug, every hardware and graphics bit, SpawnBroker,
DeviceSecret, InputConsumer and the capture bits. The theorems carry that
check into what any such process can hold.
-/

import Nonos.WordSet

namespace Nonos.InheritBound

open Nonos.WordSet

def coreExec : Nat := 0
def ipc : Nat := 3
def memory : Nat := 4

/-- `AMBIENT_CAPS`. -/
def ambient : Nat := 2 ^ coreExec ||| 2 ^ ipc ||| 2 ^ memory

/-- `FORBIDDEN_AMBIENT`: Admin 9, Driver 16, DeviceEnum 15, Mmio 17, Irq 18,
    Dma 19, Pio 20, Debug 8, the four graphics bits 11 to 14, SpawnBroker 23,
    DeviceSecret 35, InputConsumer 36 and the capture bits 37 to 39. -/
def forbidden : Nat :=
  2 ^ 9 ||| 2 ^ 16 ||| 2 ^ 15 ||| 2 ^ 17 ||| 2 ^ 18 ||| 2 ^ 19 ||| 2 ^ 20 ||| 2 ^ 8 |||
  2 ^ 11 ||| 2 ^ 12 ||| 2 ^ 13 ||| 2 ^ 14 ||| 2 ^ 23 ||| 2 ^ 35 ||| 2 ^ 36 |||
  2 ^ 37 ||| 2 ^ 38 ||| 2 ^ 39

/-- The compile-time assertion in `ambient.rs`. -/
theorem ambient_disjoint_forbidden : (ambient &&& forbidden) = 0 := by decide

/-- `compute_inherited_caps`: pid 1, a live parent's word, or no parent. -/
def inherited (pid : Nat) (parent : Option Nat) : Nat :=
  if pid = 1 then ambient &&& ambient
  else match parent with
    | some caps => caps &&& ambient
    | none => 0

theorem inherited_within_ambient (pid : Nat) (parent : Option Nat) :
    Within (inherited pid parent) ambient := by
  unfold inherited
  split
  · exact within_and_right _ _
  · cases parent with
    | some c => exact within_and_right _ _
    | none => exact within_zero _

theorem inherited_within_parent (pid : Nat) (caps : Nat) (h : pid ≠ 1) :
    Within (inherited pid (some caps)) caps := by
  simp only [inherited, h, if_false]
  exact within_and_left _ _

theorem orphan_inherits_nothing (pid : Nat) (h : pid ≠ 1) : inherited pid none = 0 := by
  simp [inherited, h]

/-- **No kernel-created process starts with a forbidden bit.** -/
theorem inherited_never_forbidden (pid : Nat) (parent : Option Nat) (i : Nat)
    (hf : bit forbidden i = true) : bit (inherited pid parent) i = false := by
  have ha : bit ambient i = false := by
    cases h : bit ambient i
    · rfl
    · have := congrArg (fun n => Nat.testBit n i) ambient_disjoint_forbidden
      simp only [Nat.testBit_and, Nat.zero_testBit] at this
      simp only [bit] at h hf
      rw [h, hf] at this
      cases this
  cases hi : bit (inherited pid parent) i
  · rfl
  · rw [bit_of_within (inherited_within_ambient pid parent) hi] at ha
    cases ha

/-- In particular none starts with Admin. -/
theorem inherited_never_admin (pid : Nat) (parent : Option Nat) :
    bit (inherited pid parent) 9 = false :=
  inherited_never_forbidden pid parent 9 (by decide)

/-- Inheritance down a chain never grows: a grandchild holds at most what its
    grandparent held, and at most the ambient set. -/
theorem chain_narrows (c : Nat) (p1 p2 : Nat) (h1 : p1 ≠ 1) (h2 : p2 ≠ 1) :
    Within (inherited p2 (some (inherited p1 (some c)))) c :=
  within_trans (inherited_within_parent p2 _ h2) (inherited_within_parent p1 c h1)

end Nonos.InheritBound
