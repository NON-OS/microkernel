// NONOS Operating System (AGPL-3.0-or-later)
//! The least bLength of each descriptor the readers take (USB 2.0 tables
//! 9-10, 9-12 and 9-13, the association descriptor of USB 3.2 table 9-16,
//! the SuperSpeed companion of table 9-27; as recalled, unverified).

use crate::sweep::build::{cs_least, IAD};

/// The least bLength of `d` by its type; 2 for one no reader takes.
pub fn least(d: &[u8]) -> usize {
    match d[1] {
        0x02 | 0x04 => 9,
        0x05 => 7,
        0x30 => 6,
        0x24 if d.len() >= 3 => cs_least(d[2]),
        0x24 => 3,
        IAD => 8,
        _ => 2,
    }
}

/// `d` holds every byte its type names.
pub fn whole(d: &[u8]) -> bool {
    d.len() >= least(d)
}
