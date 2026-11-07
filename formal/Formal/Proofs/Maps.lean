/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

How each state update reads back: which mapping a lookup can return after a
map, an unmap, a surface release or a teardown.
-/

import Formal.Step

namespace Formal

theorem mapAt_some {s : State} {p : Pid} {va : VA} {mo : Option Mapping} {q : Pid} {va' : VA}
    {m : Mapping} (h : s.mapAt p va mo q va' = some m) :
    (q = p ∧ va' = va ∧ mo = some m) ∨ (¬(q = p ∧ va' = va) ∧ s.maps q va' = some m) := by
  unfold State.mapAt at h
  by_cases hq : q = p
  · subst hq
    rw [upd_same] at h
    by_cases hv : va' = va
    · subst hv; rw [upd_same] at h; exact Or.inl ⟨rfl, rfl, h⟩
    · rw [upd_other _ _ hv] at h
      exact Or.inr ⟨fun h' => hv h'.2, h⟩
  · rw [upd_other _ _ hq] at h
    exact Or.inr ⟨fun h' => hq h'.1, h⟩

theorem unmapSurface_some {s : State} {p : Pid} {sid : Nat} {q : Pid} {va : VA} {m : Mapping}
    (h : s.unmapSurface p sid q va = some m) :
    s.maps q va = some m ∧ (q = p → s.frames m.frame ≠ .surface sid) := by
  unfold State.unmapSurface at h
  by_cases hq : q = p
  · subst hq
    rw [upd_same] at h
    cases hm : s.maps q va with
    | none => rw [hm] at h; simp at h
    | some m' =>
        rw [hm] at h
        simp only at h
        by_cases hf : s.frames m'.frame = .surface sid
        · rw [if_pos hf] at h; simp at h
        · rw [if_neg hf] at h
          cases h
          exact ⟨rfl, fun _ => hf⟩
  · rw [upd_other _ _ hq] at h
    exact ⟨h, fun h' => absurd h' hq⟩

theorem teardown_maps_some {s : State} {p q : Pid} {va : VA} {m : Mapping}
    (h : (s.teardown p).maps q va = some m) : q ≠ p ∧ s.maps q va = some m := by
  simp only [State.teardown] at h
  by_cases hq : q = p
  · subst hq; rw [upd_same] at h; simp at h
  · rw [upd_other _ _ hq] at h; exact ⟨hq, h⟩

end Formal
