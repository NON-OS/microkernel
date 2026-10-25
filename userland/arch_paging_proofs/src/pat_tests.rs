// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's IA32_PAT value: entry 1 write-combining, the rest as the
//! SDM's reset table, so a page's PAT, PCD and PWT bits pick WB, WC, UC- or UC.

#[path = "../../../src/arch/x86_64/pat/value.rs"]
mod value;

use value::*;

#[test]
fn reset_value_gains_wc_in_entry_one_only() {
    assert_eq!(with_wc(PAT_RESET), 0x0007_0406_0007_0106);
    for i in [0, 2, 3, 4, 5, 6, 7] {
        assert_eq!(entry(with_wc(PAT_RESET), i), entry(PAT_RESET, i));
    }
}

#[test]
fn on_any_table_firmware_left_only_entry_one_changes() {
    // The kernel writes with_wc of whatever the MSR holds, not the reset table.
    for p in [0u64, u64::MAX, 0x0407_0506_0007_0106, 0x0102_0304_0506_0704, PAT_RESET] {
        let w = with_wc(p);
        assert_eq!((w >> 8) & 0xFF, MT_WC as u64, "{p:#x}");
        assert_eq!(w & !0xFF00, p & !0xFF00, "{p:#x}");
        assert_eq!(with_wc(w), w, "{p:#x}");
    }
}

#[test]
fn page_bits_select_the_types_the_kernel_maps_with() {
    let pat = with_wc(PAT_RESET);
    assert_eq!(entry(pat, index(false, false, false)), MT_WB);
    assert_eq!(entry(pat, index(false, false, true)), MT_WC);
    assert_eq!(entry(pat, index(false, true, false)), MT_UC_MINUS);
    assert_eq!(entry(pat, index(false, true, true)), MT_UC);
    assert_eq!(index(false, false, true), WC_INDEX);
}

#[test]
fn reset_table_reads_as_the_sdm_lists_it() {
    let want = [MT_WB, MT_WT, MT_UC_MINUS, MT_UC, MT_WB, MT_WT, MT_UC_MINUS, MT_UC];
    for (i, t) in want.iter().enumerate() {
        assert_eq!(entry(PAT_RESET, i as u32), *t);
    }
}

#[test]
fn writing_twice_changes_nothing() {
    assert_eq!(with_wc(with_wc(PAT_RESET)), with_wc(PAT_RESET));
}
