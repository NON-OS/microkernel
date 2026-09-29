/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Bit facts the refinement files share.

Most flag readers in the kernel are `word & (1 << k) != 0`, and most field readers
are `word & mask` for a low mask. Aeneas lowers those to a `lift` of the scalar
`&&&` and a comparison, so the step from the extracted comparison to "bit `k` of
the word" is the same in every file. It is proven once here, over the scalar's
value, so each refinement file can say which bit a reader reads in one line.

Nothing in this file is about extracted code, and nothing here uses `bv_decide`:
each fact reduces to `Nat.testBit` and closes in the kernel.
-/

import Aeneas

open Aeneas Aeneas.Std

namespace NonosExtraction.Bits

/-- ANDing with a power of two keeps that one bit and nothing else. -/
theorem land_two_pow (n k : Nat) : n &&& 2 ^ k = if n.testBit k then 2 ^ k else 0 := by
  apply Nat.eq_of_testBit_eq
  intro i
  rw [Nat.testBit_and, Nat.testBit_two_pow]
  by_cases hik : k = i
  · subst hik
    cases n.testBit k <;> simp [Nat.testBit_two_pow_self]
  · cases n.testBit k <;> simp [hik]

/-- `word & m != 0` for a one-bit mask `m = 2^k` is bit `k` of the word. -/
theorem reads_bit {ty : UScalarTy} (e m z : UScalar ty) (k : Nat)
    (hm : m.val = 2 ^ k) (hz : z.val = 0) :
    ((e &&& m) != z) = e.val.testBit k := by
  have hv : (e &&& m).val = if e.val.testBit k then 2 ^ k else 0 := by
    rw [UScalar.val_and, hm, land_two_pow]
  cases h : e.val.testBit k
  · simp only [h, Bool.false_eq_true, ite_false] at hv
    have : (e &&& m) = z := UScalar.eq_of_val_eq (by rw [hv, hz])
    rw [this]
    simp
  · simp only [h, ite_true] at hv
    have hne : (e &&& m) ≠ z := by
      intro h0
      have := congrArg UScalar.val h0
      rw [hv, hz] at this
      simp at this
    simpa using hne

/-- `word & m == m` for a one-bit mask is the same bit, read the other way. -/
theorem reads_bit_eq {ty : UScalarTy} (e m : UScalar ty) (k : Nat) (hm : m.val = 2 ^ k) :
    decide ((e &&& m) = m) = e.val.testBit k := by
  have hv : (e &&& m).val = if e.val.testBit k then 2 ^ k else 0 := by
    rw [UScalar.val_and, hm, land_two_pow]
  cases h : e.val.testBit k
  · simp only [h, Bool.false_eq_true, ite_false] at hv
    have hne : (e &&& m) ≠ m := by
      intro h0
      have := congrArg UScalar.val h0
      rw [hv, hm] at this
      exact absurd this.symm (Nat.two_pow_pos k).ne'
    simpa using hne
  · simp only [h, ite_true] at hv
    simpa using (UScalar.eq_of_val_eq (by rw [hv, hm]) : (e &&& m) = m)

/-- ANDing with a low mask `2^n - 1` keeps the low `n` bits. -/
theorem land_low_mask {ty : UScalarTy} (e m : UScalar ty) (n : Nat) (hm : m.val = 2 ^ n - 1) :
    (e &&& m).val = e.val % 2 ^ n := by
  rw [UScalar.val_and, hm, Nat.and_two_pow_sub_one_eq_mod]

/-- A bit of `!word` is set exactly inside the word's width where the word's bit
    is clear. -/
theorem testBit_val_not {ty : UScalarTy} (x : UScalar ty) (i : Nat) :
    (~~~x).val.testBit i = (decide (i < ty.numBits) && !x.val.testBit i) := by
  show (~~~x.bv).toNat.testBit i = (decide (i < ty.numBits) && !x.bv.toNat.testBit i)
  rw [BitVec.testBit_toNat, BitVec.testBit_toNat, BitVec.getLsbD_not]

/-- A word's bits above its width are clear. -/
theorem testBit_val_high {ty : UScalarTy} (x : UScalar ty) (i : Nat) (hi : ty.numBits ≤ i) :
    x.val.testBit i = false := by
  apply Nat.testBit_eq_false_of_lt
  exact Nat.lt_of_lt_of_le x.bv.isLt (Nat.pow_le_pow_right (by decide) hi)

/-- A word is zero exactly when none of its bits is set. -/
theorem val_eq_zero_iff {ty : UScalarTy} (x : UScalar ty) :
    x.val = 0 ↔ ∀ i, x.val.testBit i = false := by
  constructor
  · intro h i
    rw [h, Nat.zero_testBit]
  · intro h
    apply Nat.eq_of_testBit_eq
    intro i
    rw [h i, Nat.zero_testBit]

/-- Clearing the low `k` bits with `word & !(2^k - 1)` rounds the word down to a
    multiple of `2^k`. -/
theorem land_not_low_mask {ty : UScalarTy} (x m : UScalar ty) (k : Nat)
    (hm : m.val = 2 ^ k - 1) :
    (x &&& ~~~m).val = x.val / 2 ^ k * 2 ^ k := by
  apply Nat.eq_of_testBit_eq
  intro i
  rw [UScalar.val_and, Nat.testBit_and, testBit_val_not, hm, Nat.testBit_two_pow_sub_one,
    Nat.testBit_mul_two_pow, Nat.testBit_div_two_pow]
  by_cases hik : k ≤ i
  · have hsub : i - k + k = i := Nat.sub_add_cancel hik
    rw [hsub]
    by_cases hw : i < ty.numBits
    · simp [hik, hw, Nat.not_lt.mpr hik]
    · rw [testBit_val_high x i (Nat.not_lt.mp hw)]
      simp
  · have hlt : i < k := Nat.lt_of_not_le hik
    simp [hik, hlt]

end NonosExtraction.Bits
