// NONOS Operating System (AGPL-3.0-or-later)
//! The walk reads each CCSIDR_EL1 field at its full width, in both layouts.

use crate::walk::run;
use crate::walk_model::{check, expected, fixture};

#[test]
fn the_wide_layout_reaches_past_the_narrow_fields() {
    let (ctypes, shapes) = ([2, 4], [(6, 2048, 2), (6, 1, 40_000)]);
    let got = run(&fixture(&ctypes, 2, &shapes, true));
    assert_eq!(got, expected(&ctypes, 2, &shapes));
}

#[test]
fn every_field_is_read_at_its_full_width() {
    // The top bit of each field set: 1024 ways and 32768 sets narrow, more
    // than 2^20 ways wide.
    check(&[2, 2], 2, &[(6, 1024, 1), (6, 1, 32_768)]);
    let shapes = [(6, (1 << 20) + 1, 1)];
    assert_eq!(run(&fixture(&[2], 1, &shapes, true)), expected(&[2], 1, &shapes));
}
