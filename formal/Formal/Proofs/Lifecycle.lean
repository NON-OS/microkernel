/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Process lifetime. A pid with no live capsule owns nothing: no mapping, no
private frame, no surface view or right, no queued message. So teardown
gives everything back, and a pid that is reused starts from nothing the old
holder left behind.
-/

import Formal.Proofs.Isolation

namespace Formal

/-- What a pid with no capsule may not hold. -/
def DeadHoldsNothing (s : State) : Prop :=
  ∀ p, s.caps p = none →
    (∀ va, s.maps p va = none) ∧
    (∀ f, s.frames f ≠ .proc p) ∧
    (∀ sid sf, s.surfaces sid = some sf → p ∉ sf.attached ∧ ∀ e ∈ sf.named, e.1 ≠ p) ∧
    s.inbox p = []

theorem dead_init : DeadHoldsNothing State.init := by
  intro p _
  refine ⟨fun _ => rfl, fun _ h => (by cases h), fun sid sf h => (by simp [State.init] at h), rfl⟩

theorem drop_named_ne (p : Pid) (sf : Surface) {e : Pid × Bool} (h : e ∈ (sf.drop p).named) :
    e.1 ≠ p := by
  unfold Surface.drop at h
  split at h
  · simp at h
  · have := (List.mem_filter.1 h).2
    simpa using this

/-- Teardown leaves `p` holding nothing, whatever the state was. -/
theorem teardown_releases (s : State) (p : Pid) :
    (s.teardown p).caps p = none ∧
    (∀ va, (s.teardown p).maps p va = none) ∧
    (∀ f, (s.teardown p).frames f ≠ .proc p) ∧
    (∀ sid sf, (s.teardown p).surfaces sid = some sf → p ∉ sf.attached ∧ ∀ e ∈ sf.named, e.1 ≠ p) ∧
    (s.teardown p).inbox p = [] := by
  refine ⟨by simp [State.teardown], fun va => by simp [State.teardown], fun f => ?_, ?_, by
    simp [State.teardown]⟩
  · simp only [State.teardown]
    split
    · intro h; cases h
    · assumption
  · intro sid sf h
    simp only [State.teardown, option_map_eq_some] at h
    obtain ⟨sf0, _, rfl⟩ := h
    refine ⟨?_, fun e he => drop_named_ne p sf0 he⟩
    rw [drop_attached, List.mem_filter]
    simp

theorem dead_teardown {s : State} (hs : DeadHoldsNothing s) (p : Pid) :
    DeadHoldsNothing (s.teardown p) := by
  intro q hq
  by_cases he : q = p
  · subst he; exact (teardown_releases s q).2
  · have hq' : s.caps q = none := by simpa [State.teardown, upd_other _ _ he] using hq
    obtain ⟨h1, h2, h3, h4⟩ := hs q hq'
    refine ⟨fun va => by simp [State.teardown, upd_other _ _ he, h1], fun f => ?_, ?_, by
      simp [State.teardown, upd_other _ _ he, h4]⟩
    · simp only [State.teardown]
      split
      · intro h; cases h
      · exact h2 f
    · intro sid sf h
      simp only [State.teardown, option_map_eq_some] at h
      obtain ⟨sf0, h0, rfl⟩ := h
      obtain ⟨ha, hn⟩ := h3 sid sf0 h0
      refine ⟨?_, fun e hm => hn e (drop_named p sf0 hm)⟩
      rw [drop_attached, List.mem_filter]
      exact fun h => ha h.1

