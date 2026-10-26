// NONOS Operating System (AGPL-3.0-or-later)
//! Free RAM from a NONOS loader's EFI map: only Conventional memory, page
//! aligned inward, neighbours merged, and a map entry that wraps left out.
//! Entries are (type, start, pages, attributes); `e` gives write-back.

use crate::efi_runs::{from_efi, CONVENTIONAL, SP, WB};

const LOADER_DATA: u32 = 2;
const BOOT_SERVICES_DATA: u32 = 4;
const RUNTIME_DATA: u32 = 6;

fn e(ty: u32, start: u64, pages: u64) -> (u32, u64, u64, u64) {
    (ty, start, pages, WB | 0x7)
}

#[test]
fn only_conventional_memory_is_free() {
    // A map shaped like AAVMF's on QEMU virt: the loader's data and the
    // runtime services between free runs, boot services data beside them.
    let map = [
        e(CONVENTIONAL, 0x4000_0000, 0x100),
        e(LOADER_DATA, 0x4010_0000, 0x5800),
        e(CONVENTIONAL, 0x4590_0000, 0x3_0000),
        e(BOOT_SERVICES_DATA, 0x7590_0000, 0x100),
        e(RUNTIME_DATA, 0x75A0_0000, 0x20),
        e(CONVENTIONAL, 0x75A2_0000, 0x10),
        e(CONVENTIONAL, 0x75A3_0000, 0x10),
    ];
    let u = from_efi(map.into_iter());
    assert_eq!(
        u.runs(),
        [(0x4000_0000, 0x4010_0000), (0x4590_0000, 0x7590_0000), (0x75A2_0000, 0x75A4_0000)]
    );
    assert_eq!(u.largest(), Some((0x4590_0000, 0x7590_0000)));
}

#[test]
fn every_other_type_is_kept() {
    let all = (0..16u32).filter(|&t| t != CONVENTIONAL).map(|t| e(t, u64::from(t) << 24, 4));
    assert!(from_efi(all).runs().is_empty());
}

#[test]
fn an_unaligned_entry_is_cut_inward() {
    let u = from_efi([e(CONVENTIONAL, 0x1_0010, 2)].into_iter());
    assert_eq!(u.runs(), [(0x1_1000, 0x1_2000)]);
}

#[test]
fn an_entry_past_the_top_is_left_out() {
    let wraps = [e(CONVENTIONAL, u64::MAX - 0x1000, 4), e(CONVENTIONAL, 0x1000, u64::MAX)];
    assert!(from_efi(wraps.into_iter()).runs().is_empty());
}

#[test]
fn only_write_back_memory_not_set_aside_is_free() {
    let map = [
        (CONVENTIONAL, 0x1000_0000, 0x10, 0x1),
        (CONVENTIONAL, 0x2000_0000, 0x10, WB | SP),
        (CONVENTIONAL, 0x3000_0000, 0x10, WB),
    ];
    assert_eq!(from_efi(map.into_iter()).runs(), [(0x3000_0000, 0x3001_0000)]);
}
