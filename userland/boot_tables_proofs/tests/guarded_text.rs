// NONOS Operating System (AGPL-3.0-or-later)
//! With BTI on, GP is set on the image's text and on nothing else: every
//! page EL1 can run is guarded, and no page it cannot run claims to be (Arm
//! ARM, DDI 0487, FEAT_BTI, the GP bit of a stage 1 descriptor).

use boot_tables_proofs::leaves::{leaves, GP, PXN};
use boot_tables_proofs::probe::{self, GUARDED, TEXT};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};
use core::sync::atomic::Ordering;

#[test]
fn only_text_is_guarded_and_all_of_it_is() {
    GUARDED.store(true, Ordering::Relaxed);
    probe::build(&BootInfo {
        regions: vec![MemoryRegion {
            base: 0x4000_0000,
            size: 0x4000_0000,
            region_type: MemoryType::Available,
        }],
        uart: Window { base: 0x0900_0000, size: 0x1000 },
        gic_dist_base: 0x0800_0000,
        gic_redist_base: 0x080a_0000,
        ..BootInfo::default()
    });
    let mut guarded = 0;
    for leaf in leaves() {
        let in_text = (TEXT.0..TEXT.1).contains(&leaf.va);
        assert_eq!(leaf.descriptor & GP != 0, in_text, "{:#x}", leaf.va);
        assert_eq!(leaf.descriptor & PXN == 0, in_text, "{:#x}", leaf.va);
        guarded += usize::from(in_text);
    }
    assert_eq!(guarded as u64, (TEXT.1 - TEXT.0) / 4096);
}
