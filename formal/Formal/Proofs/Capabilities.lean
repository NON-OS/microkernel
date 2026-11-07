/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Capability bounds and admission evidence. Every live capsule keeps the
request it was admitted on, and that request passes the admission decision
with the word it was given; its token is its own, minted under its current
epoch over exactly the word it holds; what it holds is what it was admitted
with plus what `sys_cap_grant` has added; and all of it stays under the image
ceiling.
-/

import Formal.Proofs.Bridge
import Formal.Step

namespace Formal

/-- The invariant on every live capsule. -/
def CapInv (P : Platform) (s : State) : Prop :=
  ∀ p c, s.caps p = some c →
    c.token = mint p c.epoch c.held ∧
    Within c.held (c.admitted ||| c.received) ∧
    Within c.admitted P.imageCeiling ∧
    Within c.received P.imageCeiling ∧
    admission P c.request = some c.admitted

/-- What an accepted admission establishes. -/
theorem admission_spec {P : Platform} {r : Request} {w : Caps} (h : admission P r = some w) :
    r.structural = true ∧
    Within (r.manifest.required ||| r.manifest.optional) r.cert.ceiling ∧
    sigsOk P r.manifest r.cert = true ∧
    Within r.granted (r.manifest.required ||| r.manifest.optional) ∧
    Within r.install P.imageCeiling ∧
    r.attested = true ∧
    w = r.install &&& P.profileKeep := by
  unfold admission at h
  split at h
  · rename_i hc
    simp only [Bool.and_eq_true, withinCeiling, grantWithinManifest, decide_eq_true_eq] at hc
    obtain ⟨⟨⟨⟨⟨h1, h2⟩, h3⟩, h4⟩, h5⟩, h6⟩ := hc
    exact ⟨h1, h2, h3, h4, h5, h6, (Option.some.inj h).symm⟩
  · cases h

theorem admission_within_image {P : Platform} {r : Request} {w : Caps} (h : admission P r = some w) :
    Within w P.imageCeiling := by
  obtain ⟨_, _, _, _, h5, _, rfl⟩ := admission_spec h
  exact within_trans (within_and_left _ _) h5

theorem admission_within_cert {P : Platform} {r : Request} {w : Caps} (h : admission P r = some w) :
    Within w r.cert.ceiling := by
  obtain ⟨_, h2, _, _, _, _, rfl⟩ := admission_spec h
  exact within_trans (within_and_left _ _) (install_within_cert h2)

theorem capInv_init (P : Platform) : CapInv P State.init := by
  intro p c h
  simp [State.init] at h

/-- The invariant reads only the capsule table. -/
theorem CapInv.congr {P : Platform} {s s' : State} (hs : CapInv P s) (h : s'.caps = s.caps) :
    CapInv P s' := by
  intro p c hc
  rw [h] at hc
  exact hs p c hc

theorem capInv_set {P : Platform} {s : State} (hs : CapInv P s) {t : Pid} {c : Capsule}
    (hc : c.token = mint t c.epoch c.held ∧ Within c.held (c.admitted ||| c.received) ∧
      Within c.admitted P.imageCeiling ∧ Within c.received P.imageCeiling ∧
      admission P c.request = some c.admitted) :
    CapInv P { s with caps := upd s.caps t (some c) } := by
  intro p c' h
  by_cases he : p = t
  · subst he
    simp only [upd_same, Option.some.injEq] at h
    subst h
    exact hc
  · simp only [upd_other _ _ he] at h
    exact hs p c' h

theorem capInv_revoke {P : Platform} {s : State} (hs : CapInv P s) {t : Pid} {c : Capsule}
    (hc : s.caps t = some c) (mask : Caps) :
    CapInv P { s with caps := upd s.caps t (some (c.revoke t mask)) } := by
  obtain ⟨_, h2, h3, h4, h5⟩ := hs t c hc
  exact capInv_set hs ⟨rfl, within_trans (within_excess _ _) h2, h3, h4, h5⟩

