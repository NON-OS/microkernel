/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

pid_inode, on the extracted code.

The crate root carries a forwarding function for the kernel's
`pid_dir_inode`, written by hand because its `Option` result is outside what
tools/extraction/mirror_crate.py forwards. A wrapper that quietly did
something other than call through would make every theorem about the function
a statement about code nobody runs, so it is proven to be the function below.
-/

import NonosExtraction.ProcfsPidInode

open Aeneas Aeneas.Std Result
open nonos_x_procfs_pid_inode

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction.ProcfsPidInode

/-! ### The forwarding functions add nothing -/

theorem the_pid_dir_inode_wrapper_is_its_method (a : Std.I32) :
    pid_dir_inode a = pid_inode.pid_dir_inode a := rfl

/-! ### The inode number of a pid directory

    `lookup_root` and `procfs_readdir` in `src/fs/procfs/inode.rs` number the
    directory of process `pid` as `pid * 1000 + 100`. They used to compute that
    as `pid as u64 * 1000 + 100` on a pid parsed from the name as an `i32`, so
    the name `-1` cast to `2^64 - 1` and overflowed the product: a panic in a debug
    build and a wrapped inode in a release one. `pid_dir_inode` refuses a
    negative pid, and the theorems below say it never fails, answers `none`
    exactly for a negative pid, gives every other pid the number
    `pid * 1000 + 100`, and gives distinct pids distinct numbers.

    What these cannot establish: that the number does not collide with the
    fixed root entries' inodes, which `procfs_root_entries` chooses and which
    are not extracted, or that the process exists. `lookup_root` also accepts
    only names made of ASCII digits, so `+5` no longer names process 5; that
    test is on a `str` and is not extracted either.
-/

/-- A negative pid has no directory inode. -/
theorem pid_dir_inode_refuses_a_negative_pid (p : Std.I32) (h : p.val < 0) :
    pid_dir_inode p = ok none := by
  unfold pid_dir_inode pid_inode.pid_dir_inode
  have : p < 0#i32 := by scalar_tac
  simp only [this, ↓reduceIte]

/-- A non-negative `i32` cast to `u64` keeps its value: the sign bit is clear,
    so sign extension is zero extension. -/
private theorem cast_of_nonneg (p : Std.I32) (h : 0 ≤ p.val) :
    (IScalar.hcast .U64 p).val = p.val.toNat := by
  have hmsb : p.bv.msb = false := by
    rw [BitVec.msb_eq_false_iff_two_mul_lt]
    have e : p.val = p.bv.toInt := rfl
    rw [BitVec.toInt_eq_toNat_cond] at e
    split at e <;> first | (simp_all; done) | (simp_all; omega)
  show (p.bv.signExtend 64).toNat = _
  rw [BitVec.signExtend_eq_setWidth_of_msb_false hmsb, BitVec.toNat_setWidth]
  have e : p.val = p.bv.toInt := rfl
  rw [BitVec.toInt_eq_toNat_of_msb hmsb] at e
  have : p.bv.toNat < 2 ^ 32 := p.bv.isLt
  rw [Nat.mod_eq_of_lt (by simp at *; omega)]
  omega

/-- Every other pid gets `pid * 1000 + 100`, and the computation cannot
    overflow: the largest `i32` gives a number far below `2^64`. -/
theorem pid_dir_inode_is_pid_times_1000_plus_100 (p : Std.I32) (h : 0 ≤ p.val) :
    ∃ n : Std.U64, pid_dir_inode p = ok (some n) ∧ n.val = p.val.toNat * 1000 + 100 := by
  unfold pid_dir_inode pid_inode.pid_dir_inode
  have hn : ¬ p < 0#i32 := by scalar_tac
  simp only [hn, ↓reduceIte, lift, bind_tc_ok]
  have hc := cast_of_nonneg p h
  have ⟨m, hm, hmv⟩ := WP.spec_imp_exists
    (U64.mul_spec (x := IScalar.hcast .U64 p) (y := 1000#u64) (by scalar_tac))
  simp only [hm, bind_tc_ok]
  have ⟨a, ha, hav⟩ := WP.spec_imp_exists
    (U64.add_spec (x := m) (y := 100#u64) (by scalar_tac))
  simp only [ha]
  exact ⟨a, rfl, by scalar_tac⟩

/-- `pid_dir_inode` never fails, and answers `none` exactly for a negative
    pid. -/
theorem pid_dir_inode_is_none_exactly_for_a_negative_pid (p : Std.I32) :
    ∃ r, pid_dir_inode p = ok r ∧ (r = none ↔ p.val < 0) := by
  by_cases h : p.val < 0
  · exact ⟨none, pid_dir_inode_refuses_a_negative_pid p h, by simp [h]⟩
  · obtain ⟨n, hn, -⟩ := pid_dir_inode_is_pid_times_1000_plus_100 p (by omega)
    exact ⟨some n, hn, by simp [h]⟩

/-- Distinct pids get distinct directory inodes, so a lookup by inode names one
    process. -/
theorem pid_dir_inode_separates_pids (p q : Std.I32) (n : Std.U64)
    (hp : pid_dir_inode p = ok (some n)) (hq : pid_dir_inode q = ok (some n)) : p = q := by
  have np : 0 ≤ p.val := by
    by_contra h
    rw [pid_dir_inode_refuses_a_negative_pid p (by omega)] at hp
    simp at hp
  have nq : 0 ≤ q.val := by
    by_contra h
    rw [pid_dir_inode_refuses_a_negative_pid q (by omega)] at hq
    simp at hq
  obtain ⟨a, ha, hav⟩ := pid_dir_inode_is_pid_times_1000_plus_100 p np
  obtain ⟨b, hb, hbv⟩ := pid_dir_inode_is_pid_times_1000_plus_100 q nq
  rw [ha] at hp; rw [hb] at hq
  simp only [ok.injEq, Option.some.injEq] at hp hq
  subst hp; subst hq
  apply IScalar.eq_of_val_eq
  omega

/-- Concrete edges: `-1` and the smallest `i32` have no inode, `0` is 100, and
    the largest `i32` is `2147483647100`. -/
theorem pid_dir_inode_at_the_edges :
    pid_dir_inode (-1)#i32 = ok none ∧ pid_dir_inode (-2147483648)#i32 = ok none ∧
      pid_dir_inode 0#i32 = ok (some 100#u64) ∧
      pid_dir_inode 2147483647#i32 = ok (some 2147483647100#u64) := by
  refine ⟨rfl, rfl, ?_, ?_⟩ <;> rfl

/-! ### Axiom profile -/

#print axioms NonosExtraction.ProcfsPidInode.the_pid_dir_inode_wrapper_is_its_method
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_refuses_a_negative_pid
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_is_pid_times_1000_plus_100
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_is_none_exactly_for_a_negative_pid
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_separates_pids
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_at_the_edges

end NonosExtraction.ProcfsPidInode
