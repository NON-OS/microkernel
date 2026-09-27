/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Asking a unit for something it does not have.

Bit 11 of a second-level leaf asks the translation unit to keep the access
coherent with the CPU caches. On a unit that reports snoop control in `ECAP.SC`
it means that. On a unit that does not, it is a reserved bit, and an access
through an entry with a reserved bit set does not reach memory: it faults, with
reason 0xC, malformed entry.

So the bit carries a precondition, and the precondition is about the hardware
rather than about the argument. `leaf` cannot check it. It takes a `snoop` flag
and sets the bit when asked, which `NonosExtraction.a_snooped_leaf_sets_bit_eleven`
proves on the extracted encoder. The obligation is the caller's, and this file is
where it is written down.

`unconditionalSnoop` is the shape the identity map had: `leaf(addr, true, true,
true)`, the bit set for every entry it writes, on every unit.
`the_unconditional_request_faults_on_a_unit_without_snoop_control` is what that
costs, and `the_gated_request_never_faults` is the shape that does not: read
`ECAP.SC` once at bring-up and ask for snoop only when the unit said yes.

What is assumed and what is proven. That a reserved bit makes an access fault is
the architecture's rule, not something derivable here, so it is the definition of
`Faults` and named as such. Everything else follows from it. The bit position and
the encoder's behaviour are not assumed: both are theorems over extracted code in
`NonosExtraction.IommuRefinement`.
-/

namespace Nonos.IommuSnoop

/-! ### The unit -/

/-- What the translation unit reported about itself. Only the one capability this
    file is about. -/
structure Unit where
  /-- `ECAP.SC`: the unit honours the snoop bit. -/
  snoopControl : Bool
  deriving DecidableEq, Repr

/-- A unit like the one QEMU presents, which does not report snoop control. -/
def withoutSnoopControl : Unit := ⟨false⟩

/-- A unit that does. -/
def withSnoopControl : Unit := ⟨true⟩

/-! ### The rule

    The architecture's, not this file's. -/

/-- An access through a leaf faults when the leaf asks for snoop and the unit does
    not report snoop control, because the bit is then reserved.

    This is the definition the rest of the file rests on, and it is the one thing
    here that is assumed rather than derived. -/
def Faults (u : Unit) (snoopRequested : Bool) : Prop :=
  snoopRequested = true ∧ u.snoopControl = false

instance (u : Unit) (s : Bool) : Decidable (Faults u s) := by
  unfold Faults; infer_instance

/-- Nothing faults when snoop was not asked for, whatever the unit is. -/
theorem no_request_never_faults (u : Unit) : ¬ Faults u false := by
  intro ⟨h, _⟩
  exact Bool.noConfusion h

/-- And nothing faults on a unit that reports the capability, whatever was
    asked. -/
theorem a_capable_unit_never_faults (s : Bool) : ¬ Faults withSnoopControl s := by
  intro ⟨_, h⟩
  unfold withSnoopControl at h
  exact Bool.noConfusion h

/-! ### What the caller may ask for -/

/-- How a mapping path decides the flag. -/
abbrev Decision := Unit → Bool

/-- The shape the identity map had: ask for snoop always. -/
def unconditionalSnoop : Decision := fun _ => true

/-- The shape that reads the capability first. -/
def gatedSnoop : Decision := fun u => u.snoopControl

/-- A decision is sound when it never produces a faulting entry, on any unit. -/
def Sound (d : Decision) : Prop := ∀ u, ¬ Faults u (d u)

/-! ### The two shapes -/

/-- Asking always faults on a unit without the capability. Every entry the path
    writes is affected, not a corner of them, because the decision does not look
    at anything. -/
theorem the_unconditional_request_faults_on_a_unit_without_snoop_control :
    Faults withoutSnoopControl (unconditionalSnoop withoutSnoopControl) := by
  unfold Faults unconditionalSnoop withoutSnoopControl
  exact ⟨rfl, rfl⟩

/-- So it is not sound. -/
theorem the_unconditional_request_is_unsound : ¬ Sound unconditionalSnoop := by
  intro h
  exact h withoutSnoopControl the_unconditional_request_faults_on_a_unit_without_snoop_control

/-- It is right on a unit that does report the capability, which is why hardware
    with snoop control never showed the fault and the emulated unit did. -/
theorem the_unconditional_request_is_fine_on_capable_hardware :
    ¬ Faults withSnoopControl (unconditionalSnoop withSnoopControl) :=
  a_capable_unit_never_faults _

/-- The gated decision never faults, on either unit. -/
theorem the_gated_request_never_faults : Sound gatedSnoop := by
  intro u
  intro ⟨hs, hc⟩
  unfold gatedSnoop at hs
  rw [hc] at hs
  exact Bool.noConfusion hs

/-- And it still asks for snoop where snoop is available, so the gate costs
    nothing on hardware that has it. A gate that refused always would satisfy
    `Sound` and lose the feature. -/
theorem the_gate_still_asks_where_it_can :
    gatedSnoop withSnoopControl = true := rfl

theorem the_gate_declines_where_it_cannot :
    gatedSnoop withoutSnoopControl = false := rfl

/-- Refusing always is sound and useless, stated so `Sound` is not mistaken for
    the whole requirement. The requirement is `Sound` together with
    `the_gate_still_asks_where_it_can`. -/
def neverSnoop : Decision := fun _ => false

theorem refusing_always_is_sound_and_useless :
    Sound neverSnoop ∧ neverSnoop withSnoopControl = false := by
  refine ⟨?_, rfl⟩
  intro u
  exact no_request_never_faults u

/-! ### Where the capability has to be read

    The gate is only as good as the value it reads, and that value is recorded
    once at bring-up from a register. A decision that reads it before it has been
    recorded sees the default. -/

/-- The recorded capability, before and after bring-up has read the register. -/
inductive Recorded where
  | notYetProbed
  | probed (snoopControl : Bool)
  deriving DecidableEq, Repr

/-- What a decision sees. Before the register is read there is only the default,
    which has to be the conservative one. -/
def observed : Recorded → Bool
  | .notYetProbed => false
  | .probed sc => sc

/-- The default is no snoop, so a mapping written before the unit was probed asks
    for nothing it might not get. An unprobed default of `true` would reintroduce
    the fault for every entry built during bring-up. -/
theorem the_default_is_conservative : observed .notYetProbed = false := rfl

/-- A decision driven by the recorded value is sound on every unit whose
    capability was recorded faithfully. -/
theorem reading_the_record_is_sound (sc : Bool) :
    ¬ Faults ⟨sc⟩ (observed (.probed sc)) := by
  intro ⟨hs, hc⟩
  unfold observed at hs
  simp only at hc
  rw [hc] at hs
  exact Bool.noConfusion hs

/-- And sound before the probe as well, which is what makes the ordering of
    bring-up and table construction a preference rather than a correctness
    condition. -/
theorem reading_the_record_is_sound_before_the_probe (u : Unit) :
    ¬ Faults u (observed .notYetProbed) := by
  rw [the_default_is_conservative]
  exact no_request_never_faults u

end Nonos.IommuSnoop
