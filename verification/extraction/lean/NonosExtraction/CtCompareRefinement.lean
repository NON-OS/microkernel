/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The shared thirty-two byte comparison answers equality, on the extracted code.

`crypto::util::constant_time::compare::ct_eq_32` is the comparison ed25519 now
reaches for the signature check. Its loop ORs the XOR of every byte pair into an
accumulator, a `compiler_fence` separates the loop from the test, and the answer
is whether the accumulator is zero. `CtEqRefinement` shows ed25519 goes through
it and that the fence sits between loop and test; the theorem here is what the
answer means: whenever the call returns, it returns `true` exactly when the two
arrays are equal. The loop is proven against an invariant over the range
iterator, so every one of the thirty-two bytes is shown to count.

What this cannot say: `compiler_fence` is an opaque atomic, so the theorem holds
for whatever the fence returns and says nothing about timing. That the running
time does not depend on where the arrays differ is outside a value-level proof,
as `CtEqRefinement` states.
-/

import NonosExtraction.Ct
import NonosExtraction.Bits

open Aeneas Aeneas.Std Result
open nonos_ct

set_option linter.hashCommand false

namespace NonosExtraction.CtCompare

private theorem u8_or_eq_zero (x y : Std.U8) : (x ||| y) = 0#u8 ↔ x = 0#u8 ∧ y = 0#u8 := by
  have z : ∀ w : Std.U8, w = 0#u8 ↔ ∀ i, w.val.testBit i = false := fun w => by
    rw [← Bits.val_eq_zero_iff]
    exact ⟨fun h => by rw [h]; rfl, fun h => UScalar.eq_of_val_eq (by rw [h]; rfl)⟩
  rw [z, z, z]
  simp only [UScalar.val_or, Nat.testBit_or, Bool.or_eq_false_iff]
  exact ⟨fun h => ⟨fun i => (h i).1, fun i => (h i).2⟩, fun h i => ⟨h.1 i, h.2 i⟩⟩

private theorem u8_xor_eq_zero (x y : Std.U8) : (x ^^^ y) = 0#u8 ↔ x = y := by
  constructor
  · intro h
    have hv := congrArg UScalar.val h
    simp only [UScalar.val_xor] at hv
    apply UScalar.eq_of_val_eq
    apply Nat.eq_of_testBit_eq
    intro i
    have := congrArg (fun n => Nat.testBit n i) hv
    simp only [Nat.testBit_xor] at this
    cases hx : x.val.testBit i <;> cases hy : y.val.testBit i <;> simp_all
  · rintro rfl
    apply UScalar.eq_of_val_eq
    simp [UScalar.val_xor]

