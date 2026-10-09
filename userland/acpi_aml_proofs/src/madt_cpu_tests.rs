// NONOS Operating System (AGPL-3.0-or-later)
//! MADT processor entries are kept or dropped by Linux's rules.

use crate::arch::x86_64::acpi::hw::madt_cpu::*;

#[test]
fn online_capable_counts_only_from_madt_revision_5() {
    assert!(processor_usable(MADT_FLAG_ENABLED, 1));
    assert!(processor_usable(MADT_FLAG_ONLINE_CAPABLE, 5));
    assert!(!processor_usable(MADT_FLAG_ONLINE_CAPABLE, 4), "bit 1 is reserved before 6.3");
    assert!(!processor_usable(0, 5));
}

#[test]
fn placeholder_ids_are_not_processors() {
    assert!(!keep_entry(0xFF, false, true));
    assert!(!keep_entry(0xFFFF_FFFF, true, false));
}

#[test]
fn small_id_x2apic_duplicates_are_dropped_when_xapic_entries_exist() {
    assert!(!keep_entry(3, true, true));
    assert!(keep_entry(3, true, false), "an x2APIC-only table keeps every ID");
    assert!(keep_entry(3, false, true));
}

#[test]
fn x2apic_ids_above_255_are_always_kept() {
    for id in [255u32, 256, 1024, 0x1_0000] {
        assert!(keep_entry(id, true, true), "id {id}");
    }
    assert!(needs_x2apic([0u32, 2, 4, 300]));
    assert!(!needs_x2apic(0u32..64));
}

#[test]
fn a_64_cpu_x2apic_table_with_mirrored_xapic_entries_yields_64_processors() {
    // Firmware listing CPUs 0..=63 as type 0 and again as type 9, plus 64
    // more as type 9 only with IDs from 256.
    let mut kept = alloc::vec::Vec::new();
    for id in 0u32..64 {
        if keep_entry(id, false, true) && !kept.contains(&id) {
            kept.push(id);
        }
        if keep_entry(id, true, true) && !kept.contains(&id) {
            kept.push(id);
        }
    }
    for id in 256u32..320 {
        if keep_entry(id, true, true) && !kept.contains(&id) {
            kept.push(id);
        }
    }
    assert_eq!(kept.len(), 128);
}
