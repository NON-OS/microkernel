/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The allowlist, `src/syscall/contract/allowlist/{set,decide}.rs`, as Aeneas
translates it. Each process carries two bitmaps over the registry's slots,
one answer for a caller that launches apps and one for a caller that does
not, and every syscall is decided by one bit test. Proved here, for any
registry count up to the bitmap's 256 bits and any slot lookup:

- `plain` and `launcher` read bit `s % 64` of word `s / 64`, and refuse a
  slot past the count;
- `EMPTY`, the list of a token that failed its MAC, permits nothing;
- `set` raises exactly the asked bit in each map and leaves every other
  slot's answer as it was;
- `allowed` refuses a number with no registry slot, and otherwise reads the
  map the caller's standing selects.

Together with `CapTableRefinement`, which proves the table each bit is set
from equal to the model's, these give the model's `allowed list nr l =
list nr l` for the bitmap `derive` builds slot by slot.
-/

import Allowlist.Funs
open Aeneas Aeneas.Std Result

namespace NonosExtraction.Allowlist
open nonos_x_allowlist

abbrev AL := syscall.contract.allowlist.set.Allowlist
abbrev Words := Array Std.U64 4#usize

/-- Bit `s` of a four-word map: bit `s % 64` of word `s / 64`. -/
def bitAt (a : Words) (s : Nat) : Bool := (a.val[s / 64]!).val.testBit (s % 64)

