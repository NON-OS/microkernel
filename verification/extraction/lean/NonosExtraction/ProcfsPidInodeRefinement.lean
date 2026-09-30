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

    Every inode under a pid directory is `(pid << 20) | k`: the directory is
    `k = 0` and its entries are `k = 1` to `103` (`pid_entries`), and the root
    entries all sit below `2 ^ 20` (`procfs_root_entries`, 105 at most).
    `lookup_root` and `procfs_readdir` take the directory's number from
    `pid_dir_inode`, which answers `none` for a pid of zero or below.

    The numbering used to be `pid as u64 * 1000 + 100` on a pid parsed as an
    `i32`. The name `-1` overflowed that product, a panic in a debug build and
    a wrapped inode in a release one; pid 0's directory got 100, the root
    `sys` entry's inode, and pid 0's `status` entry got 1, which
    `procfs_lookup` dispatches as the root; and pid 131072's directory got
    131072100, the inode of pid 125's `task` entry.

    The theorems below say `pid_dir_inode` never fails, answers `none` exactly
    for a pid of zero or below, gives every other pid `pid * 2 ^ 20`, and that
    this number is at least `2 ^ 20`, so above every root entry, and is no
    entry inode `(q << 20) | k` with `1 ≤ k < 2 ^ 20` of any pid.

    What these cannot establish: that the process exists. `pid_entries` and
    `procfs_root_entries` build `Vec`s and are not extracted, so their inode
    numbers are taken from the source as stated above. `lookup_root` also
    accepts only names made of ASCII digits, so `+5` no longer names process 5;
    that test is on a `str` and is not extracted either.
-/

/-- A pid of zero or below has no directory inode. -/
theorem pid_dir_inode_refuses_a_pid_below_one (p : Std.I32) (h : p.val ≤ 0) :
    pid_dir_inode p = ok none := by
  unfold pid_dir_inode pid_inode.pid_dir_inode
  have : p <= 0#i32 := by scalar_tac
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

/-- Every pid from 1 gets `pid * 2 ^ 20`, and the shift cannot lose bits: the
    largest `i32` gives a number below `2 ^ 51`. -/
