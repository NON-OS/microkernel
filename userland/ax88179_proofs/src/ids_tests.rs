// NONOS Operating System (AGPL-3.0-or-later)
//! The devices taken by vendor and product: every one Linux v6.12 lists,
//! and none it does not.

use crate::chip::{CONFIG, DEVICE};
use crate::seen::verdict;

/// Linux v6.12's table, as read there; Dell's adapters are not in it.
const LINUX_IDS: [(u16, u16); 13] = [
    (0x0b95, 0x1790),
    (0x0b95, 0x178a),
    (0x04b4, 0x3610),
    (0x2001, 0x4a00),
    (0x0df6, 0x0072),
    (0x04e8, 0xa100),
    (0x17ef, 0x304b),
    (0x050d, 0x0128),
    (0x0930, 0x0a13),
    (0x0711, 0x0179),
    (0x07c9, 0x000e),
    (0x07c9, 0x000f),
    (0x07c9, 0x0010),
];

fn with_ids(vendor: u16, product: u16) -> [u8; 18] {
    let mut d = DEVICE;
    d[8..10].copy_from_slice(&vendor.to_le_bytes());
    d[10..12].copy_from_slice(&product.to_le_bytes());
    d
}

#[test]
fn every_listed_device_binds_and_no_other() {
    for (v, p) in LINUX_IDS {
        assert_eq!(verdict(with_ids(v, p), vec![CONFIG.to_vec()]).0, "ours", "{v:04x}:{p:04x}");
    }
    for (v, p) in [(0x0b95, 0x7720), (0x0bda, 0x8153), (0x0b95, 0x1791), (0x0000, 0x1790)] {
        let (said, calls) = verdict(with_ids(v, p), vec![CONFIG.to_vec()]);
        assert_eq!((said, calls.len()), ("not ours", 0), "{v:04x}:{p:04x}");
    }
}
