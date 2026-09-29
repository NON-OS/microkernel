/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

iommu_regs_window, on the extracted code.

`probe_at` maps 4 KiB of a remapping unit's registers. Two register sets have
no fixed place: the IOTLB register sits at `IRO * 16 + 8`, with IRO in ECAP
bits 8 to 17, and the `NFR + 1` fault records start at `FRO * 16`, with FRO in
CAP bits 24 to 33. Both fields reach past 16 KiB. `invalidate_iotlb_global`
and `take_fault` wrote and read at those offsets, and `RemapUnit`'s accessors
checked the window only with `debug_assert!`, which release builds leave out,
so a unit reporting IRO or FRO of 256 or more got volatile MMIO past the
mapped page. `probe_at` now refuses such a unit with `registers_fit`, and the
accessors check the window in every build.

The theorems below read the three fields exactly, say `registers_fit` holds
exactly when both register sets end inside the window, and give the
consequence `probe_at` relies on: for an accepted unit, the IOTLB register and
every fault record from index 0 to NFR end inside the page.

What these cannot establish: that the unit's register set is no larger than
the page. The DRHD structure reports that size, and a unit whose registers
extend past 4 KiB is refused here rather than mapped in full.
-/

import NonosExtraction.IommuRegsWindow
import NonosExtraction.Bits

open Aeneas Aeneas.Std Result
open nonos_x_iommu_regs_window

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction.IommuRegsWindow

/-! ### The forwarding functions add nothing -/

theorem the_registers_fit_wrapper_is_its_method (a : Std.U64) (b : Std.U64) (c : Std.Usize) :
    registers_fit a b c = arch.x86_64.iommu.regs.window.registers_fit a b c := rfl

/-! ### The register fields and the window -/

private theorem shr_ok (v : Std.U64) (k : Std.I32) (h0 : 0 ≤ k.val) (h1 : k.val < 64) :
    ∃ z : Std.U64, v >>> k = ok z ∧ z.val = v.val / 2 ^ k.toNat := by
  obtain ⟨z, hz, hzv, -⟩ := WP.spec_imp_exists
    (UScalar.ShiftRight_IScalar_spec v k h0 (by simpa using h1))
  exact ⟨z, hz, by rw [hzv, Nat.shiftRight_eq_div_pow]⟩

private theorem to_usize (x : Std.U64) (h : x.val < 2 ^ 32) :
    (UScalar.cast .Usize x).val = x.val := by
  rw [UScalar.cast_val_eq]
  apply Nat.mod_eq_of_lt
  have : 32 ≤ UScalarTy.Usize.numBits := by
    simp only [UScalarTy.numBits]; rcases System.Platform.numBits_eq with h | h <;> omega
  exact Nat.lt_of_lt_of_le h (Nat.pow_le_pow_right (by decide) this)

/-- The fault-recording registers start at the FRO field, CAP bits 24 to 33,
    in units of sixteen bytes. -/
