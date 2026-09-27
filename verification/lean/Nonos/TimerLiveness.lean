/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What keeps time when the timer does not.

On the HP the local APIC timer never fired. Nothing crashed. The machine booted,
drew its desktop, and then did not respond to anything, because a scheduler that
only runs at a tick does not run, and a task that only yields at a tick does not
yield. It read as a hang in the input path for a while, which is what a liveness
failure looks like from the outside: not a wrong answer, no answer.

The fix was a second source, the real-time clock at a hundred and twenty-eight
hertz, and the property it provides is the one worth writing down.
`one_source_is_a_single_point_of_failure` says what one source buys. `a_second_live_source_keeps_time`
says what two buy. And `a_derived_source_adds_nothing` says why the second one has
to be a different piece of hardware: a heartbeat computed from the timer's own
count dies exactly when the timer does, and it is very easy to add one by
accident.

The bound at the end is what a periodic source gives beyond liveness. A tick every
period means no task holds the processor for longer than a period, which turns
preemption from something that happens into something with a number attached.
-/

namespace Nonos.TimerLiveness

/-! ### Sources -/

/-- A source of ticks, and whether the hardware is actually delivering them. -/
structure Source where
  /-- Delivering interrupts on this machine. -/
  live : Bool
  /-- How long between ticks, in microseconds. -/
  period : Nat
  deriving DecidableEq, Repr

/-- Time advances if any source ticks. -/
def advances (sources : List Source) : Bool := sources.any (fun s => s.live)

/-- The local APIC timer, on a machine where it does not fire. -/
def deadTimer : Source := ⟨false, 1000⟩

/-- The real-time clock at a hundred and twenty-eight hertz: a period of 7812
    microseconds, rounded down from 7812.5. -/
def rtcHeartbeat : Source := ⟨true, 7812⟩

/-! ### Liveness -/

/-- One source that does not fire is a machine that does not keep time. Nothing
    else in the system has to be wrong. -/
theorem one_source_is_a_single_point_of_failure : advances [deadTimer] = false := rfl

/-- Two sources keep time if either fires, so a dead timer beside a live clock is
    a machine that still runs. -/
theorem a_second_live_source_keeps_time : advances [deadTimer, rtcHeartbeat] = true := rfl

/-- In general: one live source anywhere in the list is enough. -/
theorem any_live_source_suffices (sources : List Source) (s : Source)
    (hmem : s ∈ sources) (hlive : s.live = true) : advances sources = true := by
  unfold advances
  exact List.any_eq_true.mpr ⟨s, hmem, hlive⟩

/-- And time stops only if every source is dead, which is the statement that the
    sources are redundant rather than layered. -/
theorem time_stops_only_if_all_are_dead (sources : List Source)
    (h : advances sources = false) : ∀ s ∈ sources, s.live = false := by
  intro s hmem
  unfold advances at h
  by_cases hs : s.live
  · exfalso
    rw [List.any_eq_true.mpr ⟨s, hmem, hs⟩] at h
    exact Bool.noConfusion h
  · simp at hs
    exact hs

/-- No sources at all is the same as all of them dead. An empty list is the case
    that is easy to forget and produces the identical machine. -/
theorem no_sources_is_no_time : advances [] = false := rfl

/-! ### Independence -/

/-- A second source computed from the first: a heartbeat that counts the timer's
    own interrupts, or a watchdog armed by the timer's handler. -/
def derived (s : Source) : Source := ⟨s.live, s.period⟩

/-- It adds nothing. A machine with a source and its derivative advances exactly
    when the source does, so the redundancy is on paper only. -/
theorem a_derived_source_adds_nothing (s : Source) :
    advances [s, derived s] = advances [s] := by
  unfold advances derived
  simp

/-- On the machine where it mattered: the timer is dead, so its derivative is dead,
    so the pair is dead. -/
theorem a_derived_heartbeat_does_not_save_the_hp :
    advances [deadTimer, derived deadTimer] = false := rfl

/-- Independence is the property that makes a second source worth adding: the two
    are not live together and not dead together by construction. -/
def Independent (a b : Source) : Prop := a.live = false → b.live = true

/-- An independent second source keeps time whenever the first fails, which is
    exactly what redundancy has to mean here. -/
theorem an_independent_source_covers_the_failure (a b : Source)
    (h : Independent a b) (hdead : a.live = false) : advances [a, b] = true := by
  unfold advances
  simp [h hdead]

/-- The clock is independent of the timer on this machine, which is the fact the
    fix rests on and the fact that had to be measured rather than assumed. -/
theorem the_clock_is_independent_of_the_timer : Independent deadTimer rtcHeartbeat := by
  intro _
  rfl

/-- A derived source is never independent of what it was derived from. -/
theorem derived_is_never_independent (s : Source) (h : s.live = false) :
    ¬ Independent s (derived s) := by
  intro hi
  have := hi h
  unfold derived at this
  simp only at this
  rw [h] at this
  exact Bool.noConfusion this

/-! ### What a period buys -/

/-- The longest a task can hold the processor: until the next tick. -/
def maxHold (s : Source) : Nat := s.period

/-- A hundred and twenty-eight hertz bounds it at 7812 microseconds, under eight
    milliseconds, which is below the interval a person notices. -/
theorem the_heartbeat_bounds_the_stall : maxHold rtcHeartbeat = 7812 := rfl

theorem the_bound_is_under_ten_milliseconds : maxHold rtcHeartbeat < 10000 := by
  show (7812 : Nat) < 10000
  omega

/-- A dead source bounds nothing: there is no next tick, so a compute-bound task
    holds the processor for as long as it likes. This is the difference between a
    slow machine and an unresponsive one. -/
def Bounded (sources : List Source) (limit : Nat) : Prop :=
  ∃ s ∈ sources, s.live = true ∧ s.period ≤ limit

theorem a_dead_source_bounds_nothing (limit : Nat) : ¬ Bounded [deadTimer] limit := by
  intro ⟨s, hmem, hlive, _⟩
  simp at hmem
  subst hmem
  exact Bool.noConfusion hlive

/-- And a live periodic source does, at its own period. -/
theorem the_heartbeat_bounds_preemption : Bounded [deadTimer, rtcHeartbeat] 7812 := by
  refine ⟨rtcHeartbeat, ?_, rfl, ?_⟩
  · simp
  · show (7812 : Nat) ≤ 7812
    omega

/-- Adding a faster source tightens the bound; it never loosens it, so more
    sources is monotone in the right direction. -/
theorem more_sources_never_loosen_the_bound (sources : List Source) (s : Source)
    (limit : Nat) (h : Bounded sources limit) : Bounded (s :: sources) limit := by
  obtain ⟨t, hmem, hlive, hper⟩ := h
  exact ⟨t, List.mem_cons_of_mem s hmem, hlive, hper⟩

end Nonos.TimerLiveness
