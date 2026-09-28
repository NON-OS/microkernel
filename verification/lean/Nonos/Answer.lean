/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What a guest is allowed to observe when a call is answered.

A foreign guest traps, the personality decides, and a value reaches the guest's
result register. Two things must hold and neither was stated anywhere until a
real defect made the omission expensive.

A refusal has to arrive as a refusal. The Linux convention is that a result in
[-4095, -1] is an error and everything else is a value, so a refusal encoded
outside that window is read by the caller as success. A gate that refuses and is
then reported as success is not a gate.

And a call that has not been answered must deliver nothing at all. The kernel
saved every trapped register file in the slot the scheduler resumes from, so a
parked guest could be resumed from its own trap frame. The result register at
trap time holds the syscall number, so mmap answered 9 and mprotect answered 10:
both in the success window, both small, both entirely convincing. Every refusal
the personality made became a success, and the tests that exercised those
refusals passed because everything appeared to work.

`parked_yields_nothing` is the theorem that defect violates. `no_request_in_the_answer`
is the sharper statement of the same thing: what reaches the guest is a function
of the outcome, and the request is not an input to it.
-/

namespace Nonos.Answer

/-- The largest errno the Linux ABI reserves. A result at or below `-MAX_ERRNO`
    is a value rather than an error, which is why an errno may not be encoded
    outside the window. -/
def maxErrno : Nat := 4095

/-- What the personality decided. `parked` is not an outcome the guest can see;
    it is the state of a call whose answer has not been chosen yet. -/
inductive Outcome where
  | served (v : Nat)
  | refused (e : Nat)
  | parked
  deriving DecidableEq, Repr

/-- An errno is in range when it names an error the caller can act on. Zero is
    not an errno: encoded, it would be `-0`, which is `0`, which is success. -/
def validErrno (e : Nat) : Prop := 0 < e ∧ e ≤ maxErrno

instance : DecidablePred validErrno := fun e => by
  unfold validErrno; infer_instance

/-- The result register as the guest reads it, signed. `none` is the absence of
    an answer, which is what a parked call must produce. -/
def observe : Outcome → Option Int
  | .served v  => some (v : Int)
  | .refused e => some (-(e : Int))
  | .parked    => none

/-- The caller's test: everything in the reserved window is an error. -/
def isError (r : Int) : Prop := -(maxErrno : Int) ≤ r ∧ r < 0

instance : DecidablePred isError := fun r => by
  unfold isError; infer_instance

/-! ### A refusal is visible as one -/

/-- Every valid errno encodes into the error window, so a caller that applies
    the standard test sees the refusal. -/
theorem refusal_is_an_error {e : Nat} (h : validErrno e) :
    ∀ r, observe (.refused e) = some r → isError r := by
  intro r hr
  simp only [observe, Option.some.injEq] at hr
  subst hr
  obtain ⟨hpos, hle⟩ := h
  unfold isError maxErrno
  unfold maxErrno at hle
  omega

/-- A served value is never mistaken for an error, so the two cases the caller
    distinguishes really are distinct. -/
theorem served_is_not_an_error (v : Nat) :
    ∀ r, observe (.served v) = some r → ¬ isError r := by
  intro r hr herr
  simp only [observe, Option.some.injEq] at hr
  subst hr
  unfold isError at herr
  omega

/-- The two above, together: what the guest sees answers exactly which case the
    personality chose. This is what makes `isError` a sound test rather than a
    convention everyone happens to follow. -/
theorem error_test_is_faithful {e v : Nat} (h : validErrno e) :
    (∀ r, observe (.refused e) = some r → isError r) ∧
    (∀ r, observe (.served v) = some r → ¬ isError r) :=
  ⟨refusal_is_an_error h, served_is_not_an_error v⟩

/-! ### A parked call answers nothing -/

/-- A call that has not been decided delivers no value.

    The kernel defect violated exactly this: a parked guest resumed from its
    saved trap frame observed the register file as it stood at trap time. -/
theorem parked_yields_nothing : observe .parked = none := rfl