theorem fault_recording_offset_is_the_fro_field (cap : Std.U64) :
    ∃ n : Std.Usize, arch.x86_64.iommu.regs.cap.fault.fault_recording_offset cap = ok n ∧
      n.val = cap.val / 2 ^ 24 % 1024 * 16 := by
  unfold arch.x86_64.iommu.regs.cap.fault.fault_recording_offset
  obtain ⟨z, hz, hzv⟩ := shr_ok cap 24#i32 (by decide) (by decide)
  simp only [hz, lift, bind_tc_ok]
  have hf : (z &&& 1023#u64).val = cap.val / 2 ^ 24 % 1024 := by
    rw [Bits.land_low_mask z 1023#u64 10 rfl, hzv]; rfl
  have hc := to_usize (z &&& 1023#u64) (by rw [hf]; omega)
  obtain ⟨n, hn, hnv⟩ := WP.spec_imp_exists
    (Usize.mul_spec (x := UScalar.cast .Usize (z &&& 1023#u64)) (y := 16#usize) (by scalar_tac))
  exact ⟨n, hn, by rw [hnv, hc, hf]; rfl⟩

/-- The number of fault-recording registers is the NFR field, CAP bits 40 to
    47, plus one. -/
theorem fault_recording_count_is_nfr_plus_one (cap : Std.U64) :
    ∃ c : Std.U16, arch.x86_64.iommu.regs.cap.fault.fault_recording_count cap = ok c ∧
      c.val = cap.val / 2 ^ 40 % 256 + 1 := by
  unfold arch.x86_64.iommu.regs.cap.fault.fault_recording_count
  obtain ⟨z, hz, hzv⟩ := shr_ok cap 40#i32 (by decide) (by decide)
  simp only [hz, lift, bind_tc_ok]
  have hf : (z &&& 255#u64).val = cap.val / 2 ^ 40 % 256 := by
    rw [Bits.land_low_mask z 255#u64 8 rfl, hzv]; rfl
  have hc : (UScalar.cast .U16 (z &&& 255#u64)).val = cap.val / 2 ^ 40 % 256 := by
    rw [UScalar.cast_val_eq, hf]; simp only [UScalarTy.numBits]; omega
  obtain ⟨c, hcc, hcv⟩ := WP.spec_imp_exists
    (U16.add_spec (x := UScalar.cast .U16 (z &&& 255#u64)) (y := 1#u16) (by scalar_tac))
  exact ⟨c, hcc, by rw [hcv, hc]; rfl⟩

/-- The IOTLB register is eight bytes past the IRO field, ECAP bits 8 to 17, in
    units of sixteen bytes. -/
theorem iotlb_offset_is_eight_past_the_iro_field (ecap : Std.U64) :
    ∃ n : Std.Usize, arch.x86_64.iommu.regs.offsets.invalidate.iotlb_offset ecap = ok n ∧
      n.val = ecap.val / 2 ^ 8 % 1024 * 16 + 8 := by
  unfold arch.x86_64.iommu.regs.offsets.invalidate.iotlb_offset
    arch.x86_64.iommu.regs.offsets.invalidate.iva_offset
  obtain ⟨z, hz, hzv⟩ := shr_ok ecap 8#i32 (by decide) (by decide)
  simp only [hz, lift, bind_tc_ok]
  have hf : (z &&& 1023#u64).val = ecap.val / 2 ^ 8 % 1024 := by
    rw [Bits.land_low_mask z 1023#u64 10 rfl, hzv]; rfl
  have hc := to_usize (z &&& 1023#u64) (by rw [hf]; omega)
  obtain ⟨m, hm, hmv⟩ := WP.spec_imp_exists
    (Usize.mul_spec (x := UScalar.cast .Usize (z &&& 1023#u64)) (y := 16#usize) (by scalar_tac))
  simp only [hm, bind_tc_ok]
  obtain ⟨n, hn, hnv⟩ := WP.spec_imp_exists (Usize.add_spec (x := m) (y := 8#usize) (by scalar_tac))
  exact ⟨n, hn, by rw [hnv, hmv, hc, hf]; rfl⟩

/-- `registers_fit` holds exactly when the IOTLB register, eight bytes wide at
    `IRO * 16 + 8`, and all `NFR + 1` sixteen-byte fault records from
    `FRO * 16` end inside the window. -/
theorem registers_fit_is_both_register_sets_inside_the_window
    (cap ecap : Std.U64) (w : Std.Usize) :
    registers_fit cap ecap w =
      ok (decide (ecap.val / 2 ^ 8 % 1024 * 16 + 16 ≤ w.val ∧
        cap.val / 2 ^ 24 % 1024 * 16 + (cap.val / 2 ^ 40 % 256 + 1) * 16 ≤ w.val)) := by
  unfold registers_fit arch.x86_64.iommu.regs.window.registers_fit
  obtain ⟨o, ho, hov⟩ := fault_recording_offset_is_the_fro_field cap
  obtain ⟨c, hc, hcv⟩ := fault_recording_count_is_nfr_plus_one cap
  obtain ⟨t, ht, htv⟩ := iotlb_offset_is_eight_past_the_iro_field ecap
  simp only [ho, hc, ht, lift, bind_tc_ok]
  have hcu : (UScalar.cast .Usize c).val = c.val := by
    rw [UScalar.cast_val_eq]; apply Nat.mod_eq_of_lt
    have : 16 ≤ UScalarTy.Usize.numBits := by
      simp only [UScalarTy.numBits]; rcases System.Platform.numBits_eq with h | h <;> omega
    exact Nat.lt_of_lt_of_le c.hBounds (Nat.pow_le_pow_right (by decide) this)
  obtain ⟨m, hm, hmv⟩ := WP.spec_imp_exists
    (Usize.mul_spec (x := UScalar.cast .Usize c) (y := 16#usize) (by scalar_tac))
  simp only [hm, bind_tc_ok]
  obtain ⟨e, he, hev⟩ := WP.spec_imp_exists (Usize.add_spec (x := o) (y := m) (by scalar_tac))
  simp only [he, bind_tc_ok]
  obtain ⟨t8, ht8, ht8v⟩ := WP.spec_imp_exists (Usize.add_spec (x := t) (y := 8#usize) (by scalar_tac))
  simp only [ht8, bind_tc_ok]
  have hev' : e.val = cap.val / 2 ^ 24 % 1024 * 16 + (cap.val / 2 ^ 40 % 256 + 1) * 16 := by
    rw [hev, hov, hmv, hcu, hcv]; rfl
  have ht8v' : t8.val = ecap.val / 2 ^ 8 % 1024 * 16 + 16 := by rw [ht8v, htv]; rfl
  by_cases h1 : t8.val ≤ w.val
  · have : t8 ≤ w := h1
    simp only [this, ↓reduceIte]
    congr 1
    rw [← ht8v', ← hev']
    simp [h1]
  · have : ¬ t8 ≤ w := h1
    simp only [this, ↓reduceIte]
    congr 1
    rw [← ht8v']
    simp [h1]

/-- The consequence the probe relies on: once `registers_fit` accepts a unit for
    the 4 KiB window, the IOTLB register and every fault record the driver reads
    or clears, index `0` to `NFR`, end inside the mapped page. -/
theorem an_accepted_unit_keeps_every_register_access_in_the_page (cap ecap : Std.U64)
    (h : registers_fit cap ecap 4096#usize = ok true) :
    ecap.val / 2 ^ 8 % 1024 * 16 + 8 + 8 ≤ 4096 ∧
      ∀ i, i < cap.val / 2 ^ 40 % 256 + 1 →
        cap.val / 2 ^ 24 % 1024 * 16 + i * 16 + 16 ≤ 4096 := by
  rw [registers_fit_is_both_register_sets_inside_the_window] at h
  simp only [ok.injEq, decide_eq_true_eq] at h
  have hw : (4096#usize : Std.Usize).val = 4096 := rfl
  rw [hw] at h
  refine ⟨by omega, fun i hi => ?_⟩
  have : (i + 1) * 16 ≤ (cap.val / 2 ^ 40 % 256 + 1) * 16 := Nat.mul_le_mul_right _ hi
  omega

/-- At the page edge: IRO 255 and FRO 255 with one record end exactly at 4096
    and are accepted; IRO 256 or FRO 256, the first positions past the page, are
    refused. -/
theorem registers_fit_at_the_page_edge :
    registers_fit 0#u64 0xFF00#u64 4096#usize = ok true ∧
      registers_fit 0#u64 0x10000#u64 4096#usize = ok false ∧
      registers_fit 0xFF000000#u64 0#u64 4096#usize = ok true ∧
      registers_fit 0x100000000#u64 0#u64 4096#usize = ok false := by
  simp only [registers_fit_is_both_register_sets_inside_the_window]
  refine ⟨?_, ?_, ?_, ?_⟩ <;> rfl

/-! ### Axiom profile -/

#print axioms NonosExtraction.IommuRegsWindow.the_registers_fit_wrapper_is_its_method
#print axioms NonosExtraction.IommuRegsWindow.fault_recording_offset_is_the_fro_field
#print axioms NonosExtraction.IommuRegsWindow.fault_recording_count_is_nfr_plus_one
#print axioms NonosExtraction.IommuRegsWindow.iotlb_offset_is_eight_past_the_iro_field
#print axioms NonosExtraction.IommuRegsWindow.registers_fit_is_both_register_sets_inside_the_window
#print axioms NonosExtraction.IommuRegsWindow.an_accepted_unit_keeps_every_register_access_in_the_page
#print axioms NonosExtraction.IommuRegsWindow.registers_fit_at_the_page_edge

end NonosExtraction.IommuRegsWindow
