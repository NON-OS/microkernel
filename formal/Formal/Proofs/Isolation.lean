/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Memory isolation. A capsule maps a frame only if the frame is its own private
memory, or a surface buffer it holds a view of; a view is taken only by the
surface's owner or by a receiver the project named. Private frames are mapped
once. The surface share is the one sanctioned crossing, and the invariant
states it rather than leaving it out.
-/

import Formal.Proofs.Maps

namespace Formal

/-- `q` may have frame `f` in its address space. -/
def Mine (s : State) (q : Pid) (f : Frame) : Prop :=
  s.frames f = .proc q ∨
    ∃ sid sf, s.frames f = .surface sid ∧ s.surfaces sid = some sf ∧ q ∈ sf.attached

/-- The isolation invariant. -/
structure Iso (s : State) : Prop where
  /-- Every mapped frame is the mapper's own or a surface it attached. -/
  mapped : ∀ q va m, s.maps q va = some m → Mine s q m.frame
  /-- A private frame is mapped at one address only. -/
  unique : ∀ q va va' m m', s.maps q va = some m → s.maps q va' = some m' →
    m.frame = m'.frame → s.frames m.frame = .proc q → va = va'
  /-- A slot's frames belong to the slot. -/
  owned : ∀ sid sf, s.surfaces sid = some sf → ∀ f ∈ sf.frames, s.frames f = .surface sid
  /-- Every view is the project's or a named receiver's. -/
  consent : ∀ sid sf, s.surfaces sid = some sf → ∀ q ∈ sf.attached, q = sf.owner ∨ q ∈ sf.consented
  /-- Every live right was given as a separate step. -/
  named : ∀ sid sf, s.surfaces sid = some sf → ∀ e ∈ sf.named, e.1 ∈ sf.consented

theorem iso_init : Iso State.init where
  mapped := by intro q va m h; simp [State.init] at h
  unique := by intro q va va' m m' h; simp [State.init] at h
  owned := by intro sid sf h; simp [State.init] at h
  consent := by intro sid sf h; simp [State.init] at h
  named := by intro sid sf h; simp [State.init] at h

theorem mine_mono {s s' : State} {q : Pid} {f : Frame} (hf : s'.frames f = s.frames f)
    (hsurf : ∀ sid sf, s.frames f = .surface sid → s.surfaces sid = some sf → q ∈ sf.attached →
      ∃ sf', s'.surfaces sid = some sf' ∧ q ∈ sf'.attached) :
    Mine s q f → Mine s' q f := by
  intro h
  rcases h with h | ⟨sid, sf, hfs, hsf, hq⟩
  · exact Or.inl (hf ▸ h)
  · obtain ⟨sf', h1, h2⟩ := hsurf sid sf hfs hsf hq
    exact Or.inr ⟨sid, sf', hf ▸ hfs, h1, h2⟩

/-- The invariant reads only the mappings, the frame owners and the registry. -/
theorem Iso.congr {s s' : State} (hs : Iso s) (hm : s'.maps = s.maps)
    (hf : s'.frames = s.frames) (hsu : s'.surfaces = s.surfaces) : Iso s' := by
  have e : ∀ q f, Mine s' q f ↔ Mine s q f := by intro q f; simp only [Mine, hf, hsu]
  constructor
  · intro q va m h; rw [hm] at h; exact (e _ _).2 (hs.mapped q va m h)
  · intro q va va' m m' h1 h2 he hp
    rw [hm] at h1 h2; rw [hf] at hp
    exact hs.unique q va va' m m' h1 h2 he hp
  · intro sid sf h; rw [hsu] at h; rw [hf]; exact hs.owned sid sf h
  · intro sid sf h; rw [hsu] at h; exact hs.consent sid sf h
  · intro sid sf h; rw [hsu] at h; exact hs.named sid sf h