theorem capInv_step {P : Platform} {g : GrantRule} {s s' : State} {op : Op} (hs : CapInv P s)
    (st : Step P g s op s') : CapInv P s' := by
  cases st with
  | spawn _ hw =>
      exact capInv_set hs ⟨rfl, within_or_left _ _, admission_within_image hw, within_zero _, hw⟩
  | load => exact hs.congr rfl
  | deny => exact hs
  | protect => exact hs.congr rfl
  | quarantine hc => exact capInv_revoke hs hc _
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
  | surfaceFree => exact hs.congr rfl
  | call _ _ eff =>
      cases eff with
      | other => exact hs
      | @capGrant t mask cp ct hp _ hm ht =>
          obtain ⟨_, p2, _, p4, _⟩ := hs _ cp hp
          obtain ⟨_, t2, t3, t4, t5⟩ := hs t ct ht
          -- What the caller hands over is under the ceiling, since it holds it.
          have hmask : Within mask P.imageCeiling :=
            within_trans hm (within_trans p2 (within_or (hs _ cp hp).2.2.1 p4))
          refine capInv_set hs ⟨rfl, ?_, t3, within_or t4 hmask, t5⟩
          show Within (ct.held ||| mask) (ct.admitted ||| (ct.received ||| mask))
          exact within_or
            (within_trans t2 (within_or_mono (within_refl _) (within_or_left _ _)))
            (within_trans (within_or_right ct.received mask) (within_or_right _ _))
      | capRevoke _ _ ht => exact capInv_revoke hs ht _
      | mmap => exact hs.congr rfl
      | munmap => exact hs.congr rfl
      | ipcSend => exact hs.congr rfl
      | surfaceRegister => exact hs.congr rfl
      | surfaceShare => exact hs.congr rfl
      | surfaceAttach => exact hs.congr rfl
      | surfaceRelease => exact hs.congr rfl

theorem capInv_reachable {P : Platform} {g : GrantRule} : ∀ s, Reachable P g s → CapInv P s :=
  Reachable.induct (capInv_init P) fun _ _ _ _ hs st => capInv_step hs st

/-! ### Without Admin in the image, nothing is ever added -/

/-- The image ceiling excludes Admin, so no capsule can hold it. -/
def NoAdmin (P : Platform) : Prop := has P.imageCeiling Data.capAdmin = false

/-- Nothing received, in every live capsule. -/
def NothingReceived (s : State) : Prop := ∀ p c, s.caps p = some c → c.received = 0

theorem nothingReceived_step {P : Platform} {g : GrantRule} (hna : NoAdmin P) {s s' : State} {op : Op}
    (hr : Reachable P g s) (hs : NothingReceived s) (st : Step P g s op s') : NothingReceived s' := by
  have frame : ∀ s'', s''.caps = s.caps → NothingReceived s'' := by
    intro s'' h p c hc; rw [h] at hc; exact hs p c hc
  have setr : ∀ (t : Pid) (c : Capsule), c.received = 0 →
      NothingReceived { s with caps := upd s.caps t (some c) } := by
    intro t c h0 p c' h
    by_cases he : p = t
    · subst he; simp only [upd_same, Option.some.injEq] at h; subst h; exact h0
    · simp only [upd_other _ _ he] at h; exact hs p c' h
  cases st with
  | spawn => exact setr _ _ rfl
  | load => exact frame _ rfl
  | deny => exact hs
  | protect => exact frame _ rfl
  | @quarantine q mask c hc => exact setr q (c.revoke q mask) (hs q c hc)
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
      | @capGrant _ _ cp _ hp hadm _ _ =>
          -- The caller would hold Admin, which the ceiling excludes.
          obtain ⟨_, h2, h3, h4, _⟩ := capInv_reachable _ hr _ cp hp
          have := has_of_within (within_trans h2 (within_or h3 h4)) hadm
          rw [hna] at this
          cases this
      | @capRevoke t mask _ ct _ _ ht => exact setr t (ct.revoke t mask) (hs t ct ht)
      | other => exact hs
      | mmap => exact frame _ rfl
      | munmap => exact frame _ rfl
      | ipcSend => exact frame _ rfl
      | surfaceRegister => exact frame _ rfl
      | surfaceShare => exact frame _ rfl
      | surfaceAttach => exact frame _ rfl
      | surfaceRelease => exact frame _ rfl

theorem nothingReceived_reachable {P : Platform} {g : GrantRule} (hna : NoAdmin P) :
    ∀ s, Reachable P g s → NothingReceived s :=
  Reachable.induct (fun p c h => by simp [State.init] at h)
    fun _ _ _ hr hs st => nothingReceived_step hna hr hs st

end Formal
