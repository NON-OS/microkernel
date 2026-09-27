/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Bringing up the other processors, in the one order that works.

An application processor starts in a state where almost nothing is installed. It
has no descriptor tables of its own, no local interrupt controller enabled, and no
per-processor base, and every one of those is needed by the first interrupt it
takes. So enabling interrupts is not a step that can go anywhere in the sequence:
too early and the first interrupt runs a handler with no table to find it and no
per-processor pointer to read, and too late means never, because a processor that
halts with interrupts disabled is not idle, it is gone.

That is the content of this file. `interrupts_need_the_tables` and
`interrupts_need_the_per_cpu_base` are the two lower bounds,
`halting_with_interrupts_off_never_wakes` is the upper one, and
`the_window_is_exactly_one_step` says the three together leave one position in the
sequence. A bring-up sequence is not a list of things to do, it is an order, and
the order is forced.

The last group is about the handshake. A processor that has started is not a
processor that is ready, and the boot processor has to wait for the second fact
rather than the first: `started_is_not_ready` and
`waiting_for_ready_is_what_makes_it_safe` are why the count of live processors is
taken from an acknowledgement and not from a delay.
-/

namespace Nonos.ApBringup

/-! ### What a processor has installed -/

/-- The state of one application processor during bring-up. -/
structure Cpu where
  /-- Its own descriptor and interrupt tables are loaded. -/
  tables : Bool
  /-- Its local interrupt controller is enabled. -/
  lapic : Bool
  /-- Its per-processor base is installed, so a handler can find its own data. -/
  percpu : Bool
  /-- Interrupts are enabled. -/
  interrupts : Bool
  /-- It has acknowledged that it is ready. -/
  ready : Bool
  deriving DecidableEq, Repr

/-- As the processor arrives out of the start-up interrupt: nothing of its own. -/
def arrived : Cpu :=
  ⟨false, false, false, false, false⟩

/-! ### The steps -/

inductive Step where
  | loadTables
  | enableLapic
  | setPerCpu
  | enableInterrupts
  | acknowledge
  | halt
  deriving DecidableEq, Repr

def apply (c : Cpu) : Step → Cpu
  | .loadTables => { c with tables := true }
  | .enableLapic => { c with lapic := true }
  | .setPerCpu => { c with percpu := true }
  | .enableInterrupts => { c with interrupts := true }
  | .acknowledge => { c with ready := true }
  | .halt => c

def run (c : Cpu) : List Step → Cpu
  | [] => c
  | s :: rest => run (apply c s) rest

/-! ### What an interrupt needs -/

/-- A processor can service an interrupt only with its tables, its controller and
    its per-processor base in place. Anything less is a fault inside the fault
    path, which on an application processor with no stack of its own is a
    reset. -/
def CanService (c : Cpu) : Prop :=
  c.tables = true ∧ c.lapic = true ∧ c.percpu = true

/-- Taking an interrupt without them is unrecoverable. -/
def Survives (c : Cpu) : Prop := c.interrupts = true → CanService c

instance (c : Cpu) : Decidable (CanService c) := by
  unfold CanService; infer_instance

instance (c : Cpu) : Decidable (Survives c) := by
  unfold Survives; infer_instance

/-- Every state the processor passes through, including the one it starts in and
    the one it ends in. A constraint on the final state says nothing about a
    window, and the window is the whole of this defect. -/
def trace (c : Cpu) : List Step → List Cpu
  | [] => [c]
  | s :: rest => c :: trace (apply c s) rest

/-- The processor never takes an interrupt it cannot service, at any point in the
    sequence rather than at the end of it. -/
def SurvivesThroughout (c : Cpu) (steps : List Step) : Prop :=
  ∀ s ∈ trace c steps, Survives s

instance (c : Cpu) (steps : List Step) : Decidable (SurvivesThroughout c steps) := by
  unfold SurvivesThroughout; infer_instance

/-- The arrived processor survives, because interrupts are off. Doing nothing is
    safe; it is the first step that can be wrong. -/
theorem arrived_survives : Survives arrived := by
  intro h
  exact absurd h (by simp [arrived])

/-- Enabling interrupts before the tables does not survive. -/
theorem interrupts_need_the_tables :
    ¬ Survives (run arrived [.enableLapic, .setPerCpu, .enableInterrupts]) := by
  intro h
  have := h rfl
  obtain ⟨ht, _, _⟩ := this
  exact Bool.noConfusion ht

/-- Nor before the per-processor base. This is the subtle one: the tables are
    there, so the handler is found, and then it reads through a base that has not
    been set. -/
