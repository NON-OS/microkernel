/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

What the extracted capability table's rows rest on. A token's list of
capabilities stands for the word with bit `idx c` set for each `c` it holds,
and `Capability::bit` is that bit; `grants` is membership in the list, and
for a token with no expiry `is_valid` is a non-empty list. Each is proved of
the code Aeneas translated, with the library calls defined in
CapTable/FunsExternal.lean.
-/

import CapTable.Funs
import Formal.Gate
open Aeneas Aeneas.Std Result

set_option linter.unusedSimpArgs false

namespace NonosExtraction.CapTable
open nonos_x_cap_table

abbrev Cap := capabilities.types.defs.Capability
abbrev Token := capabilities.token.types.defs.CapabilityToken
abbrev Sys := syscall.numbers.defs.SyscallNumber

deriving instance DecidableEq for capabilities.types.defs.Capability

/-- A capability's place in the word: its position in `capability_table!`. -/
def idx (c : Cap) : Nat := capabilities.types.defs.Capability.ctorIdx c

/-- The word a list of capabilities stands for. -/
def word (ps : List Cap) : Nat := ps.foldr (fun c w => 2 ^ idx c ||| w) 0

/-- The syscall's ABI number, its `tag4` discriminant. -/
def tag (n : Sys) : Nat := (syscall.numbers.defs.SyscallNumber.read_discriminant n).val

set_option maxHeartbeats 4000000 in
theorem disc_inj (a b : Cap) :
    capabilities.types.defs.Capability.read_discriminant a =
      capabilities.types.defs.Capability.read_discriminant b ↔ a = b := by
  constructor
  · intro h
    cases a <;> cases b <;> first | rfl | (simp [capabilities.types.defs.Capability.read_discriminant] at h)
  · intro h; rw [h]

theorem eq_spec (a b : Cap) :
    capabilities.types.defs.Capability.Insts.CoreCmpPartialEqCapability.eq a b = ok (decide (a = b)) := by
  simp [capabilities.types.defs.Capability.Insts.CoreCmpPartialEqCapability.eq, disc_inj]

theorem any_grants (c : Cap) : ∀ (l : List Cap) (n : Nat), ∃ k,
    sliceAny capabilities.token.types.query.CapabilityToken.grants.closure.Insts.CoreOpsFunctionFnMutTupleSharedCapabilityBool
      c l n = ok (decide (c ∈ l), k)
  | [], n => ⟨n, rfl⟩
  | x :: xs, n => by
      by_cases h : x = c
      · subst h
        exact ⟨n + 1, by simp [sliceAny, capabilities.token.types.query.CapabilityToken.grants.closure.Insts.CoreOpsFunctionFnMutTupleSharedCapabilityBool.call_mut, eq_spec]⟩
      · obtain ⟨k, hk⟩ := any_grants c xs (n + 1)
        refine ⟨k, ?_⟩
        have hc : ¬ c = x := fun e => h e.symm
        simp [sliceAny, capabilities.token.types.query.CapabilityToken.grants.closure.Insts.CoreOpsFunctionFnMutTupleSharedCapabilityBool.call_mut, eq_spec, h, hk, hc]

theorem grants_spec (t : Token) (c : Cap) :
    capabilities.token.types.query.CapabilityToken.grants t c = ok (decide (c ∈ t.permissions.val)) := by
  obtain ⟨k, hk⟩ := any_grants c t.permissions.val 0
  simp [capabilities.token.types.query.CapabilityToken.grants, core.slice.Slice.iter,
    core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.any, alloc.vec.Vec.deref, hk]

theorem all_grants (t : Token) : ∀ (l : List Cap) (n : Nat), ∃ k,
    sliceAll capabilities.token.types.query.CapabilityToken.grants_all.closure.Insts.CoreOpsFunctionFnMutTupleSharedCapabilityBool
      t l n = ok (l.all (fun c => decide (c ∈ t.permissions.val)), k)
  | [], n => ⟨n, rfl⟩
  | x :: xs, n => by
      by_cases hx : x ∈ t.permissions.val
      · obtain ⟨k, hk⟩ := all_grants t xs (n + 1)
        exact ⟨k, by simp [sliceAll, capabilities.token.types.query.CapabilityToken.grants_all.closure.Insts.CoreOpsFunctionFnMutTupleSharedCapabilityBool.call_mut, grants_spec, hx, hk]⟩
      · exact ⟨n + 1, by simp [sliceAll, capabilities.token.types.query.CapabilityToken.grants_all.closure.Insts.CoreOpsFunctionFnMutTupleSharedCapabilityBool.call_mut, grants_spec, hx]⟩

