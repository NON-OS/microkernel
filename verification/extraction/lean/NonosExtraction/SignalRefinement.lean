/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Signal delivery policy, on the extracted code.

`Signal.lean` is generated from `src/process/signal/helpers.rs` by Charon and
Aeneas. The property that matters is the one POSIX exists to guarantee and that a
process would very much like to break: `SIGKILL` and `SIGSTOP` cannot be caught,
cannot be ignored and cannot be blocked, so a process cannot make itself
unkillable or unstoppable. `sigkill_is_unmaskable` and `sigstop_is_unmaskable`
say it over the functions the kernel calls.

The three predicates are the same predicate. `can_be_caught`, `can_be_ignored`
and `can_be_blocked` have identical bodies, and `the_three_predicates_are_one`
proves they agree on every signal. That is worth recording rather than tidying:
three names for one policy means a change to one of them is a silent divergence,
and the proof is what would fail if someone changed one and not the others.

Everything here is total. A signal number is a byte and the predicates are
comparisons and matches, so there is no arithmetic and nothing to overflow,
which is the contrast with `VectorsRefinement` where the one arithmetic step is
the one thing that can fail.
-/

import NonosExtraction.Signal

open Aeneas Aeneas.Std Result
open nonos_signal

set_option linter.hashCommand false

namespace NonosExtraction

/-! ### Totality -/

theorem is_valid_signal_is_total (s : Std.U8) :
    ∃ b, process.signal.helpers.is_valid_signal s = ok b := by
  unfold process.signal.helpers.is_valid_signal
  split <;> exact ⟨_, rfl⟩

theorem can_be_caught_is_total (s : Std.U8) :
    ∃ b, process.signal.helpers.can_be_caught s = ok b := by
  unfold process.signal.helpers.can_be_caught
  split <;> exact ⟨_, rfl⟩

theorem is_synchronous_is_total (s : Std.U8) :
    ∃ b, process.signal.helpers.is_synchronous s = ok b := by
  unfold process.signal.helpers.is_synchronous
  split <;> exact ⟨_, rfl⟩

theorem is_fatal_by_default_is_total (s : Std.U8) :
    ∃ b, process.signal.helpers.is_fatal_by_default s = ok b := by
  unfold process.signal.helpers.is_fatal_by_default
  split <;> exact ⟨_, rfl⟩

/-! ### What a process cannot escape -/

/-- `SIGKILL` cannot be caught, ignored or blocked. A process holding no
    capability at all still cannot arrange to survive it, because the policy is
    in the kernel's predicate rather than in the process's disposition table. -/
theorem sigkill_is_unmaskable :
    process.signal.helpers.can_be_caught 9#u8 = ok false ∧
      process.signal.helpers.can_be_ignored 9#u8 = ok false ∧
      process.signal.helpers.can_be_blocked 9#u8 = ok false := by
  unfold process.signal.helpers.can_be_caught process.signal.helpers.can_be_ignored
    process.signal.helpers.can_be_blocked process.signal.constants.SIGKILL
    process.signal.constants.SIGSTOP
  exact ⟨by rfl, by rfl, by rfl⟩

/-- And `SIGSTOP` likewise, so a process cannot make itself unstoppable either.
    The two are separate guarantees: one is about termination and one is about
    scheduling. -/
theorem sigstop_is_unmaskable :
    process.signal.helpers.can_be_caught 19#u8 = ok false ∧
      process.signal.helpers.can_be_ignored 19#u8 = ok false ∧
      process.signal.helpers.can_be_blocked 19#u8 = ok false := by
  unfold process.signal.helpers.can_be_caught process.signal.helpers.can_be_ignored
    process.signal.helpers.can_be_blocked process.signal.constants.SIGKILL
    process.signal.constants.SIGSTOP
  exact ⟨by rfl, by rfl, by rfl⟩

/-- Every other signal can be handled, so the policy is two exceptions rather
    than a general refusal. Shown at the signals a program actually installs
    handlers for. -/
theorem ordinary_signals_can_be_handled :
    process.signal.helpers.can_be_caught 2#u8 = ok true ∧
      process.signal.helpers.can_be_caught 15#u8 = ok true ∧
      process.signal.helpers.can_be_caught 11#u8 = ok true := by
  unfold process.signal.helpers.can_be_caught process.signal.constants.SIGKILL
    process.signal.constants.SIGSTOP
  exact ⟨by rfl, by rfl, by rfl⟩

/-! ### Three names, one policy -/

/-- The three predicates agree on every signal, because they are the same
    expression written three times.

    Keeping this as a theorem rather than collapsing the three into one is the
    point: if someone widens `can_be_blocked` and leaves the others alone, this
    is what stops the build, and the divergence would otherwise be invisible. -/
theorem the_three_predicates_are_one (s : Std.U8) :
    process.signal.helpers.can_be_caught s = process.signal.helpers.can_be_ignored s ∧
      process.signal.helpers.can_be_ignored s = process.signal.helpers.can_be_blocked s :=
  ⟨rfl, rfl⟩

/-- So a signal that can be caught can be blocked, and the converse, which is the
    consequence a caller may rely on while the three stay identical. -/