theorem dead_step {P : Platform} {g : GrantRule} {s s' : State} {op : Op}
    (hs : DeadHoldsNothing s) (st : Step P g s op s') : DeadHoldsNothing s' := by
  cases st with
  | @spawn p r w hn _ =>
      intro q hq
      by_cases he : q = p
      · subst he; simp at hq
      · exact hs q (by simpa [upd_other _ _ he] using hq)
  | @load p va f perm c hc hf _ _ =>
      intro q hq
      have hne : q ≠ p := by intro h; subst h; rw [hc] at hq; cases hq
      obtain ⟨h1, h2, h3, h4⟩ := hs q hq
      refine ⟨fun v => ?_, fun f' => ?_, h3, h4⟩
      · simp [State.mapAt, upd_other _ _ hne, h1]
      · by_cases hf' : f' = f
        · subst hf'; simp only [upd_same]; intro h; cases h; exact hne rfl
        · simp only [upd_other _ _ hf']; exact h2 f'
  | deny => exact hs
  | @denyKill p _ _ _ _ => exact dead_teardown hs p
  | @protect p va perm m hm _ =>
      intro q hq
      have hne : q ≠ p := by intro h; subst h; rw [(hs q hq).1 va] at hm; cases hm
      obtain ⟨h1, h2, h3, h4⟩ := hs q hq
      exact ⟨fun v => by simp [State.mapAt, upd_other _ _ hne, h1], h2, h3, h4⟩
  | @quarantine p mask c hc =>
      intro q hq
      have hne : q ≠ p := by intro h; subst h; simp at hq
      exact hs q (by simpa [upd_other _ _ hne] using hq)
  | @teardown p _ _ => exact dead_teardown hs p
  | @surfaceFree sid sf _ _ _ =>
      intro q hq
      obtain ⟨h1, h2, h3, h4⟩ := hs q hq
      refine ⟨h1, fun f => ?_, fun sid' sf' h => ?_, h4⟩
      · simp only
        split
        · intro h; cases h
        · exact h2 f
      · by_cases he : sid' = sid
        · subst he; simp at h
        · exact h3 sid' sf' (by simpa [upd_other _ _ he] using h)
  | @call s'' p nr h hg _ eff =>
      -- The caller passed the gate, so it is alive.
      obtain ⟨cp, hcp, _, _⟩ := gate_some hg
      have alive : ∀ q, s.caps q = none → q ≠ p := by
        intro q hq he; subst he; rw [hcp] at hq; cases hq
      cases eff with
      | other => exact hs
      | @capGrant t mask _ ct _ _ _ ht _ =>
          intro q hq
          have hne : q ≠ t := by intro h; subst h; simp at hq
          exact hs q (by simpa [upd_other _ _ hne] using hq)
      | @capRevoke t mask _ ct _ _ ht =>
          intro q hq
          have hne : q ≠ t := by intro h; subst h; simp at hq
          exact hs q (by simpa [upd_other _ _ hne] using hq)
      | @mmap va f perm _ _ _ =>
          intro q hq
          have hne := alive q hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          refine ⟨fun v => ?_, fun f' => ?_, h3, h4⟩
          · simp [State.mapAt, upd_other _ _ hne, h1]
          · by_cases hf' : f' = f
            · subst hf'; simp only [upd_same]; intro h; cases h; exact hne rfl
            · simp only [upd_other _ _ hf']; exact h2 f'
      | @munmap va m _ _ =>
          intro q hq
          have hne := alive q hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          refine ⟨fun v => ?_, fun f' => ?_, h3, h4⟩
          · simp [State.mapAt, upd_other _ _ hne, h1]
          · by_cases hf' : f' = m.frame
            · subst hf'; simp only [upd_same]; intro h; cases h
            · simp only [upd_other _ _ hf']; exact h2 f'
      | @ipcSend dst msg c hd =>
          intro q hq
          have hne : q ≠ dst := by intro h; subst h; rw [hd] at hq; cases hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          exact ⟨h1, h2, h3, by simp [upd_other _ _ hne, h4]⟩
      | @surfaceRegister sid fs hn hfree =>
          intro q hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          refine ⟨h1, fun f => ?_, fun sid' sf' h => ?_, h4⟩
          · simp only
            split
            · intro h; cases h
            · exact h2 f
          · by_cases he : sid' = sid
            · subst he
              simp only [upd_same, Option.some.injEq] at h
              subst h
              exact ⟨by simp, by simp⟩
            · exact h3 sid' sf' (by simpa [upd_other _ _ he] using h)
      | @surfaceShare sid r w sf c hsf _ _ hrc _ =>
          intro q hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          have hne : q ≠ r := by intro h; subst h; rw [hrc] at hq; cases hq
          refine ⟨h1, h2, fun sid' sf' h => ?_, h4⟩
          by_cases he : sid' = sid
          · subst he
            simp only [upd_same, Option.some.injEq] at h
            subst h
            obtain ⟨ha, hn⟩ := h3 _ sf hsf
            refine ⟨ha, fun e hm => ?_⟩
            rcases List.mem_cons.1 hm with h | h
            · subst h; exact hne.symm
            · exact hn e h
          · exact h3 sid' sf' (by simpa [upd_other _ _ he] using h)
      | @surfaceAttach sid va f sf hsf _ _ _ _ =>
          intro q hq
          have hne := alive q hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          refine ⟨fun v => by simp [State.mapAt, upd_other _ _ hne, h1], h2,
            fun sid' sf' h => ?_, h4⟩
          by_cases he : sid' = sid
          · subst he
            simp only [upd_same, Option.some.injEq] at h
            subst h
            obtain ⟨ha, hn⟩ := h3 _ sf hsf
            refine ⟨fun hm => ?_, hn⟩
            rcases List.mem_cons.1 hm with h | h
            · exact hne h
            · exact ha h
          · exact h3 sid' sf' (by simpa [upd_other _ _ he] using h)
      | @surfaceRelease sid sf hsf =>
          intro q hq
          have hne := alive q hq
          obtain ⟨h1, h2, h3, h4⟩ := hs q hq
          refine ⟨fun v => by simp [State.unmapSurface, upd_other _ _ hne, h1], h2,
            fun sid' sf' h => ?_, h4⟩
          by_cases he : sid' = sid
          · subst he
            simp only [upd_same, Option.some.injEq] at h
            subst h
            obtain ⟨ha, hn⟩ := h3 _ sf hsf
            refine ⟨?_, fun e hm => hn e (releaseBy_named _ sf hm)⟩
            rw [releaseBy_attached, List.mem_filter]
            exact fun h => ha h.1
          · exact h3 sid' sf' (by simpa [upd_other _ _ he] using h)

/-- **A dead pid holds nothing.** In every reachable state, a pid with no
    live capsule has no mapping, owns no private frame, holds no surface view
    and no share right, and has no queued message. Teardown therefore gives
    back everything, and a reused pid inherits nothing. -/
theorem dead_holds_nothing {P : Platform} {g : GrantRule} :
    ∀ s, Reachable P g s → DeadHoldsNothing s :=
  Reachable.induct dead_init fun _ _ _ _ hs st => dead_step hs st

/-- **A capsule starts empty.** The capsule a spawn creates has no
    mappings, no private frames, no surface views or rights and no messages:
    nothing from an earlier holder of its pid reaches it. -/
theorem spawn_starts_empty {P : Platform} {g : GrantRule} {s s' : State} (hr : Reachable P g s)
    {p : Pid} {r : Request} (st : Step P g s (.spawn p r) s') :
    (∀ va, s'.maps p va = none) ∧ (∀ f, s'.frames f ≠ .proc p) ∧
    (∀ sid sf, s'.surfaces sid = some sf → p ∉ sf.attached ∧ ∀ e ∈ sf.named, e.1 ≠ p) ∧
    s'.inbox p = [] := by
  cases st with
  | spawn hn _ => exact dead_holds_nothing s hr p hn

/-- **IPC touches only the receiver's inbox.** A send changes no capability,
    no mapping, no frame, no surface and no other process's inbox. -/
theorem ipc_touches_only_receiver {P : Platform} {g : GrantRule} {s s' : State} {p : Pid}
    {nr : Nat} {dst : Pid} {msg : List Nat} (st : Step P g s (.call p nr (.ipcSend dst msg)) s') :
    s'.caps = s.caps ∧ s'.maps = s.maps ∧ s'.frames = s.frames ∧ s'.surfaces = s.surfaces ∧
      (∀ q, q ≠ dst → s'.inbox q = s.inbox q) ∧ s'.inbox dst = s.inbox dst ++ [msg] := by
  cases st with
  | call _ _ eff =>
      cases eff with
      | ipcSend => exact ⟨rfl, rfl, rfl, rfl, fun q hq => upd_other _ _ hq, upd_same _ _ _⟩

end Formal