/-- No mapping reaches a free frame. -/
theorem Iso.not_free {s : State} (hs : Iso s) {q : Pid} {va : VA} {m : Mapping}
    (h : s.maps q va = some m) : s.frames m.frame ≠ .free := by
  intro hf
  rcases hs.mapped q va m h with h' | ⟨_, _, h', _, _⟩ <;> rw [hf] at h' <;> cases h'

/-- A fresh frame mapped into `p`: `mmap` and the spawn loads. -/
theorem iso_fresh {s : State} (hs : Iso s) {p : Pid} {va : VA} {f : Frame} {perm : Perm}
    (hf : s.frames f = .free) (hv : s.maps p va = none) :
    Iso { s with maps := s.mapAt p va (some ⟨f, perm⟩), frames := upd s.frames f (.proc p) } := by
  have noMap : ∀ q va m, s.maps q va = some m → m.frame ≠ f := by
    intro q va m h he
    exact hs.not_free h (he ▸ hf)
  constructor
  · intro q va' m h
    rcases mapAt_some h with ⟨hq, _, he⟩ | ⟨_, h⟩
    · cases he; subst hq; exact Or.inl (by simp)
    · have hne := noMap _ _ _ h
      refine mine_mono (s := s) ?_ (fun sid sf _ hsf hq => ⟨sf, hsf, hq⟩) (hs.mapped _ _ _ h)
      simp [upd_other _ _ hne]
  · intro q v1 v2 m1 m2 h1 h2 he hp
    rcases mapAt_some h1 with ⟨hq1, hv1, he1⟩ | ⟨hn1, h1⟩ <;>
      rcases mapAt_some h2 with ⟨hq2, hv2, he2⟩ | ⟨hn2, h2⟩
    · rw [hv1, hv2]
    · cases he1; exact absurd he.symm (noMap _ _ _ h2)
    · cases he2; exact absurd he (noMap _ _ _ h1)
    · have hne := noMap _ _ _ h1
      simp only [upd_other _ _ hne] at hp
      exact hs.unique q v1 v2 m1 m2 h1 h2 he hp
  · intro sid sf hsf g hg
    have hgs := hs.owned sid sf hsf g hg
    have hne : g ≠ f := fun he => by rw [he, hf] at hgs; cases hgs
    simp only [upd_other _ _ hne]
    exact hgs
  · exact hs.consent
  · exact hs.named

theorem iso_protect {s : State} (hs : Iso s) {p : Pid} {va : VA} {m : Mapping} {perm : Perm}
    (hm : s.maps p va = some m) :
    Iso { s with maps := s.mapAt p va (some ⟨m.frame, perm⟩) } := by
  have back : ∀ q va' m', s.mapAt p va (some ⟨m.frame, perm⟩) q va' = some m' →
      ∃ m'', s.maps q va' = some m'' ∧ m''.frame = m'.frame := by
    intro q va' m' h
    rcases mapAt_some h with ⟨hq, hv, he⟩ | ⟨_, h⟩
    · cases he; subst hq; subst hv; exact ⟨m, hm, rfl⟩
    · exact ⟨m', h, rfl⟩
  constructor
  · intro q va' m' h
    obtain ⟨m'', h'', hf⟩ := back q va' m' h
    exact hf ▸ hs.mapped _ _ _ h''
  · intro q v1 v2 m1 m2 h1 h2 he hp
    obtain ⟨n1, h1', hf1⟩ := back _ _ _ h1
    obtain ⟨n2, h2', hf2⟩ := back _ _ _ h2
    exact hs.unique q v1 v2 n1 n2 h1' h2' (by rw [hf1, hf2, he]) (by rw [hf1]; exact hp)
  · exact hs.owned
  · exact hs.consent
  · exact hs.named