theorem and_shift_ne_zero (x : Std.U64) (r : Std.Usize) (hr : r.val < 64) (b : Std.U64)
    (hb : b.val = (1 <<< r.val) % Std.U64.size) :
    ((x &&& b) != 0#u64) = x.val.testBit r.val := by
  have hb' : b.val = 2 ^ r.val := by
    rw [hb, Nat.shiftLeft_eq, Nat.one_mul]
    have h64 : Std.U64.size = 2 ^ 64 := by rw [Std.U64.size_def, Std.U64.numBits_def]; rfl
    rw [h64]
    exact Nat.mod_eq_of_lt (Nat.pow_lt_pow_right (by decide) hr)
  have hv : (x &&& b).val = (x.val.testBit r.val).toNat * 2 ^ r.val := by
    rw [UScalar.val_and, hb', Nat.and_two_pow]
  cases ht : x.val.testBit r.val with
  | false =>
      have h0 : (x &&& b) = 0#u64 := by
        apply UScalar.eq_of_val_eq; rw [hv, ht]; simp
      simp [h0]
  | true =>
      have h0 : (x &&& b) ≠ 0#u64 := by
        intro h; have := congrArg UScalar.val h; rw [hv, ht] at this; simp at this
      simp [h0]

theorem plain_spec (L : AL) (s : Std.Usize) {c : Std.Usize}
    (hc : syscall.abi.index.COUNT = ok c) (hb : c.val ≤ 256) :
    syscall.contract.allowlist.set.Allowlist.impl.plain L s =
      ok (decide (s.val < c.val) && bitAt L.plain s.val) := by
  unfold syscall.contract.allowlist.set.Allowlist.impl.plain
  rw [hc]
  simp only [bind_tc_ok]
  by_cases hs : s.val < c.val
  · have hlt : s < c := hs
    rw [if_pos hlt]
    obtain ⟨w, hw, hwv, _⟩ := UScalar.div_bv_spec s (y := 64#usize) (by simp)
    rw [hw]; simp only [bind_tc_ok]
    have hl : w.val < L.plain.length := by simp [hwv]; omega
    obtain ⟨x, hx, hxv⟩ := WP.spec_imp_exists (Array.index_usize_spec L.plain w hl)
    rw [hx]; simp only [bind_tc_ok]
    obtain ⟨r, hr, hrv, _⟩ := WP.spec_imp_exists (UScalar.rem_bv_spec s (y := 64#usize) (by simp))
    rw [hr]; simp only [bind_tc_ok]
    have hr64 : r.val < 64 := by rw [hrv]; simp; omega
    obtain ⟨b, hbit, hbv, _⟩ := WP.spec_imp_exists (U64.ShiftLeft_spec 1#u64 r (by simpa using hr64))
    rw [hbit]; simp only [bind_tc_ok, lift]
    rw [and_shift_ne_zero x r hr64 b (by simpa using hbv)]
    simp only [bitAt, hs, decide_true, Bool.true_and, hxv, hrv, hwv]
    have hlen : s.val / 64 < L.plain.val.length := by
      have := L.plain.property; simp at this ⊢; omega
    simp [List.getElem!_eq_getElem?_getD, List.getElem?_eq_getElem hlen]
  · have hlt : ¬ s < c := hs
    rw [if_neg hlt]
    simp [hs]

theorem launcher_spec (L : AL) (s : Std.Usize) {c : Std.Usize}
    (hc : syscall.abi.index.COUNT = ok c) (hb : c.val ≤ 256) :
    syscall.contract.allowlist.set.Allowlist.impl.launcher L s =
      ok (decide (s.val < c.val) && bitAt L.launcher s.val) := by
  unfold syscall.contract.allowlist.set.Allowlist.impl.launcher
  rw [hc]
  simp only [bind_tc_ok]
  by_cases hs : s.val < c.val
  · have hlt : s < c := hs
    rw [if_pos hlt]
    obtain ⟨w, hw, hwv, _⟩ := UScalar.div_bv_spec s (y := 64#usize) (by simp)
    rw [hw]; simp only [bind_tc_ok]
    have hl : w.val < L.launcher.length := by simp [hwv]; omega
    obtain ⟨x, hx, hxv⟩ := WP.spec_imp_exists (Array.index_usize_spec L.launcher w hl)
    rw [hx]; simp only [bind_tc_ok]
    obtain ⟨r, hr, hrv, _⟩ := WP.spec_imp_exists (UScalar.rem_bv_spec s (y := 64#usize) (by simp))
    rw [hr]; simp only [bind_tc_ok]
    have hr64 : r.val < 64 := by rw [hrv]; simp; omega
    obtain ⟨b, hbit, hbv, _⟩ := WP.spec_imp_exists (U64.ShiftLeft_spec 1#u64 r (by simpa using hr64))
    rw [hbit]; simp only [bind_tc_ok, lift]
    rw [and_shift_ne_zero x r hr64 b (by simpa using hbv)]
    simp only [bitAt, hs, decide_true, Bool.true_and, hxv, hrv, hwv]
    have hlen : s.val / 64 < L.launcher.val.length := by
      have := L.launcher.property; simp at this ⊢; omega
    simp [List.getElem!_eq_getElem?_getD, List.getElem?_eq_getElem hlen]
  · have hlt : ¬ s < c := hs
    rw [if_neg hlt]
    simp [hs]

/-- `EMPTY`, the list of a token that fails its MAC check, permits nothing. -/
theorem empty_permits_nothing (s : Std.Usize) {c : Std.Usize}
    (hc : syscall.abi.index.COUNT = ok c) (hb : c.val ≤ 256) :
    syscall.contract.allowlist.set.Allowlist.impl.plain
        syscall.contract.allowlist.set.Allowlist.EMPTY s = ok false ∧
      syscall.contract.allowlist.set.Allowlist.impl.launcher
        syscall.contract.allowlist.set.Allowlist.EMPTY s = ok false := by
  rw [plain_spec _ s hc hb, launcher_spec _ s hc hb]
  have h0 : ∀ n, bitAt (Array.repeat 4#usize 0#u64) n = false := by
    intro n
    simp only [bitAt, Array.repeat, List.getElem!_eq_getElem?_getD]
    generalize n / 64 = i
    rcases i with _ | _ | _ | _ | i <;> simp
  unfold syscall.contract.allowlist.set.Allowlist.EMPTY
  simp [h0]

/-- One word raised by one bit: the bits of every other word, and every
    other bit of this word, are as they were. -/
theorem bitAt_raise (a : Words) (w : Std.Usize) (hw : w.val < 4) (x b : Std.U64)
    (hx : x = a.val[w.val]!) (hb : b.val = 2 ^ (k : Nat)) (n : Nat) :
    bitAt (a.set w (x ||| b)) n = (bitAt a n || decide (n / 64 = w.val ∧ n % 64 = k)) := by
  have hlen : a.val.length = 4 := by have := a.property; simpa using this
  simp only [bitAt, Std.Array.set_val_eq]
  by_cases hq : n / 64 = w.val
  · rw [hq]
    have hset : (a.val.set w.val (x ||| b))[w.val]! = x ||| b := by
      simp [hlen, hw]
    rw [hset, UScalar.val_or, hb, Nat.testBit_or, Nat.testBit_two_pow, hx]
    by_cases hk : k = n % 64
    · subst hk; simp
    · have : ¬ n % 64 = k := fun e => hk e.symm
      simp [hk, this]
  · have hne : w.val ≠ n / 64 := fun e => hq e.symm
    simp [List.getElem!_eq_getElem?_getD, hq]

theorem bang_eq (a : Words) (i : Nat) (h : i < a.val.length) : a.val[i]! = a.val[i] := by
  simp [List.getElem!_eq_getElem?_getD, List.getElem?_eq_getElem h]

theorem slot_eq_iff (n s : Nat) : (n / 64 = s / 64 ∧ n % 64 = s % 64) ↔ n = s := by
  constructor
  · rintro ⟨h1, h2⟩
    rw [← Nat.div_add_mod n 64, ← Nat.div_add_mod s 64, h1, h2]
  · rintro rfl; exact ⟨rfl, rfl⟩

/-- **`set` raises exactly the asked bits.** For a slot inside the count,
    each map gains the slot's bit when asked and every other slot keeps its
    answer; this is the step `derive` takes once per registered slot. -/
theorem set_spec (L : AL) (s : Std.Usize) {c : Std.Usize}
    (hc : syscall.abi.index.COUNT = ok c) (hb : c.val ≤ 256) (hs : s.val < c.val) (p q : Bool) :
    ∃ L', syscall.contract.allowlist.set.Allowlist.set L s p q = ok L' ∧
      (∀ n, bitAt L'.plain n = (bitAt L.plain n || (p && decide (n = s.val)))) ∧
      (∀ n, bitAt L'.launcher n = (bitAt L.launcher n || (q && decide (n = s.val)))) := by
  unfold syscall.contract.allowlist.set.Allowlist.set
  rw [hc]
  simp only [bind_tc_ok]
  have hge : ¬ s ≥ c := by
    show ¬ c.val ≤ s.val
    omega
  rw [if_neg hge]
  obtain ⟨w, hw, hwv, _⟩ := UScalar.div_bv_spec s (y := 64#usize) (by simp)
  rw [hw]; simp only [bind_tc_ok]
  obtain ⟨r, hr, hrv, _⟩ := WP.spec_imp_exists (UScalar.rem_bv_spec s (y := 64#usize) (by simp))
  rw [hr]; simp only [bind_tc_ok]
  have hr64 : r.val < 64 := by rw [hrv]; simp; omega
  obtain ⟨b, hbit, hbv, _⟩ := WP.spec_imp_exists (U64.ShiftLeft_spec 1#u64 r (by simpa using hr64))
  rw [hbit]; simp only [bind_tc_ok]
  have hb2 : b.val = 2 ^ r.val := by
    have h64 : Std.U64.size = 2 ^ 64 := by rw [Std.U64.size_def, Std.U64.numBits_def]; rfl
    have hlt : 2 ^ r.val < 2 ^ 64 := Nat.pow_lt_pow_right (by decide) hr64
    rw [hbv, h64]
    simp [Nat.shiftLeft_eq, Nat.mod_eq_of_lt hlt]
    simpa using hlt
  have hw3 : w.val < 4 := by simp [hwv]; omega
  have hwd : w.val = s.val / 64 := by simp [hwv]
  have hrd : r.val = s.val % 64 := by simp [hrv]
  have key : ∀ n, decide (n / 64 = w.val ∧ n % 64 = r.val) = decide (n = s.val) := by
    intro n; rw [hwd, hrd]; exact decide_eq_decide.2 (slot_eq_iff n s.val)
  have hlp : w.val < L.plain.length := by simpa using hw3
  have hll : w.val < L.launcher.length := by simpa using hw3
  have hpv : w.val < L.plain.val.length := by have := L.plain.property; simp at this ⊢; omega
  have hlv : w.val < L.launcher.val.length := by have := L.launcher.property; simp at this ⊢; omega
  cases p <;> cases q
  · exact ⟨L, rfl, fun n => by simp, fun n => by simp⟩
  · obtain ⟨x, hx, hxv⟩ := WP.spec_imp_exists (Array.index_usize_spec L.launcher w hll)
    obtain ⟨a, ha, hav⟩ := WP.spec_imp_exists (Array.update_spec L.launcher w (x ||| b) hll)
    refine ⟨{ L with launcher := a }, by simp [hx, ha, lift], fun n => by simp, fun n => ?_⟩
    rw [hav, bitAt_raise L.launcher w hw3 x b (by rw [hxv, bang_eq _ _ (by assumption)]) hb2 n, key]; simp
  · obtain ⟨x, hx, hxv⟩ := WP.spec_imp_exists (Array.index_usize_spec L.plain w hlp)
    obtain ⟨a, ha, hav⟩ := WP.spec_imp_exists (Array.update_spec L.plain w (x ||| b) hlp)
    refine ⟨{ L with plain := a }, by simp [hx, ha, lift], fun n => ?_, fun n => by simp⟩
    rw [hav, bitAt_raise L.plain w hw3 x b (by rw [hxv, bang_eq _ _ (by assumption)]) hb2 n, key]; simp
  · obtain ⟨x, hx, hxv⟩ := WP.spec_imp_exists (Array.index_usize_spec L.plain w hlp)
    obtain ⟨a, ha, hav⟩ := WP.spec_imp_exists (Array.update_spec L.plain w (x ||| b) hlp)
    obtain ⟨y, hy, hyv⟩ := WP.spec_imp_exists (Array.index_usize_spec L.launcher w hll)
    obtain ⟨a2, ha2, hav2⟩ := WP.spec_imp_exists (Array.update_spec L.launcher w (y ||| b) hll)
    refine ⟨{ L with plain := a, launcher := a2 }, by simp [hx, ha, hy, ha2, lift], fun n => ?_, fun n => ?_⟩
    · rw [hav, bitAt_raise L.plain w hw3 x b (by rw [hxv, bang_eq _ _ (by assumption)]) hb2 n, key]; simp
    · show bitAt a2 n = _
      rw [hav2, bitAt_raise L.launcher w hw3 y b (by rw [hyv, bang_eq _ _ hlv]) hb2 n, key]; simp

/-- **`allowed` refuses an unregistered number.** A number the registry
    gives no slot is refused whatever the maps hold. -/
theorem allowed_refuses_unregistered (L : AL) (nr : Std.U64) (lc : Bool)
    (h : syscall.abi.index.slot_of nr = ok none) :
    syscall.contract.allowlist.decide.allowed L nr lc = ok false := by
  simp [syscall.contract.allowlist.decide.allowed, h]

/-- **`allowed` reads the map the caller's standing selects.** For a
    registered number, a launcher is answered from the launcher map and any
    other caller from the plain map, each by the slot's bit. -/
theorem allowed_reads_its_slot (L : AL) (nr : Std.U64) (lc : Bool) {s c : Std.Usize}
    (h : syscall.abi.index.slot_of nr = ok (some s))
    (hc : syscall.abi.index.COUNT = ok c) (hb : c.val ≤ 256) :
    syscall.contract.allowlist.decide.allowed L nr lc =
      ok (decide (s.val < c.val) && bitAt (if lc then L.launcher else L.plain) s.val) := by
  cases lc <;>
    simp [syscall.contract.allowlist.decide.allowed, h, plain_spec _ _ hc hb, launcher_spec _ _ hc hb]

/-! ### The extraction root's forwarding functions -/

theorem the_set_wrapper_is_its_method (L : AL) (s : Std.Usize) (p q : Bool) :
    syscall.contract.allowlist.set L s p q = syscall.contract.allowlist.set.Allowlist.set L s p q := rfl

theorem the_plain_wrapper_is_its_method (L : AL) (s : Std.Usize) :
    syscall.contract.allowlist.plain L s = syscall.contract.allowlist.set.Allowlist.impl.plain L s := rfl

theorem the_launcher_wrapper_is_its_method (L : AL) (s : Std.Usize) :
    syscall.contract.allowlist.launcher L s =
      syscall.contract.allowlist.set.Allowlist.impl.launcher L s := rfl

theorem the_empty_wrapper_is_EMPTY :
    syscall.contract.allowlist.empty = ok syscall.contract.allowlist.set.Allowlist.EMPTY := rfl

end NonosExtraction.Allowlist

/-! ### Axiom profile -/

#print axioms NonosExtraction.Allowlist.plain_spec
#print axioms NonosExtraction.Allowlist.launcher_spec
#print axioms NonosExtraction.Allowlist.empty_permits_nothing
#print axioms NonosExtraction.Allowlist.set_spec
#print axioms NonosExtraction.Allowlist.allowed_refuses_unregistered
#print axioms NonosExtraction.Allowlist.allowed_reads_its_slot