theorem interrupts_need_the_per_cpu_base :
    ¬ Survives (run arrived [.loadTables, .enableLapic, .enableInterrupts]) := by
  intro h
  have := h rfl
  obtain ⟨_, _, hp⟩ := this
  exact Bool.noConfusion hp

/-- Nor before the controller is enabled. -/
theorem interrupts_need_the_lapic :
    ¬ Survives (run arrived [.loadTables, .setPerCpu, .enableInterrupts]) := by
  intro h
  have := h rfl
  obtain ⟨_, hl, _⟩ := this
  exact Bool.noConfusion hl

/-- With all three installed first, enabling interrupts survives. -/
theorem interrupts_after_everything_survive :
    Survives (run arrived [.loadTables, .enableLapic, .setPerCpu, .enableInterrupts]) := by
  intro _
  exact ⟨rfl, rfl, rfl⟩

/-! ### The other direction -/

/-- A halted processor wakes only if interrupts are enabled. -/
def Wakes (c : Cpu) : Prop := c.interrupts = true

/-- Halting with interrupts disabled is a processor that never runs again. It looks
    like a successful idle: nothing faults, nothing is logged, and the core simply
    does not participate. -/
theorem halting_with_interrupts_off_never_wakes :
    ¬ Wakes (run arrived [.loadTables, .enableLapic, .setPerCpu, .halt]) := by
  intro h
  exact Bool.noConfusion h

/-- Halting with them enabled wakes. -/
theorem halting_with_interrupts_on_wakes :
    Wakes (run arrived [.loadTables, .enableLapic, .setPerCpu, .enableInterrupts, .halt]) := rfl

/-- The two constraints together: interrupts must be enabled after the three
    installations and before the halt, which leaves exactly one position for it in
    the sequence. -/
def theSequence : List Step :=
  [.loadTables, .enableLapic, .setPerCpu, .enableInterrupts, .acknowledge, .halt]

theorem the_window_is_exactly_one_step :
    SurvivesThroughout arrived theSequence ∧ Wakes (run arrived theSequence) := by
  refine ⟨?_, rfl⟩
  decide

/-- Moving the enable one step earlier does not survive. The final state has the
    per-processor base set, so a constraint on the end of the sequence would call
    this correct; the state in the middle is the one that resets the machine. -/
theorem one_step_earlier_does_not_survive :
    ¬ SurvivesThroughout arrived
      [.loadTables, .enableLapic, .enableInterrupts, .setPerCpu, .acknowledge, .halt] := by
  decide

/-- And the final state of that bad order does survive, which is exactly why the
    prefix-wise statement is the one worth having. -/
theorem the_bad_order_looks_correct_at_the_end :
    Survives (run arrived
      [.loadTables, .enableLapic, .enableInterrupts, .setPerCpu, .acknowledge, .halt]) := by
  intro _
  exact ⟨rfl, rfl, rfl⟩

/-- Removing the enable breaks the other constraint. -/
theorem removing_it_does_not_wake :
    ¬ Wakes (run arrived [.loadTables, .enableLapic, .setPerCpu, .acknowledge, .halt]) := by
  intro h
  exact Bool.noConfusion h

/-! ### The handshake -/

/-- A processor that has begun executing our code. -/
def Started (c : Cpu) : Prop := c.tables = true

/-- Started is not ready: the tables are loaded long before the processor is in a
    state anything may be scheduled onto. -/
theorem started_is_not_ready :
    Started (run arrived [.loadTables]) ∧
      ¬ (run arrived [.loadTables]).ready = true := by
  constructor
  · rfl
  · intro h
    exact Bool.noConfusion h

/-- Ready implies the processor got as far as acknowledging, which it does after
    everything else. So waiting for the acknowledgement is waiting for the whole
    sequence, and waiting for a delay is waiting for nothing in particular. -/
theorem waiting_for_ready_is_what_makes_it_safe :
    (run arrived theSequence).ready = true ∧
      SurvivesThroughout arrived theSequence ∧ Wakes (run arrived theSequence) := by
  refine ⟨rfl, ?_, rfl⟩
  decide

/-- A processor counted before it acknowledges is a processor work can be
    scheduled onto before it can service an interrupt. -/
theorem counting_before_the_acknowledgement_is_unsafe :
    ¬ Survives (run arrived [.loadTables, .enableInterrupts]) := by
  intro h
  have := h rfl
  obtain ⟨_, hl, _⟩ := this
  exact Bool.noConfusion hl

end Nonos.ApBringup
