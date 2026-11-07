/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The grant bound. Under the shipped rule a grant must lie inside the
target's envelope, recorded once at spawn, so a capsule's word never leaves
the envelope, and the envelope is inside what its manifest declared under
the image ceiling.
-/

import Formal.Proofs.Capabilities

namespace Formal

theorem within_and {a b c : Caps} (hb : Within a b) (hc : Within a c) : Within a (b &&& c) :=
  (within_iff _ _).2 fun i h => by
    rw [has_and, has_of_within hb h, has_of_within hc h]; rfl

/-- What the manifest declares, under the image ceiling. -/
def declared (P : Platform) (r : Request) : Caps :=
  (r.manifest.required ||| r.manifest.optional) &&& P.imageCeiling

/-- The installed word is inside the declared bound. -/
theorem admitted_within_declared {P : Platform} {r : Request} {w : Caps}
    (h : admission P r = some w) : Within w (declared P r) := by
  obtain ⟨_, _, _, _, h5, _, rfl⟩ := admission_spec h
  have hman : Within r.install (r.manifest.required ||| r.manifest.optional) :=
    within_or_mono (within_refl _) (within_and_left _ _)
  exact within_trans (within_and_left _ _) (within_and hman h5)

/-- The recorded envelope is inside the declared bound. -/
theorem envelope_within_declared {P : Platform} {r : Request} {w : Caps}
    (h : admission P r = some w) : Within (envelopeOf P r w) (declared P r) :=
  within_or (within_and_left _ _) (admitted_within_declared h)

/-- The envelope invariant: every capsule's word is inside its envelope, and
    the envelope is the one recorded at its spawn. -/
def EnvInv (P : Platform) (s : State) : Prop :=
  ∀ p c, s.caps p = some c → Within c.held c.envelope ∧ c.envelope = envelopeOf P c.request c.admitted

theorem envInv_step {P : Platform} {s s' : State} {op : Op} (hs : EnvInv P s)
    (st : Step P .targetEnvelope s op s') : EnvInv P s' := by
  have frame : ∀ s'', s''.caps = s.caps → EnvInv P s'' := by
    intro s'' h p c hc; rw [h] at hc; exact hs p c hc
  have setc : ∀ (t : Pid) (c : Capsule), Within c.held c.envelope →
      c.envelope = envelopeOf P c.request c.admitted →
      EnvInv P { s with caps := upd s.caps t (some c) } := by
    intro t c h1 h2 p c' h
    by_cases he : p = t
    · subst he; simp only [upd_same, Option.some.injEq] at h; subst h; exact ⟨h1, h2⟩
    · simp only [upd_other _ _ he] at h; exact hs p c' h
  cases st with
  | spawn => exact setc _ _ (within_or_right _ _) rfl
  | load => exact frame _ rfl
  | deny => exact hs
  | protect => exact frame _ rfl
  | @quarantine q mask c hc =>
      obtain ⟨h1, h2⟩ := hs q c hc
      exact setc q (c.revoke q mask) (within_trans (within_excess _ _) h1) h2
  | teardown =>
      rename_i q _ _
      intro p c h
      simp only [State.teardown] at h
      by_cases he : p = q
      · subst he; simp at h
      · simp only [upd_other _ _ he] at h; exact hs p c h
  | @denyKill q _ _ _ _ =>
      intro p c h
      simp only [State.teardown] at h
      by_cases he : p = q
      · subst he; simp at h
      · simp only [upd_other _ _ he] at h; exact hs p c h
  | surfaceFree => exact frame _ rfl
  | call _ _ eff =>
      cases eff with
      | @capGrant t mask _ ct _ _ _ ht hallow =>
          obtain ⟨h1, h2⟩ := hs t ct ht
          exact setc t (ct.grant t mask) (within_or h1 hallow) h2
      | @capRevoke t mask _ ct _ _ ht =>
          obtain ⟨h1, h2⟩ := hs t ct ht
          exact setc t (ct.revoke t mask) (within_trans (within_excess _ _) h1) h2
      | other => exact hs
      | mmap => exact frame _ rfl
      | munmap => exact frame _ rfl
      | ipcSend => exact frame _ rfl
      | surfaceRegister => exact frame _ rfl
      | surfaceShare => exact frame _ rfl
      | surfaceAttach => exact frame _ rfl
      | surfaceRelease => exact frame _ rfl

theorem envInv_reachable {P : Platform} : ∀ s, Reachable P .targetEnvelope s → EnvInv P s :=
  Reachable.induct (fun p c h => by simp [State.init] at h)
    fun _ _ _ _ hs st => envInv_step hs st

end Formal