/-- `grants_all` is every listed capability held. -/
theorem grants_all_spec (t : Token) (s : Slice Cap) :
    capabilities.token.types.query.CapabilityToken.grants_all t s =
      ok (s.val.all (fun c => decide (c ∈ t.permissions.val))) := by
  obtain ⟨k, hk⟩ := all_grants t s.val 0
  simp [capabilities.token.types.query.CapabilityToken.grants_all, core.slice.Slice.iter,
    core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.all, hk]

theorem is_valid_spec (t : Token) (h : t.expires_at_ms = none) :
    capabilities.token.types.query.CapabilityToken.is_valid t = ok (decide (t.permissions.val ≠ [])) := by
  simp [capabilities.token.types.query.CapabilityToken.is_valid,
    capabilities.token.types.query.CapabilityToken.not_expired,
    capabilities.token.types.query.CapabilityToken.has_any_permission,
    core.option.Option.map_or, alloc.vec.Vec.is_empty, h]
  generalize t.permissions.val = ps
  cases ps <;> rfl

set_option maxHeartbeats 4000000 in
theorem idx_inj (a b : Cap) : idx a = idx b ↔ a = b := by
  constructor
  · intro h
    cases a <;> cases b <;> first | rfl | (simp [idx, capabilities.types.defs.Capability.ctorIdx] at h)
  · intro h; rw [h]

theorem mem_idx (c : Cap) (ps : List Cap) :
    decide (c ∈ ps) = ps.any (fun x => idx x == idx c) := by
  induction ps with
  | nil => rfl
  | cons x xs ih =>
      simp only [List.mem_cons, List.any_cons, ← ih]
      by_cases h : c = x
      · subst h; simp
      · have h' : ¬ idx x = idx c := fun e => h ((idx_inj x c).1 e).symm
        simp [h, h']

theorem has_word (ps : List Cap) (i : Nat) :
    Formal.has (word ps) i = ps.any (fun x => idx x == i) := by
  induction ps with
  | nil => simp [word, Formal.has]
  | cons x xs ih =>
      simp only [word, List.foldr_cons, Formal.has, Nat.testBit_or, Nat.testBit_two_pow,
        List.any_cons] at ih ⊢
      rw [ih]
      by_cases e : idx x = i <;> simp [e]

theorem word_ne_zero (ps : List Cap) : (word ps != 0) = !ps.isEmpty := by
  cases ps with
  | nil => rfl
  | cons x xs =>
      have h : Formal.has (word (x :: xs)) (idx x) = true := by
        rw [has_word]; simp
      have hne : word (x :: xs) ≠ 0 := by
        intro hz; rw [hz] at h; simp [Formal.has] at h
      simp [hne]

/-! The same three facts in the form the rows compare in: every condition a
decision of a proposition about the token's list. -/

theorem mem_iff_idx (c : Cap) (ps : List Cap) :
    decide (c ∈ ps) = decide (∃ x ∈ ps, idx x = idx c) := by
  rw [mem_idx, Bool.eq_iff_iff]; simp [List.any_eq_true]

theorem has_word_iff (ps : List Cap) (i : Nat) :
    Formal.has (word ps) i = decide (∃ x ∈ ps, idx x = i) := by
  rw [has_word, Bool.eq_iff_iff]; simp [List.any_eq_true]

theorem word_ne_zero_iff (ps : List Cap) : (word ps != 0) = decide (ps ≠ []) := by
  rw [word_ne_zero]; cases ps <;> rfl

/-- Whether a token's list holds the capability with index `k`. The rows
    compare both sides in terms of this one Boolean per capability. -/
def holds (ps : List Cap) (k : Nat) : Bool := ps.any (fun x => idx x == k)

theorem mem_holds (c : Cap) (ps : List Cap) : decide (c ∈ ps) = holds ps (idx c) := mem_idx c ps

theorem has_word_holds (ps : List Cap) (i : Nat) : Formal.has (word ps) i = holds ps i := has_word ps i

/-- `Capability::bit` is `1 << idx c` for every capability, so `word` is the
    word the kernel's `caps_bits` builds from the same list. -/
theorem bit_is_two_pow (c : Cap) :
    ∃ x, capabilities.capability_bit c = ok x ∧ x.val = 2 ^ idx c := by
  cases c <;> exact ⟨_, rfl, rfl⟩

end NonosExtraction.CapTable
