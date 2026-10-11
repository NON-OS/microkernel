// NONOS Operating System (AGPL-3.0-or-later)
//! Event records, and the walk indices against the address they came from.

use crate::smmu::encode::event::{decode, name, Decoded};
use crate::smmu::encode::pte::{in_range, level_index};

#[test]
fn events_carry_kind_stream_and_faulting_address() {
    let bad_stream = [0x02 | 0x0123 << 32, 0, 0xdead_0000, 0];
    assert_eq!(decode(&bad_stream), Decoded { kind: 0x02, sid: 0x123, address: 0 });
    assert_eq!(name(0x02), b"C_BAD_STREAMID");
    let translation = [0x10 | 0x8 << 32, 0, 0x7f_c060_3000, 0];
    let d = decode(&translation);
    assert_eq!((d.kind, d.sid, d.address), (0x10, 8, 0x7f_c060_3000));
    assert_eq!(name(d.kind), b"F_TRANSLATION");
    assert_eq!(name(0x13), b"F_PERMISSION");
    assert_eq!(name(0xee), b"unknown");
}

#[test]
fn the_three_indices_rebuild_every_page_of_the_input_range() {
    // A stride through the 39-bit range that lands on every index value of
    // every level many times over.
    let mut iova = 0u64;
    while in_range(iova, 0x1000) {
        let (l1, l2, l3) = (level_index(iova, 1), level_index(iova, 2), level_index(iova, 3));
        assert!(l1 < 512 && l2 < 512 && l3 < 512);
        let rebuilt = (l1 as u64) << 30 | (l2 as u64) << 21 | (l3 as u64) << 12;
        assert_eq!(rebuilt, iova, "{iova:#x}");
        iova += 0x1000 * 4099;
    }
}