theorem iso_munmap {s : State} (hs : Iso s) {p : Pid} {va : VA} {m : Mapping}
    (hm : s.maps p va = some m) (hp : s.frames m.frame = .proc p) :
    Iso { s with maps := s.mapAt p va none, frames := upd s.frames m.frame .free } := by
  constructor
  · intro q va' m' h
    rcases mapAt_some h with ⟨_, _, he⟩ | ⟨hn, h⟩
    · cases he
    · by_cases hf : m'.frame = m.frame
      · exfalso
        rcases hs.mapped _ _ _ h with h' | ⟨_, _, h', _, _⟩
        · rw [hf, hp] at h'
          cases h'
          exact hn ⟨rfl, hs.unique p va' va m' m h hm hf (by rw [hf]; exact hp)⟩
        · rw [hf, hp] at h'; cases h'
      · refine mine_mono (s := s) ?_ (fun sid sf _ hsf hq => ⟨sf, hsf, hq⟩) (hs.mapped _ _ _ h)
        simp [upd_other _ _ hf]
  · intro q v1 v2 m1 m2 h1 h2 he hq
    rcases mapAt_some h1 with ⟨_, _, he1⟩ | ⟨_, h1⟩
    · cases he1
    rcases mapAt_some h2 with ⟨_, _, he2⟩ | ⟨_, h2⟩
    · cases he2
    have hne : m1.frame ≠ m.frame := fun h' => by
      simp only [h', upd_same] at hq; cases hq
    simp only [upd_other _ _ hne] at hq
    exact hs.unique q v1 v2 m1 m2 h1 h2 he hq
  · intro sid sf hsf g hg
    have hgs := hs.owned sid sf hsf g hg
    have hne : g ≠ m.frame := fun he => by rw [he, hp] at hgs; cases hgs
    simp only [upd_other _ _ hne]
    exact hgs
  · exact hs.consent
  · exact hs.named

theorem drop_attached (p : Pid) (sf : Surface) :
    (sf.drop p).attached = sf.attached.filter (· != p) := by
  unfold Surface.drop; split <;> rfl

theorem drop_named (p : Pid) (sf : Surface) {e : Pid × Bool} (h : e ∈ (sf.drop p).named) :
    e ∈ sf.named := by
  unfold Surface.drop at h
  split at h
  · simp at h
  · exact (List.mem_filter.1 h).1

theorem drop_rest (p : Pid) (sf : Surface) :
    (sf.drop p).frames = sf.frames ∧ (sf.drop p).owner = sf.owner ∧
      (sf.drop p).consented = sf.consented := by
  unfold Surface.drop; split <;> exact ⟨rfl, rfl, rfl⟩

theorem iso_teardown {s : State} (hs : Iso s) (p : Pid) : Iso (s.teardown p) := by
  have surf : ∀ sid sf', (s.teardown p).surfaces sid = some sf' →
      ∃ sf, s.surfaces sid = some sf ∧ sf' = sf.drop p := by
    intro sid sf' h
    simp only [State.teardown] at h
    cases hx : s.surfaces sid with
    | none => rw [hx] at h; cases h
    | some sf => rw [hx] at h; exact ⟨sf, rfl, (Option.some.inj h).symm⟩
  constructor
  · intro q va m h
    obtain ⟨hq, h⟩ := teardown_maps_some h
    rcases hs.mapped _ _ _ h with h' | ⟨sid, sf, hfs, hsf, hqa⟩
    · left
      simp only [State.teardown, h']
      rw [if_neg (by intro he; cases he; exact hq rfl)]
    · right
      refine ⟨sid, sf.drop p, ?_, ?_, ?_⟩
      · simp only [State.teardown, hfs]; rfl
      · simp only [State.teardown, hsf]; rfl
      · rw [drop_attached, List.mem_filter]
        exact ⟨hqa, by simpa using hq⟩
  · intro q v1 v2 m1 m2 h1 h2 he hp
    obtain ⟨_, h1⟩ := teardown_maps_some h1
    obtain ⟨_, h2⟩ := teardown_maps_some h2
    refine hs.unique q v1 v2 m1 m2 h1 h2 he ?_
    simp only [State.teardown] at hp
    split at hp
    · cases hp
    · exact hp
  · intro sid sf' hsf g hg
    obtain ⟨sf, h1, rfl⟩ := surf sid sf' hsf
    rw [(drop_rest p sf).1] at hg
    have := hs.owned sid sf h1 g hg
    simp only [State.teardown, this]
    rfl
  · intro sid sf' hsf q hq
    obtain ⟨sf, h1, rfl⟩ := surf sid sf' hsf
    rw [drop_attached, List.mem_filter] at hq
    rw [(drop_rest p sf).2.1, (drop_rest p sf).2.2]
    exact hs.consent sid sf h1 q hq.1
  · intro sid sf' hsf e he
    obtain ⟨sf, h1, rfl⟩ := surf sid sf' hsf
    rw [(drop_rest p sf).2.2]
    exact hs.named sid sf h1 e (drop_named p sf he)

theorem iso_surfaceFree {s : State} (hs : Iso s) {sid : Nat} {sf : Surface}
    (hsf : s.surfaces sid = some sf) (hat : sf.attached = []) :
    Iso { s with surfaces := upd s.surfaces sid none,
                 frames := fun f => if s.frames f = .surface sid then .free else s.frames f } := by
  constructor
  · intro q va m h
    rcases hs.mapped _ _ _ h with h' | ⟨sid', sf', hfs, hsf', hqa⟩
    · left; simp only [h']; rfl
    · right
      by_cases he : sid' = sid
      · subst he; rw [hsf] at hsf'; cases hsf'; rw [hat] at hqa; cases hqa
      · refine ⟨sid', sf', ?_, ?_, hqa⟩
        · simp only [hfs]; rw [if_neg (by intro h; cases h; exact he rfl)]
        · simp only [upd_other _ _ he]; exact hsf'
  · intro q v1 v2 m1 m2 h1 h2 he hp
    refine hs.unique q v1 v2 m1 m2 h1 h2 he ?_
    simp only at hp
    split at hp
    · cases hp
    · exact hp
  · intro sid' sf' h g hg
    by_cases he : sid' = sid
    · subst he; simp at h
    · simp only [upd_other _ _ he] at h
      have := hs.owned sid' sf' h g hg
      simp only [this]
      rw [if_neg (by intro h; cases h; exact he rfl)]
  · intro sid' sf' h
    by_cases he : sid' = sid
    · subst he; simp at h
    · simp only [upd_other _ _ he] at h; exact hs.consent sid' sf' h
  · intro sid' sf' h
    by_cases he : sid' = sid
    · subst he; simp at h
    · simp only [upd_other _ _ he] at h; exact hs.named sid' sf' h

theorem iso_register {s : State} (hs : Iso s) {p : Pid} {sid : Nat} {fs : List Frame}
    (hnone : s.surfaces sid = none) (hfree : ∀ f ∈ fs, s.frames f = .free) :
    Iso { s with surfaces := upd s.surfaces sid (some ⟨p, fs, [], [], [], false⟩),
                 frames := fun f => if f ∈ fs then .surface sid else s.frames f } := by
  have notin : ∀ f, s.frames f ≠ .free → f ∉ fs := fun f hf hm => hf (hfree f hm)
  constructor
  · intro q va m h
    have hn := notin _ (hs.not_free h)
    refine mine_mono (s := s) ?_ ?_ (hs.mapped _ _ _ h)
    · simp only [if_neg hn]
    intro sid' sf' _ hsf' hq
    have hne : sid' ≠ sid := fun he => by rw [he, hnone] at hsf'; cases hsf'
    exact ⟨sf', by simp only [upd_other _ _ hne]; exact hsf', hq⟩
  · intro q v1 v2 m1 m2 h1 h2 he hp
    refine hs.unique q v1 v2 m1 m2 h1 h2 he ?_
    simp only at hp
    split at hp
    · cases hp
    · exact hp
  · intro sid' sf' h g hg
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      simp only [if_pos hg]
    · simp only [upd_other _ _ he] at h
      have hgs := hs.owned sid' sf' h g hg
      have hn := notin g (by rw [hgs]; intro h; cases h)
      simp only [if_neg hn]
      exact hgs
  · intro sid' sf' h q hq
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h; cases hq
    · simp only [upd_other _ _ he] at h; exact hs.consent sid' sf' h q hq
  · intro sid' sf' h e hmem
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h; cases hmem
    · simp only [upd_other _ _ he] at h; exact hs.named sid' sf' h e hmem

theorem iso_share {s : State} (hs : Iso s) {sid : Nat} {sf : Surface} {q : Pid} {w : Bool}
    (hsf : s.surfaces sid = some sf) :
    Iso { s with surfaces := upd s.surfaces sid (some (sf.share q w)) } := by
  constructor
  · intro r va m h
    refine mine_mono (s := s) rfl ?_ (hs.mapped _ _ _ h)
    intro sid' sf' _ hsf' hr
    by_cases he : sid' = sid
    · subst he; rw [hsf] at hsf'; cases hsf'
      exact ⟨_, upd_same _ _ _, hr⟩
    · exact ⟨sf', by simp only [upd_other _ _ he]; exact hsf', hr⟩
  · exact hs.unique
  · intro sid' sf' h g hg
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      exact hs.owned _ sf hsf g hg
    · simp only [upd_other _ _ he] at h; exact hs.owned sid' sf' h g hg
  · intro sid' sf' h r hr
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      rcases hs.consent _ sf hsf r hr with h1 | h1
      · exact Or.inl h1
      · exact Or.inr (List.mem_cons_of_mem _ h1)
    · simp only [upd_other _ _ he] at h; exact hs.consent sid' sf' h r hr
  · intro sid' sf' h e hmem
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      rcases List.mem_cons.1 hmem with h1 | h1
      · subst h1; exact List.Mem.head _
      · exact List.mem_cons_of_mem _ (hs.named _ sf hsf e h1)
    · simp only [upd_other _ _ he] at h; exact hs.named sid' sf' h e hmem

theorem iso_attach {s : State} (hs : Iso s) {p : Pid} {sid : Nat} {va : VA} {f : Frame}
    {sf : Surface} {perm : Perm} (hsf : s.surfaces sid = some sf) (hf : f ∈ sf.frames)
    (hok : sf.owner = p ∨ ∃ w, (p, w) ∈ sf.named) :
    Iso { s with maps := s.mapAt p va (some ⟨f, perm⟩),
                 surfaces := upd s.surfaces sid (some (sf.attach p)) } := by
  have hfs := hs.owned sid sf hsf f hf
  have grow : ∀ r, ∀ sid' sf', s.surfaces sid' = some sf' → r ∈ sf'.attached →
      ∃ sf'', upd s.surfaces sid (some (sf.attach p)) sid' = some sf'' ∧
        r ∈ sf''.attached := by
    intro r sid' sf' hsf' hr
    by_cases he : sid' = sid
    · subst he; rw [hsf] at hsf'; cases hsf'
      exact ⟨_, upd_same _ _ _, List.mem_cons_of_mem _ hr⟩
    · exact ⟨sf', by simp only [upd_other _ _ he]; exact hsf', hr⟩
  constructor
  · intro q va' m h
    rcases mapAt_some h with ⟨hq, _, he⟩ | ⟨_, h⟩
    · cases he; subst hq
      exact Or.inr ⟨sid, _, hfs, upd_same _ _ _, List.Mem.head _⟩
    · exact mine_mono (s := s) rfl (fun sid' sf' _ hsf' hr => grow q sid' sf' hsf' hr)
        (hs.mapped _ _ _ h)
  · intro q v1 v2 m1 m2 h1 h2 he hp
    rcases mapAt_some h1 with ⟨_, _, he1⟩ | ⟨_, h1⟩
    · cases he1; simp only at hp; rw [hfs] at hp; cases hp
    rcases mapAt_some h2 with ⟨_, _, he2⟩ | ⟨_, h2⟩
    · cases he2; simp only at he hp; rw [he, hfs] at hp; cases hp
    exact hs.unique q v1 v2 m1 m2 h1 h2 he hp
  · intro sid' sf' h g hg
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      exact hs.owned _ sf hsf g hg
    · simp only [upd_other _ _ he] at h; exact hs.owned sid' sf' h g hg
  · intro sid' sf' h r hr
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      rcases List.mem_cons.1 hr with h1 | h1
      · subst h1
        rcases hok with h2 | ⟨w, h2⟩
        · exact Or.inl h2.symm
        · exact Or.inr (hs.named _ sf hsf (r, w) h2)
      · exact hs.consent _ sf hsf r h1
    · simp only [upd_other _ _ he] at h; exact hs.consent sid' sf' h r hr
  · intro sid' sf' h e hmem
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      exact hs.named _ sf hsf e hmem
    · simp only [upd_other _ _ he] at h; exact hs.named sid' sf' h e hmem

theorem releaseBy_attached (p : Pid) (sf : Surface) :
    (sf.releaseBy p).attached = sf.attached.filter (· != p) := by
  unfold Surface.releaseBy; split <;> rfl

theorem releaseBy_named (p : Pid) (sf : Surface) {e : Pid × Bool} (h : e ∈ (sf.releaseBy p).named) :
    e ∈ sf.named := by
  unfold Surface.releaseBy at h
  split at h
  · simp at h
  · exact h

theorem releaseBy_rest (p : Pid) (sf : Surface) :
    (sf.releaseBy p).frames = sf.frames ∧ (sf.releaseBy p).owner = sf.owner ∧
      (sf.releaseBy p).consented = sf.consented := by
  unfold Surface.releaseBy; split <;> exact ⟨rfl, rfl, rfl⟩

theorem iso_release {s : State} (hs : Iso s) {p : Pid} {sid : Nat} {sf : Surface}
    (hsf : s.surfaces sid = some sf) :
    Iso { s with maps := s.unmapSurface p sid,
                 surfaces := upd s.surfaces sid (some (sf.releaseBy p)) } := by
  constructor
  · intro q va m h
    obtain ⟨h, hnot⟩ := unmapSurface_some h
    rcases hs.mapped _ _ _ h with h' | ⟨sid', sf', hfs, hsf', hqa⟩
    · exact Or.inl h'
    · right
      by_cases he : sid' = sid
      · subst he
        rw [hsf] at hsf'; cases hsf'
        refine ⟨sid', sf.releaseBy p, hfs, upd_same _ _ _, ?_⟩
        rw [releaseBy_attached, List.mem_filter]
        refine ⟨hqa, ?_⟩
        by_cases hq : q = p
        · exact absurd hfs (hnot hq)
        · simpa using hq
      · exact ⟨sid', sf', hfs, by simp only [upd_other _ _ he]; exact hsf', hqa⟩
  · intro q v1 v2 m1 m2 h1 h2 he hp
    exact hs.unique q v1 v2 m1 m2 (unmapSurface_some h1).1 (unmapSurface_some h2).1 he hp
  · intro sid' sf' h g hg
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      rw [(releaseBy_rest p sf).1] at hg
      exact hs.owned _ sf hsf g hg
    · simp only [upd_other _ _ he] at h; exact hs.owned sid' sf' h g hg
  · intro sid' sf' h r hr
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      rw [releaseBy_attached, List.mem_filter] at hr
      rw [(releaseBy_rest p sf).2.1, (releaseBy_rest p sf).2.2]
      exact hs.consent _ sf hsf r hr.1
    · simp only [upd_other _ _ he] at h; exact hs.consent sid' sf' h r hr
  · intro sid' sf' h e hmem
    by_cases he : sid' = sid
    · subst he
      simp only [upd_same, Option.some.injEq] at h
      subst h
      rw [(releaseBy_rest p sf).2.2]
      exact hs.named _ sf hsf e (releaseBy_named p sf hmem)
    · simp only [upd_other _ _ he] at h; exact hs.named sid' sf' h e hmem

theorem iso_step {P : Platform} {g : GrantRule} {s s' : State} {op : Op} (hs : Iso s) (st : Step P g s op s') :
    Iso s' := by
  cases st with
  | spawn => exact hs.congr rfl rfl rfl
  | load _ hf hv _ => exact iso_fresh hs hf hv
  | deny => exact hs
  | denyKill => exact iso_teardown hs _
  | protect hm _ => exact iso_protect hs hm
  | quarantine => exact hs.congr rfl rfl rfl
  | teardown => exact iso_teardown hs _
  | surfaceFree hsf _ hat => exact iso_surfaceFree hs hsf hat
  | call _ _ eff =>
      cases eff with
      | other => exact hs
      | capGrant => exact hs.congr rfl rfl rfl
      | capRevoke => exact hs.congr rfl rfl rfl
      | mmap hf hv _ => exact iso_fresh hs hf hv
      | munmap hm hp => exact iso_munmap hs hm hp
      | ipcSend => exact hs.congr rfl rfl rfl
      | surfaceRegister hn hf => exact iso_register hs hn hf
      | surfaceShare hsf => exact iso_share hs hsf
      | surfaceAttach hsf _ hf _ hok => exact iso_attach hs hsf hf hok
      | surfaceRelease hsf => exact iso_release hs hsf

theorem iso_reachable {P : Platform} {g : GrantRule} : ∀ s, Reachable P g s → Iso s :=
  Reachable.induct iso_init fun _ _ _ _ hs st => iso_step hs st

/-- **Memory isolation.** In every reachable state, two different capsules
    map the same frame only through a surface, and each of them is the
    surface's owner or a receiver the project named. Private memory is never
    shared. -/
theorem shared_frame_is_consented {P : Platform} {g : GrantRule} {s : State} (hr : Reachable P g s)
    {p q : Pid} {va va' : VA} {m m' : Mapping}
    (hp : s.maps p va = some m) (hq : s.maps q va' = some m') (hf : m.frame = m'.frame)
    (hne : p ≠ q) :
    ∃ sid sf, s.frames m.frame = .surface sid ∧ s.surfaces sid = some sf ∧
      (p = sf.owner ∨ p ∈ sf.consented) ∧ (q = sf.owner ∨ q ∈ sf.consented) := by
  have hs := iso_reachable s hr
  have mp := hs.mapped _ _ _ hp
  have mq := hs.mapped _ _ _ hq
  rw [← hf] at mq
  rcases mp with h1 | ⟨sid, sf, h1, h2, h3⟩
  · exfalso
    rcases mq with h4 | ⟨_, _, h4, _, _⟩
    · rw [h1] at h4; cases h4; exact hne rfl
    · rw [h1] at h4; cases h4
  · rcases mq with h4 | ⟨sid', sf', h4, h5, h6⟩
    · rw [h1] at h4; cases h4
    · rw [h1] at h4; cases h4
      rw [h2] at h5; cases h5
      exact ⟨sid, sf, h1, h2, hs.consent sid sf h2 p h3, hs.consent sid sf h2 q h6⟩

/-- A private frame is in one address space at one address. -/
theorem private_frame_unshared {P : Platform} {g : GrantRule} {s : State} (hr : Reachable P g s)
    {p q : Pid} {va va' : VA} {m m' : Mapping}
    (hp : s.maps p va = some m) (hq : s.maps q va' = some m') (hf : m.frame = m'.frame)
    (hpriv : s.frames m.frame = .proc p) : q = p ∧ va' = va := by
  have hs := iso_reachable s hr
  have mq := hs.mapped _ _ _ hq
  rw [← hf] at mq
  rcases mq with h | ⟨_, _, h, _, _⟩
  · rw [hpriv] at h; cases h
    exact ⟨rfl, (hs.unique p va va' m m' hp hq hf hpriv).symm⟩
  · rw [hpriv] at h; cases h

end Formal
