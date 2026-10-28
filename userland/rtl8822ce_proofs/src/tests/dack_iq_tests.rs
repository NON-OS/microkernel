// NONOS Operating System (AGPL-3.0-or-later)
//! The DAC calibration's sample arithmetic on 10-bit two's-complement
//! values: in range within 0x64 of zero; min and max and the spread as
//! rtw88 tracks them; sorted negatives first; the middle 80's mean.

use crate::phy::dack::iq::{check, delta, minmax, offset, SN};
use crate::phy::dack::sort::{out_of_order, sort};

#[test]
fn a_sample_is_in_range_within_0x64_of_zero() {
    for (v, ok) in [(0, true), (0x64, true), (0x65, false), (0x1FF, false)] {
        assert_eq!(check(v), ok, "{v:#x}");
    }
    for (v, ok) in [(0x3FF, true), (0x39C, true), (0x39B, false), (0x200, false)] {
        assert_eq!(check(v), ok, "{v:#x}");
    }
}

#[test]
fn min_and_max_are_signed() {
    let (mut min, mut max) = (0x10, 0x10);
    for v in [0x3F0, 0x20, 0x3FF, 0x05] {
        minmax(v, &mut min, &mut max);
    }
    assert_eq!((min, max), (0x3F0, 0x20), "-16 up to 32");
    assert_eq!(delta(0x3F0, 0x20), 0x30);
    assert_eq!(delta(0x05, 0x20), 0x1B);
    assert_eq!(delta(0x3F0, 0x3FF), 0x0F);
}

#[test]
fn the_sort_puts_negatives_first() {
    assert!(out_of_order(0x10, 0x3F0) && !out_of_order(0x3F0, 0x10));
    assert!(out_of_order(0x3FF, 0x3F0) && out_of_order(0x10, 0x05));
    let mut iv = [0u32; SN];
    let mut qv = [0u32; SN];
    for (i, v) in iv.iter_mut().enumerate() {
        *v = [0x02, 0x3FE, 0x00, 0x3F0][i % 4];
    }
    qv[0] = 5;
    sort(&mut iv, &mut qv);
    assert_eq!((iv[0], iv[25], iv[50], iv[75], iv[99]), (0x3F0, 0x3FE, 0x00, 0x02, 0x02));
    assert_eq!((qv[0], qv[99]), (0, 5));
}

#[test]
fn the_offset_is_the_middle_eightys_mean() {
    assert_eq!(offset(&[0; SN]), 0);
    assert_eq!(offset(&[2; SN]), 2);
    assert_eq!(offset(&[0x3FE; SN]), 0x3FE, "minus two");
    let mut v = [0x100; SN];
    v[..10].fill(0);
    v[90..].fill(0);
    assert_eq!(offset(&v), 0x100, "the ten at each end are not counted");
}
