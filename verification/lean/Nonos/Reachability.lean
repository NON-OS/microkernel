/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

A control nobody calls.

Grepping for functions shaped like a gate, public, returning a decision, and
finding no caller, turned up fifty. An entire isolation module was in that state,
and so was a second W^X check written independently of the one on the mapping
path. Every one of them reads as a control in review. None of them refuses
anything.

This is not the same failure as a control with a bug. A buggy control refuses the
wrong things and can be tested; an unreached control refuses nothing and passes
every test written against it directly. The two theorems that matter are
`removing_an_unreached_control_changes_nothing`, which says the machine with it is
the machine without it, and `dead_controls_grant_everything`, which says fifty of
them compose to no restriction at all.

The duplicate case has its own theorem. Two W^X checks where one is dead are not
two checks: `a_duplicate_with_a_dead_copy_is_one_check` says the pair decides
exactly what the live one decides, so the second copy is worth nothing and the
belief that the property is checked twice is worth less than nothing.

`cap_audit_is_a_ratchet` is the shape the fix takes. The count is recorded, and
the obligation is that it does not grow. A number that may only shrink is
something a build can check; a note saying the list should be worked through is
not.
-/

namespace Nonos.Reachability

/-! ### Controls -/

/-- A request the machine is deciding about. -/
abbrev Request := Nat

/-- A control: the decision it makes, and how many places call it. A control with
    no callers is on no path. -/
structure Control where
  gate : Request → Bool
  callers : Nat

/-- Reached: something calls it. -/
def Reached (c : Control) : Prop := 0 < c.callers

/-- The controls that actually run. -/
def live (cs : List Control) : List Control := cs.filter (fun c => 0 < c.callers)

/-- What the machine decides: every live control has to admit the request. -/
def admits (cs : List Control) (r : Request) : Bool :=
  (live cs).all (fun c => c.gate r)

/-! ### An unreached control is not there -/

/-- Filtering drops it, so it never appears among the controls that run. -/
theorem unreached_is_not_live (cs : List Control) (c : Control) (h : c.callers = 0) :
    live (c :: cs) = live cs := by
  unfold live
  simp [h]

/-- And therefore the machine decides exactly what it would decide without it, on
    every request. Adding the control is not a change to the system. -/
theorem removing_an_unreached_control_changes_nothing (cs : List Control)
    (c : Control) (h : c.callers = 0) : ∀ r, admits (c :: cs) r = admits cs r := by
  intro r
  unfold admits
  rw [unreached_is_not_live cs c h]

/-- A reached control does change the decision: it can refuse. The difference
    between the two is one number, and the number is not in the control's own
    source. -/
theorem a_reached_control_can_refuse (c : Control) (r : Request)
    (hc : Reached c) (hg : c.gate r = false) : admits [c] r = false := by
  unfold admits live
  unfold Reached at hc
  simp [hc, hg]

/-! ### Fifty of them -/

/-- The number the sweep found. -/
def deadControls : Nat := 50

/-- A control that refuses everything and is called from nowhere: the shape all
    fifty were in. -/
def deadControl : Control where
  gate := fun _ => false
  callers := 0

/-- Any number of them, composed. -/
def deadStack (n : Nat) : List Control := List.replicate n deadControl

/-- None of them is live. -/
theorem dead_stack_is_empty (n : Nat) : live (deadStack n) = [] := by
  induction n with
  | zero => rfl
  | succ k ih =>
    unfold deadStack
    rw [List.replicate_succ]
    have h : deadControl.callers = 0 := rfl
    rw [unreached_is_not_live _ deadControl h]
    exact ih

/-- So fifty controls that each refuse everything, together, refuse nothing. The
    strictness of a control has no bearing on what it enforces if nothing calls
    it. -/
theorem dead_controls_grant_everything (r : Request) :
    admits (deadStack deadControls) r = true := by
  unfold admits
  rw [dead_stack_is_empty]
  rfl

/-- Stated for every count, so the result does not depend on the number happening
    to be fifty. -/
theorem any_number_of_dead_controls_grants_everything (n : Nat) (r : Request) :
    admits (deadStack n) r = true := by
  unfold admits
  rw [dead_stack_is_empty]
  rfl

/-! ### A duplicate whose second copy is dead -/

/-- The live check. -/
def liveCheck (g : Request → Bool) : Control := ⟨g, 1⟩

/-- A second, independently written copy, on no path. -/
def deadCopy (g : Request → Bool) : Control := ⟨g, 0⟩

/-- The pair decides what the live one decides. A second W^X check that nothing
    calls does not make the property checked twice, and the belief that it does is
    worse than knowing there is one check, because it justifies not looking at
    the one. -/
theorem a_duplicate_with_a_dead_copy_is_one_check (g g' : Request → Bool)
    (r : Request) : admits [liveCheck g, deadCopy g'] r = admits [liveCheck g] r := by
  unfold admits live liveCheck deadCopy
  simp

/-- And if the live copy is the permissive one, the pair is permissive, whatever
    the dead copy would have refused. -/
theorem the_dead_copy_does_not_tighten (g' : Request → Bool) (r : Request)
    (h : g' r = false) :
    admits [liveCheck (fun _ => true), deadCopy g'] r = true := by
  unfold admits live liveCheck deadCopy
  simp

/-! ### The ratchet -/

/-- What the sweep recorded: how many gate-shaped functions had no caller. -/
structure Audit where
  dead : Nat
  deriving DecidableEq, Repr

/-- One audit is no worse than another when it found no more dead controls. -/
def NoWorse (later earlier : Audit) : Prop := later.dead ≤ earlier.dead

/-- The obligation a build can check: the count does not grow. This is the whole
    mechanism, and it works because it is a number rather than a list of
    intentions. -/
def Ratchet (later earlier : Audit) : Prop := NoWorse later earlier

theorem ratchet_is_reflexive (a : Audit) : Ratchet a a := Nat.le_refl _

theorem ratchet_is_transitive {a b c : Audit} (h1 : Ratchet a b) (h2 : Ratchet b c) :
    Ratchet a c := Nat.le_trans h1 h2

/-- A new dead control breaks the ratchet, which is what makes it a gate on the
    build rather than a report. -/
theorem a_new_dead_control_breaks_the_ratchet (earlier : Audit) :
    ¬ Ratchet ⟨earlier.dead + 1⟩ earlier := by
  unfold Ratchet NoWorse
  simp only
  omega

/-- Wiring one up satisfies it, so the ratchet admits progress as well as
    refusing regress. -/
theorem wiring_one_up_satisfies_the_ratchet (earlier : Audit) (h : 0 < earlier.dead) :
    Ratchet ⟨earlier.dead - 1⟩ earlier := by
  unfold Ratchet NoWorse
  simp only
  omega

/-- The ratchet reaching zero is the end state: no gate-shaped function without a
    caller. Nothing below zero, so the mechanism terminates rather than
    ratcheting forever. -/
theorem zero_is_the_floor (a : Audit) (h : Ratchet a ⟨0⟩) : a.dead = 0 := by
  unfold Ratchet NoWorse at h
  simp only at h
  omega

end Nonos.Reachability