theorem catchable_iff_blockable (s : Std.U8) (b : Bool)
    (h : process.signal.helpers.can_be_caught s = ok b) :
    process.signal.helpers.can_be_blocked s = ok b := by
  rw [← h]
  exact ((the_three_predicates_are_one s).1.trans (the_three_predicates_are_one s).2).symm

/-! ### The valid range -/

/-- Zero is not a signal, so a zero argument is refused rather than treated as
    the first one. -/
theorem zero_is_not_a_signal :
    process.signal.helpers.is_valid_signal 0#u8 = ok false := by
  unfold process.signal.helpers.is_valid_signal
  rfl

/-- The range runs 1 through 64, and 65 is outside it. -/
theorem valid_range_is_1_to_64 :
    process.signal.helpers.is_valid_signal 1#u8 = ok true ∧
      process.signal.helpers.is_valid_signal 64#u8 = ok true ∧
      process.signal.helpers.is_valid_signal 65#u8 = ok false := by
  unfold process.signal.helpers.is_valid_signal process.signal.constants.SIGRTMAX
  exact ⟨by rfl, by rfl, by rfl⟩

/-- Every real-time signal is a valid signal, so the two ranges are nested rather
    than adjacent. -/
theorem rt_signals_are_valid :
    (process.signal.helpers.is_rt_signal 32#u8 = ok true ∧
      process.signal.helpers.is_valid_signal 32#u8 = ok true) ∧
    (process.signal.helpers.is_rt_signal 64#u8 = ok true ∧
      process.signal.helpers.is_valid_signal 64#u8 = ok true) := by
  unfold process.signal.helpers.is_rt_signal process.signal.helpers.is_valid_signal
    process.signal.constants.SIGRTMIN process.signal.constants.SIGRTMAX
  exact ⟨⟨by rfl, by rfl⟩, ⟨by rfl, by rfl⟩⟩

/-- And the real-time range starts above the standard signals, so a standard
    signal is never treated as queueable. -/
theorem standard_signals_are_not_rt :
    process.signal.helpers.is_rt_signal 31#u8 = ok false ∧
      process.signal.helpers.is_rt_signal 9#u8 = ok false := by
  unfold process.signal.helpers.is_rt_signal process.signal.constants.SIGRTMIN
    process.signal.constants.SIGRTMAX
  exact ⟨by rfl, by rfl⟩

/-! ### Fault signals -/

/-- The synchronous signals are the ones a fault raises on the faulting
    instruction, so they are delivered to the thread that caused them rather than
    to the process. -/
theorem faults_are_synchronous :
    process.signal.helpers.is_synchronous 11#u8 = ok true ∧
      process.signal.helpers.is_synchronous 8#u8 = ok true ∧
      process.signal.helpers.is_synchronous 4#u8 = ok true := ⟨rfl, rfl, rfl⟩

/-- A synchronous signal can still be caught, which is what makes a fault handler
    possible, and is why the unmaskable pair has to be a separate list. -/
theorem synchronous_signals_can_be_caught :
    process.signal.helpers.is_synchronous 11#u8 = ok true ∧
      process.signal.helpers.can_be_caught 11#u8 = ok true := by
  refine ⟨rfl, ?_⟩
  unfold process.signal.helpers.can_be_caught process.signal.constants.SIGKILL
    process.signal.constants.SIGSTOP
  rfl

/-- `SIGKILL` is fatal by default and cannot be caught, so the default and the
    policy agree: there is no disposition under which it does not terminate. -/
theorem sigkill_always_terminates :
    process.signal.helpers.is_fatal_by_default 9#u8 = ok true ∧
      process.signal.helpers.can_be_caught 9#u8 = ok false := by
  refine ⟨rfl, ?_⟩
  unfold process.signal.helpers.can_be_caught process.signal.constants.SIGKILL
    process.signal.constants.SIGSTOP
  rfl

/-- `SIGSTOP` is a stop signal and not fatal, so stopping and terminating are
    distinct outcomes in the table as well as in the policy. -/
theorem sigstop_stops_rather_than_kills :
    process.signal.helpers.is_stop_signal 19#u8 = ok true ∧
      process.signal.helpers.is_fatal_by_default 19#u8 = ok false := ⟨rfl, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.is_valid_signal_is_total
#print axioms NonosExtraction.can_be_caught_is_total
#print axioms NonosExtraction.is_synchronous_is_total
#print axioms NonosExtraction.is_fatal_by_default_is_total
#print axioms NonosExtraction.sigkill_is_unmaskable
#print axioms NonosExtraction.sigstop_is_unmaskable
#print axioms NonosExtraction.ordinary_signals_can_be_handled
#print axioms NonosExtraction.the_three_predicates_are_one
#print axioms NonosExtraction.catchable_iff_blockable
#print axioms NonosExtraction.zero_is_not_a_signal
#print axioms NonosExtraction.valid_range_is_1_to_64
#print axioms NonosExtraction.rt_signals_are_valid
#print axioms NonosExtraction.faults_are_synchronous
#print axioms NonosExtraction.sigkill_always_terminates
#print axioms NonosExtraction.sigstop_stops_rather_than_kills

end NonosExtraction
