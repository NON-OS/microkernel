/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The constant-time selectors for every operand, and the leading-zero count.

`CtPrimitivesRefinement` states the selectors on sample operands and records
that `ct_select_usize` and `ct_clz_u64` did not close there. They close here.

The three selectors turn a `bool` into an all-ones or all-zero mask and combine
the operands with it. The theorems below say each returns its first operand on
`true` and its second on `false` for every pair of operands, and for
`ct_select_usize` on a 32-bit or a 64-bit `usize` alike, which is what the
`cast` that stopped reduction hid.

`ct_clz_u64` is six stages of `ct_is_zero_u64` and the selectors, halving the
width each time, and a final one-bit step. `ct_is_zero_u64` calls
`u64::wrapping_neg`, which the extraction leaves opaque, so no statement about
either can be proven outright. Every theorem that needs it takes the Rust
meaning of `wrapping_neg`, two's complement negation, as the hypothesis `hneg`,
the way `ElfBoundsRefinement` takes `Option::ok_or`. Under it, `ct_is_zero_u64`
is one exactly at zero, and `ct_clz_u64` returns 64 for zero and otherwise the
shift that brings the leading one to bit 63. The proof keeps an invariant per
stage: the word is the input times a power of two without overflow, and its
leading one lies in the bits the next stage still has to inspect.

These are value statements. That the compiler keeps the computation free of
branches is a property of the generated code, and nothing here says it.
-/

import NonosExtraction.Ct

open Aeneas Aeneas.Std Result
open nonos_ct

set_option linter.hashCommand false

namespace NonosExtraction.CtSelectClz

/-! ### The selectors -/

private theorem u64_ext {x y : Std.U64} (h : x.bv = y.bv) : x = y := by
  cases x; cases y; simp_all
private theorem u32_ext {x y : Std.U32} (h : x.bv = y.bv) : x = y := by
  cases x; cases y; simp_all
private theorem usize_ext {x y : Std.Usize} (h : x.bv = y.bv) : x = y := by
  cases x; cases y; simp_all

/-- `ct_select_u64` returns its first operand on `true` and its second on
    `false`, for every pair of operands. -/