theorem pid_dir_inode_is_pid_shifted_by_twenty (p : Std.I32) (h : 0 < p.val) :
    ∃ n : Std.U64, pid_dir_inode p = ok (some n) ∧ n.val = p.val.toNat * 2 ^ 20 := by
  unfold pid_dir_inode pid_inode.pid_dir_inode pid_inode.PID_INODE_SHIFT
  have hn : ¬ p <= 0#i32 := by scalar_tac
  simp only [hn, ↓reduceIte, lift, bind_tc_ok]
  have hc := cast_of_nonneg p (by omega)
  have hp : p.val.toNat < 2 ^ 31 := by scalar_tac
  obtain ⟨z, hz, hzv, -⟩ := WP.spec_imp_exists
    (UScalar.ShiftLeft_spec (IScalar.hcast .U64 p) 20#u32 (2 ^ 64) (by decide)
      (by simp [U64.size, U64.numBits]))
  simp only [hz, bind_tc_ok]
  refine ⟨z, rfl, ?_⟩
  rw [hzv, hc, Nat.shiftLeft_eq, show (20#u32 : Std.U32).val = 20 from rfl]
  exact Nat.mod_eq_of_lt (by omega)

/-- `pid_dir_inode` never fails, and answers `none` exactly for a pid of zero
    or below. -/
theorem pid_dir_inode_is_none_exactly_below_one (p : Std.I32) :
    ∃ r, pid_dir_inode p = ok r ∧ (r = none ↔ p.val ≤ 0) := by
  by_cases h : p.val ≤ 0
  · exact ⟨none, pid_dir_inode_refuses_a_pid_below_one p h, by simp [h]⟩
  · obtain ⟨n, hn, -⟩ := pid_dir_inode_is_pid_shifted_by_twenty p (by omega)
    exact ⟨some n, hn, by simp [h]⟩

/-- A directory inode is at least `2 ^ 20`, above every root entry, and is not
    the inode `(q << 20) | k` of any entry, `1 ≤ k < 2 ^ 20`, of any pid `q`. -/
theorem a_pid_directory_inode_is_no_root_or_entry_inode (p : Std.I32) (n : Std.U64)
    (h : pid_dir_inode p = ok (some n)) :
    2 ^ 20 ≤ n.val ∧ ∀ q k : Nat, 1 ≤ k → k < 2 ^ 20 → n.val ≠ q * 2 ^ 20 + k := by
  have hp : 0 < p.val := by
    by_contra hc
    rw [pid_dir_inode_refuses_a_pid_below_one p (by omega)] at h
    simp at h
  obtain ⟨m, hm, hmv⟩ := pid_dir_inode_is_pid_shifted_by_twenty p hp
  rw [hm] at h
  simp only [ok.injEq, Option.some.injEq] at h
  subst h
  refine ⟨by rw [hmv]; omega, fun q k hk1 hk2 heq => ?_⟩
  rw [hmv] at heq
  omega

/-- Distinct pids get distinct directory inodes, so a lookup by inode names one
    process. -/
theorem pid_dir_inode_separates_pids (p q : Std.I32) (n : Std.U64)
    (hp : pid_dir_inode p = ok (some n)) (hq : pid_dir_inode q = ok (some n)) : p = q := by
  have np : 0 < p.val := by
    by_contra h
    rw [pid_dir_inode_refuses_a_pid_below_one p (by omega)] at hp
    simp at hp
  have nq : 0 < q.val := by
    by_contra h
    rw [pid_dir_inode_refuses_a_pid_below_one q (by omega)] at hq
    simp at hq
  obtain ⟨a, ha, hav⟩ := pid_dir_inode_is_pid_shifted_by_twenty p np
  obtain ⟨b, hb, hbv⟩ := pid_dir_inode_is_pid_shifted_by_twenty q nq
  rw [ha] at hp; rw [hb] at hq
  simp only [ok.injEq, Option.some.injEq] at hp hq
  subst hp; subst hq
  apply IScalar.eq_of_val_eq
  omega

/-- Concrete edges: `-1` and `0` have no inode, `1` is `2 ^ 20`, and pid
    131072, which used to share pid 125's `task` inode, is `2 ^ 37`. -/
theorem pid_dir_inode_at_the_edges :
    pid_dir_inode (-1)#i32 = ok none ∧ pid_dir_inode 0#i32 = ok none ∧
      pid_dir_inode 1#i32 = ok (some 0x100000#u64) ∧
      pid_dir_inode 131072#i32 = ok (some 0x2000000000#u64) := by
  refine ⟨rfl, rfl, ?_, ?_⟩
  · obtain ⟨n, hn, hv⟩ := pid_dir_inode_is_pid_shifted_by_twenty 1#i32 (by decide)
    rw [hn, UScalar.eq_of_val_eq (hv.trans rfl : n.val = (0x100000#u64 : Std.U64).val)]
  · obtain ⟨n, hn, hv⟩ := pid_dir_inode_is_pid_shifted_by_twenty 131072#i32 (by decide)
    rw [hn, UScalar.eq_of_val_eq (hv.trans rfl : n.val = (0x2000000000#u64 : Std.U64).val)]

/-! ### Axiom profile -/

#print axioms NonosExtraction.ProcfsPidInode.the_pid_dir_inode_wrapper_is_its_method
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_refuses_a_pid_below_one
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_is_pid_shifted_by_twenty
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_is_none_exactly_below_one
#print axioms NonosExtraction.ProcfsPidInode.a_pid_directory_inode_is_no_root_or_entry_inode
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_separates_pids
#print axioms NonosExtraction.ProcfsPidInode.pid_dir_inode_at_the_edges

end NonosExtraction.ProcfsPidInode
