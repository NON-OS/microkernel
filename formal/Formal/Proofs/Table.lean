/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Facts about the shipped syscall table as a whole, checked by Lean over the
data `gen_tables.py` reads out of `src/syscall/contract/cap_table`. Each is a
property of every one of the 185 entries, so a change to one arm that breaks
it fails the build after the tables are regenerated.
-/

import Formal.Proofs.Gate

namespace Formal

/-- Whether a requirement mentions the launcher question. -/
def Req.hasLauncher : Req → Bool
  | .launcher => true
  | .not r => r.hasLauncher
  | .and a b => a.hasLauncher || b.hasLauncher
  | .or a b => a.hasLauncher || b.hasLauncher
  | _ => false

/-- A sufficient syntactic test that a requirement cannot pass without
    capability bit `i`: every way through it reads `has i`. -/
def Req.needs (i : Nat) : Req → Bool
  | .has j => i == j
  | .and a b => a.needs i || b.needs i
  | .or a b => a.needs i && b.needs i
  | _ => false

/-- `needs` is sound: a requirement that needs bit `i` passes only on a word
    holding `i`. -/
theorem Req.needs_sound (i : Nat) (w : Caps) (l : Bool) :
    ∀ r : Req, r.needs i = true → r.eval w l = true → Formal.has w i = true
  | .has j, hn, he => by
      simp only [Req.needs, beq_iff_eq] at hn
      subst hn
      simpa [Req.eval] using he
  | .and a b, hn, he => by
      simp only [Req.needs, Bool.or_eq_true] at hn
      simp only [Req.eval, Bool.and_eq_true] at he
      rcases hn with h | h
      · exact Req.needs_sound i w l a h he.1
      · exact Req.needs_sound i w l b h he.2
  | .or a b, hn, he => by
      simp only [Req.needs, Bool.and_eq_true] at hn
      simp only [Req.eval, Bool.or_eq_true] at he
      rcases he with h | h
      · exact Req.needs_sound i w l a hn.1 h
      · exact Req.needs_sound i w l b hn.2 h
  | .yes, hn, _ => by simp [Req.needs] at hn
  | .valid, hn, _ => by simp [Req.needs] at hn
  | .launcher, hn, _ => by simp [Req.needs] at hn
  | .not _, hn, _ => by simp [Req.needs] at hn

/-- The table checks below, each as one boolean over all entries. -/
def emptyWordOnlyTtyQueryOrRandom : Bool :=
  Data.capTable.all fun e =>
    (!(e.2.eval 0 false) && !(e.2.eval 0 true)) || e.1 == Data.sysMkTtyQuery
      || e.1 == Data.sysCryptoRandom

def launcherOnlyAppLaunch : Bool :=
  Data.capTable.all fun e => !e.2.hasLauncher || e.1 == Data.sysMkAppLaunch

def everyAbiNumberClaimed : Bool :=
  Data.abiSyscalls.all fun a => Data.capTable.any (·.1 == a.1)

set_option maxRecDepth 20000 in
theorem emptyWordOnlyTtyQueryOrRandom_true : emptyWordOnlyTtyQueryOrRandom = true := by decide

set_option maxRecDepth 20000 in
theorem launcherOnlyAppLaunch_true : launcherOnlyAppLaunch = true := by decide

set_option maxRecDepth 20000 in
theorem everyAbiNumberClaimed_true : everyAbiNumberClaimed = true := by decide

/-- **A token with no capability reaches two syscalls.** With the empty
    word, every entry of the shipped table refuses, launcher or not, except
    MkTtyQuery, which asks only about the caller's own streams, and
    CryptoRandom, random bytes, which every process gets (a project decision,
    2026-11-02). -/
theorem empty_word_reaches_only_tty_query_or_random :
    ∀ e ∈ Data.capTable, ∀ l, e.2.eval 0 l = true →
      e.1 = Data.sysMkTtyQuery ∨ e.1 = Data.sysCryptoRandom := by
  intro e he l hev
  have := List.all_eq_true.1 emptyWordOnlyTtyQueryOrRandom_true e he
  simp only [Bool.or_eq_true, Bool.and_eq_true, Bool.not_eq_true', beq_iff_eq] at this
  rcases this with (⟨h0, h1⟩ | h) | h
  · cases l
    · rw [h0] at hev; cases hev
    · rw [h1] at hev; cases hev
  · exact Or.inl h
  · exact Or.inr h

/-- **The launcher question decides one syscall.** Only MkAppLaunch's
    requirement reads `caller_launches()`, so for every other number the
    shell and any other caller with the same word get the same answer. -/
theorem launcher_matters_only_for_app_launch :
    ∀ e ∈ Data.capTable, e.2.hasLauncher = true → e.1 = Data.sysMkAppLaunch := by
  intro e he hl
  have := List.all_eq_true.1 launcherOnlyAppLaunch_true e he
  simp only [hl, Bool.not_true, Bool.false_or, beq_iff_eq] at this
  exact this

/-- A requirement that never reads the launcher evaluates the same for both
    answers. -/
theorem Req.eval_launcher_free (w : Caps) :
    ∀ r : Req, r.hasLauncher = false → ∀ l l', r.eval w l = r.eval w l'
  | .yes, _, _, _ => rfl
  | .valid, _, _, _ => rfl
  | .has _, _, _, _ => rfl
  | .launcher, h, _, _ => by simp [Req.hasLauncher] at h
  | .not r, h, l, l' => by
      simp only [Req.hasLauncher] at h
      simp only [Req.eval, Req.eval_launcher_free w r h l l']
  | .and a b, h, l, l' => by
      simp only [Req.hasLauncher, Bool.or_eq_false_iff] at h
      simp only [Req.eval, Req.eval_launcher_free w a h.1 l l', Req.eval_launcher_free w b h.2 l l']
  | .or a b, h, l, l' => by
      simp only [Req.hasLauncher, Bool.or_eq_false_iff] at h
      simp only [Req.eval, Req.eval_launcher_free w a h.1 l l', Req.eval_launcher_free w b h.2 l l']

/-- **No ABI number falls through the table.** Every number the registry
    resolves is claimed by some group of `cap_table::is_allowed`, so none is
    refused only by the `unwrap_or(false)` default. -/
theorem every_abi_number_is_claimed :
    ∀ a ∈ Data.abiSyscalls, ∃ e ∈ Data.capTable, e.1 = a.1 := by
  intro a ha
  obtain ⟨e, he, heq⟩ := List.any_eq_true.1 (List.all_eq_true.1 everyAbiNumberClaimed_true a ha)
  exact ⟨e, he, by simpa using heq⟩

end Formal