@[step]
private theorem ct_eq_32_loop_spec (a b : Array Std.U8 32#usize) (iter : core.ops.range.Range Std.Usize)
    (diff : Std.U8) (hend : iter.end.val = 32) (hs : iter.start.val ≤ 32) :
    crypto.util.constant_time.compare.ct_eq_32_loop iter a b diff ⦃ r =>
      (r = 0#u8 ↔ diff = 0#u8 ∧ ∀ j, iter.start.val ≤ j → j < 32 → a.val[j]! = b.val[j]!) ⦄ := by
  unfold crypto.util.constant_time.compare.ct_eq_32_loop
  apply loop.spec_decr_nat
    (measure := fun (st : core.ops.range.Range Std.Usize × Std.U8) => 32 - st.1.start.val)
    (inv := fun (st : core.ops.range.Range Std.Usize × Std.U8) =>
      st.1.end.val = 32 ∧ iter.start.val ≤ st.1.start.val ∧ st.1.start.val ≤ 32 ∧
      ((st.2 = 0#u8 ∧ ∀ j, st.1.start.val ≤ j → j < 32 → a.val[j]! = b.val[j]!) ↔
        (diff = 0#u8 ∧ ∀ j, iter.start.val ≤ j → j < 32 → a.val[j]! = b.val[j]!)))
  · intro st hst
    obtain ⟨he, hlo, hle, hiff⟩ := hst
    unfold crypto.util.constant_time.compare.ct_eq_32_loop.body
    step*
    all_goals
      have ho : o = some i := by assumption
      by_cases hlt : st.1.start.val < st.1.end.val
      · rw [if_pos hlt] at o_post1
        obtain ⟨hsome, hstart⟩ := o_post1
        rw [hsome] at ho
        cases ho
        first
          | scalar_tac
          | (have hi3 : i3 = i1 ^^^ i2 := UScalar.eq_of_val_eq (by rw [i3_post1])
             have hd1 : diff1 = st.2 ||| i3 := UScalar.eq_of_val_eq (by rw [diff1_post1])
             have hlen : st.1.start.val < (↑a : List Std.U8).length := by scalar_tac
             have hlenb : st.1.start.val < (↑b : List Std.U8).length := by scalar_tac
             refine ⟨by rw [o_post2]; exact he, by omega, by omega, ?_, by omega⟩
             rw [← hiff, hd1, u8_or_eq_zero, hi3, u8_xor_eq_zero, i1_post, i2_post]
             constructor
             · rintro ⟨⟨h0, heq⟩, hrest⟩
               refine ⟨h0, fun j hj hj32 => ?_⟩
               rcases Nat.eq_or_lt_of_le hj with h | h
               · subst h
                 rw [getElem!_pos (↑a : List Std.U8) _ hlen, getElem!_pos (↑b : List Std.U8) _ hlenb]
                 exact heq
               · exact hrest j (by omega) hj32
             · rintro ⟨h0, hall⟩
               refine ⟨⟨h0, ?_⟩, fun j hj hj32 => hall j (by omega) hj32⟩
               have := hall st.1.start.val (le_refl _) (by omega)
               rwa [getElem!_pos (↑a : List Std.U8) _ hlen, getElem!_pos (↑b : List Std.U8) _ hlenb] at this)
      · rw [if_neg hlt] at o_post1
        rw [o_post1.1] at ho
        cases ho
  · exact ⟨hend, le_refl _, hs, Iff.rfl⟩

private theorem arrays_equal_iff_pointwise (a b : Array Std.U8 32#usize) :
    a = b ↔ ∀ j, 0 ≤ j → j < 32 → a.val[j]! = b.val[j]! := by
  constructor
  · rintro rfl; intros; rfl
  · intro h
    apply Subtype.ext
    apply List.ext_getElem (by simp [a.property, b.property])
    intro j hja hjb
    have ha : (↑a : List Std.U8).length = 32 := by simp [a.property]
    have := h j (Nat.zero_le _) (by omega)
    rwa [getElem!_pos (↑a : List Std.U8) _ hja, getElem!_pos (↑b : List Std.U8) _ hjb] at this

/-- Whatever the fence does, an answer the comparison returns is `true` exactly
    when the two thirty-two byte arrays are equal. -/
theorem ct_eq_32_answers_equality (a b : Array Std.U8 32#usize) (r : Bool)
    (h : crypto.util.constant_time.compare.ct_eq_32 a b = ok r) : r = true ↔ a = b := by
  unfold crypto.util.constant_time.compare.ct_eq_32 at h
  obtain ⟨d, hd, hpost⟩ := WP.spec_imp_exists (ct_eq_32_loop_spec a b { start := 0#usize, «end» := 32#usize } 0#u8 rfl (by decide))
  rw [hd] at h
  simp only [bind_tc_ok] at h
  cases hf : crypto.util.constant_time.barriers.compiler_fence with
  | ok u =>
    rw [hf, bind_tc_ok] at h
    simp only [ok.injEq] at h
    subst h
    rw [decide_eq_true_iff, arrays_equal_iff_pointwise]
    simp only at hpost
    rw [hpost]
    simp
  | fail e => rw [hf] at h; simp at h
  | div => rw [hf] at h; simp at h

end NonosExtraction.CtCompare

#print axioms NonosExtraction.CtCompare.ct_eq_32_answers_equality