theorem ct_select_u64_selects (c : Bool) (a b : Std.U64) :
    crypto.util.constant_time.select.ct_select_u64 c a b = ok (if c then a else b) := by
  unfold crypto.util.constant_time.select.ct_select_u64
  cases c
  · have h : (-. (IScalar.cast_fromBool .I64 false) : Result Std.I64) = ok 0#i64 := by rfl
    simp only [lift, bind_tc_ok, h]
    congr 1; apply u64_ext
    simp [UScalar.bv_and, UScalar.bv_or, UScalar.bv_not, IScalar.hcast]
  · have h : (-. (IScalar.cast_fromBool .I64 true) : Result Std.I64) = ok (-1)#i64 := by rfl
    simp only [lift, bind_tc_ok, h]
    congr 1; apply u64_ext
    have : (IScalar.hcast .U64 ((-1)#i64 : Std.I64)).bv = BitVec.allOnes 64 := by rfl
    simp [UScalar.bv_and, UScalar.bv_or, UScalar.bv_not, this]

/-- `ct_select_u32` returns its first operand on `true` and its second on
    `false`, for every pair of operands. -/
theorem ct_select_u32_selects (c : Bool) (a b : Std.U32) :
    crypto.util.constant_time.select.ct_select_u32 c a b = ok (if c then a else b) := by
  unfold crypto.util.constant_time.select.ct_select_u32
  cases c
  · have h : (-. (IScalar.cast_fromBool .I32 false) : Result Std.I32) = ok 0#i32 := by rfl
    simp only [lift, bind_tc_ok, h]
    congr 1; apply u32_ext
    simp [UScalar.bv_and, UScalar.bv_or, UScalar.bv_not, IScalar.hcast]
  · have h : (-. (IScalar.cast_fromBool .I32 true) : Result Std.I32) = ok (-1)#i32 := by rfl
    simp only [lift, bind_tc_ok, h]
    congr 1; apply u32_ext
    have : (IScalar.hcast .U32 ((-1)#i32 : Std.I32)).bv = BitVec.allOnes 32 := by rfl
    simp [UScalar.bv_and, UScalar.bv_or, UScalar.bv_not, this]

private theorem cfb_true : (IScalar.cast_fromBool .Isize true).val = 1 := by
  simp only [IScalar.cast_fromBool, ite_true, IScalar.val]
  apply BitVec.toInt_one_of_lt
  simp only [IScalarTy.numBits]
  rcases System.Platform.numBits_eq with h | h <;> omega

private theorem cfb_false : (IScalar.cast_fromBool .Isize false).val = 0 := by
  simp only [IScalar.cast_fromBool, Bool.false_eq_true, ite_false, IScalar.val]
  simp

private theorem cfb_val (c : Bool) :
    (IScalar.cast_fromBool .Isize c).val = if c then 1 else 0 := by
  cases c
  · exact cfb_false
  · exact cfb_true

/-- `ct_select_usize` returns its first operand on `true` and its second on
    `false`, on a 32-bit or a 64-bit `usize`. -/
theorem ct_select_usize_selects (c : Bool) (a b : Std.Usize) :
    crypto.util.constant_time.select.ct_select_usize c a b = ok (if c then a else b) := by
  unfold crypto.util.constant_time.select.ct_select_usize
  simp only [lift, bind_tc_ok]
  have hw := System.Platform.numBits_eq
  have hm : IScalar.cast_fromBool .Isize c ≠ IScalar.min .Isize := by
    intro e
    rw [cfb_val] at e
    simp only [IScalar.min, IScalarTy.numBits] at e
    have : (2 : Int) ^ (System.Platform.numBits - 1) ≥ 2 ^ 31 := by
      rcases hw with hn | hn <;> (rw [hn]; try decide)
    split at e <;> omega
  obtain ⟨r, hr, hrv⟩ := WP.spec_imp_exists (HNeg.hNeg.step _ hm)
  rw [cfb_val] at hrv
  simp only [hr, bind_tc_ok]
  congr 1
  apply usize_ext
  have hext : (IScalar.hcast UScalarTy.Usize r).bv = r.bv := BitVec.signExtend_eq r.bv
  have hrb : r.bv.toInt = r.val := rfl
  cases c
  · have hr0 : r.bv = 0#_ := by
      apply BitVec.eq_of_toInt_eq
      rw [hrb, hrv]; simp
    simp [UScalar.bv_and, UScalar.bv_or, UScalar.bv_not, hext, hr0]
  · have hr1 : r.bv = BitVec.allOnes _ := by
      apply BitVec.eq_of_toInt_eq
      rw [hrb, hrv, BitVec.toInt_allOnes]
      simp only [IScalarTy.numBits]
      rcases hw with hn | hn <;> simp [hn]
    simp [UScalar.bv_and, UScalar.bv_or, UScalar.bv_not, hext, hr1]

/-! ### Zero test and leading-zero count -/

/-- Given the Rust meaning of `wrapping_neg`, `ct_is_zero_u64` answers one
    exactly at zero and zero everywhere else. The top bit of `y | -y` is set
    exactly when `y` is not zero. -/
theorem ct_is_zero_u64_is_one_exactly_at_zero
    (hneg : ∀ x : Std.U64, core.num.U64.wrapping_neg x = ok ⟨-x.bv⟩) (y : Std.U64) :
    crypto.util.constant_time.compare.ct_is_zero_u64 y =
      ok (if y.val = 0 then 1#u64 else 0#u64) := by
  unfold crypto.util.constant_time.compare.ct_is_zero_u64
  simp only [hneg, lift, bind_tc_ok]
  have hy := y.hBounds
  simp only [UScalarTy.U64_numBits_eq] at hy
  obtain ⟨z, hz, hzv, -⟩ := WP.spec_imp_exists
    (UScalar.ShiftRight_IScalar_spec (y ||| (⟨-y.bv⟩ : Std.U64)) 63#i32 (by decide) (by decide))
  rw [hz]; simp only [bind_tc_ok]
  have hor : (y ||| (⟨-y.bv⟩ : Std.U64)).val = y.val ||| (2 ^ 64 - y.val) % 2 ^ 64 := by
    rw [UScalar.val_or]
    show _ ||| (-y.bv).toNat = _
    rw [BitVec.toNat_neg]
    rfl
  have hlt : y.val ||| (2 ^ 64 - y.val) % 2 ^ 64 < 2 ^ 64 :=
    Nat.or_lt_two_pow hy (Nat.mod_lt _ (by decide))
  have h63 : (63#i32 : Std.I32).toNat = 63 := rfl
  rw [h63, hor, Nat.shiftRight_eq_div_pow] at hzv
  congr 1
  apply u64_ext
  by_cases h0 : y.val = 0
  · have : z.val = 0 := by rw [hzv, h0]; decide
    have hz0 : z = 0#u64 := UScalar.eq_of_val_eq this
    simp [h0, hz0]
  · have hge : 2 ^ 63 ≤ y.val ||| (2 ^ 64 - y.val) % 2 ^ 64 := by
      by_cases hb : 2 ^ 63 ≤ y.val
      · exact Nat.le_trans hb Nat.left_le_or
      · rw [Nat.or_comm]
        refine Nat.le_trans ?_ Nat.left_le_or
        rw [Nat.mod_eq_of_lt (by omega)]
        omega
    have : z.val = 1 := by rw [hzv]; omega
    have hz1 : z = 1#u64 := UScalar.eq_of_val_eq this
    simp [h0, hz1]

/-- One halving step of the count, on naturals. `v` is the word after the
    stages so far, `x * 2 ^ m` without overflow, with its leading one in the top
    `2 * k` bits. If the top `k` bits are clear the stage shifts by `k`; either
    way the leading one ends in the top `k` bits. -/
private theorem clz_stage (x v m k : Nat) (hk : 0 < k) (hk2 : 2 * k ≤ 64)
    (hv : v = x * 2 ^ m) (hv64 : v < 2 ^ 64) (hlead : x = 0 ∨ 2 ^ (64 - 2 * k) ≤ v)
    (c : Prop) [Decidable c] (hc : c ↔ v / 2 ^ (64 - k) = 0) (v' : Nat)
    (hv' : v' = if c then v * 2 ^ k % 2 ^ 64 else v) :
    v' = x * 2 ^ (m + if c then k else 0) ∧ v' < 2 ^ 64 ∧ (x = 0 ∨ 2 ^ (64 - k) ≤ v') := by
  have hsplit : (2 : Nat) ^ 64 = 2 ^ (64 - k) * 2 ^ k := by
    rw [← Nat.pow_add]; congr 1; omega
  by_cases h : c
  · have hlt : v < 2 ^ (64 - k) := by
      have := hc.mp h
      rcases Nat.eq_zero_or_pos (2 ^ (64 - k)) with h0 | h0
      · exact absurd h0 (by positivity)
      · exact (Nat.div_eq_zero_iff_lt h0).mp this
    have hmul : v * 2 ^ k < 2 ^ 64 := by
      rw [hsplit]; exact Nat.mul_lt_mul_of_pos_right hlt (by positivity)
    simp only [h, ↓reduceIte] at hv' ⊢
    rw [Nat.mod_eq_of_lt hmul] at hv'
    refine ⟨by rw [hv', hv, Nat.pow_add, Nat.mul_assoc], by omega, ?_⟩
    rcases hlead with h0 | hl
    · exact Or.inl h0
    · right
      have e : (2 : Nat) ^ (64 - k) = 2 ^ (64 - 2 * k) * 2 ^ k := by
        rw [← Nat.pow_add]; congr 1; omega
      rw [hv', e]
      exact Nat.mul_le_mul_right _ hl
  · have hge : 2 ^ (64 - k) ≤ v := by
      by_contra hn
      exact h (hc.mpr ((Nat.div_eq_zero_iff_lt (by positivity)).mpr (by omega)))
    simp only [h, ↓reduceIte, Nat.add_zero] at hv' ⊢
    exact ⟨by rw [hv', hv], by omega, Or.inr (by omega)⟩

private theorem small_sum (a b c d e f g : Nat) (ha : a ≤ 32) (hb : b ≤ 16) (hc : c ≤ 8)
    (hd : d ≤ 4) (he : e ≤ 2) (hf : f ≤ 1) (hg : g ≤ 1) :
    a + b + c + d + e + f + g ≤ 4294967295 := by omega

private theorem small_sum6 (a b c d e f : Nat) (ha : a ≤ 32) (hb : b ≤ 16) (hc : c ≤ 8)
    (hd : d ≤ 4) (he : e ≤ 2) (hf : f ≤ 1) :
    a + b + c + d + e + f ≤ 4294967295 := by omega

private theorem le32 {x : Std.U32} (h : x.val = 32 ∨ x.val = 0) : x.val ≤ 32 := by omega
private theorem le16 {x : Std.U32} (h : x.val = 16 ∨ x.val = 0) : x.val ≤ 16 := by omega
private theorem le8 {x : Std.U32} (h : x.val = 8 ∨ x.val = 0) : x.val ≤ 8 := by omega
private theorem le4 {x : Std.U32} (h : x.val = 4 ∨ x.val = 0) : x.val ≤ 4 := by omega
private theorem le2 {x : Std.U32} (h : x.val = 2 ∨ x.val = 0) : x.val ≤ 2 := by omega
private theorem le1 {x : Std.U32} (h : x.val = 1 ∨ x.val = 0) : x.val ≤ 1 := by omega

@[local step]
private theorem sel32_step (c : Bool) (a b : Std.U32) :
    crypto.util.constant_time.select.ct_select_u32 c a b ⦃ r =>
      r = (if c then a else b) ∧ (r.val = a.val ∨ r.val = b.val) ⦄ := by
  rw [ct_select_u32_selects]
  simp only [WP.spec_ok]
  split <;> simp

@[local step]
private theorem sel64_step (c : Bool) (a b : Std.U64) :
    crypto.util.constant_time.select.ct_select_u64 c a b ⦃ r => r = if c then a else b ⦄ := by
  rw [ct_select_u64_selects]; simp

@[local step]
private theorem is_zero_step
    (hneg : ∀ x : Std.U64, core.num.U64.wrapping_neg x = ok ⟨-x.bv⟩)
    (y : Std.U64) :
    crypto.util.constant_time.compare.ct_is_zero_u64 y ⦃ r =>
      ((r != 0#u64) = true ↔ y.val = 0) ⦄ := by
  rw [ct_is_zero_u64_is_one_exactly_at_zero hneg]
  by_cases h : y.val = 0 <;> simp [h]

set_option maxHeartbeats 4000000 in
/-- Given the Rust meaning of `wrapping_neg`, `ct_clz_u64` counts leading zeros:
    it returns 64 for zero, and for any other word the shift `n` that brings its
    leading one to bit 63, so `2 ^ 63 ≤ x * 2 ^ n < 2 ^ 64`. -/
theorem ct_clz_u64_counts_leading_zeros
    (hneg : ∀ x : Std.U64, core.num.U64.wrapping_neg x = ok ⟨-x.bv⟩) (x : Std.U64) :
    crypto.util.constant_time.math.ct_clz_u64 x ⦃ n =>
      (x.val = 0 ∧ n.val = 64) ∨
        (x.val ≠ 0 ∧ 2 ^ 63 ≤ x.val * 2 ^ n.val ∧ x.val * 2 ^ n.val < 2 ^ 64) ⦄ := by
  unfold crypto.util.constant_time.math.ct_clz_u64
  step*
  · (rw [show U32.max = 4294967295 by simp [U32.max, U32.numBits], n4_post, n3_post, n2_post,
      n1_post]
     exact small_sum6 _ _ _ _ _ _ (le32 n_post2) (le16 i3_post2) (le8 i6_post2) (le4 i9_post2)
       (le2 i12_post2) (le1 i15_post2))
  · (rw [show U32.max = 4294967295 by simp [U32.max, U32.numBits], n5_post, n4_post, n3_post,
      n2_post, n1_post]
     exact small_sum _ _ _ _ _ _ _ (le32 n_post2) (le16 i3_post2) (le8 i6_post2) (le4 i9_post2)
       (le2 i12_post2) (le1 i15_post2) (le1 i18_post2))
  · have hx := x.hBounds
    simp only [UScalarTy.U64_numBits_eq] at hx
    -- the selected words and counts, as naturals
    have sel : ∀ (c : Prop) [Decidable c] (a b r : Std.U64), r = (if c then a else b) →
        r.val = if c then a.val else b.val := by
      intro c _ a b r h; subst h; split <;> rfl
    have sel32v : ∀ (c : Prop) [Decidable c] (a b r : Std.U32), r = (if c then a else b) →
        r.val = if c then a.val else b.val := by
      intro c _ a b r h; subst h; split <;> rfl
    have shl : ∀ (v r : Std.U64) (k : Nat), r.val = v.val <<< k % U64.size →
        r.val = v.val * 2 ^ k % 2 ^ 64 := by
      intro v r k h
      have hs : U64.size = 2 ^ 64 := by simp [U64.size, U64.numBits]
      rw [h, Nat.shiftLeft_eq, hs]
    have shr : ∀ (v r : Std.U64) (k : Nat), r.val = v.val >>> k → r.val = v.val / 2 ^ k := by
      intro v r k h; rw [h, Nat.shiftRight_eq_div_pow]
    have c1 := upper_zero_post.trans (by rw [shr _ _ _ i_post1])
    have c2 := upper_zero1_post.trans (by rw [shr _ _ _ i2_post1])
    have c3 := upper_zero2_post.trans (by rw [shr _ _ _ i5_post1])
    have c4 := upper_zero3_post.trans (by rw [shr _ _ _ i8_post1])
    have c5 := upper_zero4_post.trans (by rw [shr _ _ _ i11_post1])
    have c6 := upper_zero5_post.trans (by rw [shr _ _ _ i14_post1])
    have v1 := sel _ _ _ _ val_post; rw [shl _ _ _ i1_post1] at v1
    have v2 := sel _ _ _ _ val1_post; rw [shl _ _ _ i4_post1] at v2
    have v3 := sel _ _ _ _ val2_post; rw [shl _ _ _ i7_post1] at v3
    have v4 := sel _ _ _ _ val3_post; rw [shl _ _ _ i10_post1] at v4
    have v5 := sel _ _ _ _ val4_post; rw [shl _ _ _ i13_post1] at v5
    have v6 := sel _ _ _ _ val5_post; rw [shl _ _ _ i16_post1] at v6
    obtain ⟨e1, l1, d1⟩ := clz_stage x.val x.val 0 32 (by decide) (by decide) (by simp) hx
      (if h : x.val = 0 then Or.inl h else Or.inr (Nat.pos_of_ne_zero h)) _ c1 _ v1
    obtain ⟨e2, l2, d2⟩ := clz_stage x.val _ _ 16 (by decide) (by decide) e1 l1 d1 _ c2 _ v2
    obtain ⟨e3, l3, d3⟩ := clz_stage x.val _ _ 8 (by decide) (by decide) e2 l2 d2 _ c3 _ v3
    obtain ⟨e4, l4, d4⟩ := clz_stage x.val _ _ 4 (by decide) (by decide) e3 l3 d3 _ c4 _ v4
    obtain ⟨e5, l5, d5⟩ := clz_stage x.val _ _ 2 (by decide) (by decide) e4 l4 d4 _ c5 _ v5
    obtain ⟨e6, l6, d6⟩ := clz_stage x.val _ _ 1 (by decide) (by decide) e5 l5 d5 _ c6 _ v6
    have t0 := sel32v _ _ _ _ n_post1
    have t1 := sel32v _ _ _ _ i3_post1
    have t2 := sel32v _ _ _ _ i6_post1
    have t3 := sel32v _ _ _ _ i9_post1
    have t4 := sel32v _ _ _ _ i12_post1
    have t5 := sel32v _ _ _ _ i15_post1
    have t6 := sel32v _ _ _ _ i18_post1
    have c7 := final_zero_post.trans (by rw [shr _ _ _ i17_post1])
    simp only [show (32#u32 : Std.U32).val = 32 from rfl, show (16#u32 : Std.U32).val = 16 from rfl,
      show (8#u32 : Std.U32).val = 8 from rfl, show (4#u32 : Std.U32).val = 4 from rfl,
      show (2#u32 : Std.U32).val = 2 from rfl, show (1#u32 : Std.U32).val = 1 from rfl,
      show (0#u32 : Std.U32).val = 0 from rfl] at t0 t1 t2 t3 t4 t5 t6
    have hn : n.val = (0 + (if (upper_zero != 0#u64) = true then 32 else 0) +
        (if (upper_zero1 != 0#u64) = true then 16 else 0) +
        (if (upper_zero2 != 0#u64) = true then 8 else 0) +
        (if (upper_zero3 != 0#u64) = true then 4 else 0) +
        (if (upper_zero4 != 0#u64) = true then 2 else 0) +
        (if (upper_zero5 != 0#u64) = true then 1 else 0)) +
        (if (final_zero != 0#u64) = true then 1 else 0) := by
      rw [n_post, t6, n5_post, t5, n4_post, t4, n3_post, t3, n2_post, t2, n1_post, t1, t0,
        Nat.zero_add]
    by_cases hx0 : x.val = 0
    · left
      refine ⟨hx0, ?_⟩
      have z1 : val.val = 0 := by rw [e1, hx0, Nat.zero_mul]
      have z2 : val1.val = 0 := by rw [e2, hx0, Nat.zero_mul]
      have z3 : val2.val = 0 := by rw [e3, hx0, Nat.zero_mul]
      have z4 : val3.val = 0 := by rw [e4, hx0, Nat.zero_mul]
      have z5 : val4.val = 0 := by rw [e5, hx0, Nat.zero_mul]
      have z6 : val5.val = 0 := by rw [e6, hx0, Nat.zero_mul]
      rw [if_pos (c1.mpr (by rw [hx0, Nat.zero_div])), if_pos (c2.mpr (by rw [z1, Nat.zero_div])),
        if_pos (c3.mpr (by rw [z2, Nat.zero_div])), if_pos (c4.mpr (by rw [z3, Nat.zero_div])),
        if_pos (c5.mpr (by rw [z4, Nat.zero_div])), if_pos (c6.mpr (by rw [z5, Nat.zero_div])),
        if_pos (c7.mpr (by rw [z6, Nat.zero_div]))] at hn
      exact hn
    · right
      rcases d6 with h0 | h63
      · exact absurd h0 hx0
      · have hf : ¬ ((final_zero != 0#u64) = true) := by
          rw [c7]; intro h
          exact absurd ((Nat.div_eq_zero_iff_lt (by positivity)).mp h) (Nat.not_lt.mpr h63)
        rw [if_neg hf, Nat.add_zero] at hn
        rw [hn, ← e6]
        exact ⟨hx0, h63, l6⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.CtSelectClz.ct_select_u64_selects
#print axioms NonosExtraction.CtSelectClz.ct_select_u32_selects
#print axioms NonosExtraction.CtSelectClz.ct_select_usize_selects
#print axioms NonosExtraction.CtSelectClz.ct_is_zero_u64_is_one_exactly_at_zero
#print axioms NonosExtraction.CtSelectClz.ct_clz_u64_counts_leading_zeros

end NonosExtraction.CtSelectClz