/-- If the guest observed anything, the call was decided. The contrapositive is
    the property the resume path has to preserve: while a call is parked, no
    path may put a value in the result register. -/
theorem observation_implies_decided {o : Outcome} {r : Int}
    (h : observe o = some r) : o ≠ .parked := by
  intro hp
  rw [hp, parked_yields_nothing] at h
  exact Option.noConfusion h

/-! ### The request is not an input to the answer -/

/-- A syscall request. The number is carried in the same register the result
    comes back in, which is the whole reason the defect was possible. -/
structure Request where
  nr   : Nat
  args : List Nat
  deriving Repr

/-- A correct answering path is a function of the outcome alone. Stating it as a
    type says the request cannot reach the result, because it is not in scope. -/
def Answering := Outcome → Option Int

/-- `observe` is such a path. -/
def answering : Answering := observe

/-- Two calls whose outcomes agree are answered identically, whatever they
    requested. A path that let the request through would fail this: two
    different requests refused the same way would be observed differently. -/
theorem answer_ignores_the_request (o : Outcome) :
    ∀ f : Answering, f = answering → ∀ _q : Request, f o = observe o := by
  intro f hf _q
  rw [hf]; rfl

/-- The defect, written down so the theorem above has something to exclude.

    This is what the kernel did: resume a parked guest from its trap frame, so
    the result register still holds the request number. -/
def resumeFromTrapFrame (q : Request) : Option Int := some (q.nr : Int)

/-- Resuming from the trap frame reports success for a call nobody answered,
    for every syscall number, including zero. -/
theorem trap_frame_resume_fakes_success (q : Request) :
    ∀ r, resumeFromTrapFrame q = some r → ¬ isError r := by
  intro r hr herr
  simp only [resumeFromTrapFrame, Option.some.injEq] at hr
  subst hr
  unfold isError at herr
  omega

/-- And it disagrees with the correct answering path on every parked call, which
    is the defect stated as a difference rather than as a story. -/
theorem trap_frame_resume_is_unsound (q : Request) :
    resumeFromTrapFrame q ≠ answering .parked := by
  simp [resumeFromTrapFrame, answering, observe]

/-! ### The two small numbers that made it convincing -/

/-- `mmap` is 9 and `mprotect` is 10 on x86-64. Both encode as successful
    results, which is why a refused mapping looked like an address. Kept as a
    theorem rather than a comment so the shape of the failure stays on record. -/
theorem small_request_numbers_look_like_success :
    (∀ r, resumeFromTrapFrame ⟨9, []⟩ = some r → ¬ isError r) ∧
    (∀ r, resumeFromTrapFrame ⟨10, []⟩ = some r → ¬ isError r) :=
  ⟨trap_frame_resume_fakes_success _, trap_frame_resume_fakes_success _⟩

/-! ### What a gate is worth -/

/-- A gate: it inspects a request and either lets it through or refuses with an
    errno. -/
structure Gate where
  decide : Request → Outcome
  refuses_validly : ∀ q e, decide q = .refused e → validErrno e

/-- A gate's refusal reaches the caller as an error, for every request it
    refuses. Without this a gate can be correct and still useless, which is the
    state the exec gate and the tampered-library check were in: both refused,
    neither refusal arrived. -/
theorem gate_refusals_reach_the_caller (g : Gate) (q : Request) (e : Nat)
    (h : g.decide q = .refused e) :
    ∀ r, observe (g.decide q) = some r → isError r := by
  rw [h]
  exact refusal_is_an_error (g.refuses_validly q e h)

/-- A gate answered through the trap frame refuses nothing, whatever it decided.
    This is the theorem that the boot on runF1 demonstrated by experiment. -/
theorem gate_is_void_when_answered_from_the_frame (g : Gate) (q : Request) (e : Nat)
    (_ : g.decide q = .refused e) :
    ∀ r, resumeFromTrapFrame q = some r → ¬ isError r :=
  trap_frame_resume_fakes_success q

end Nonos.Answer
