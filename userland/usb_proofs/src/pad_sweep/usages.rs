// NONOS Operating System (AGPL-3.0-or-later)
//! The usages a group may name, the one field `i` has (listed, else from a
//! range on one page, else the last listed), and the ones kept: buttons
//! 1 to 32 and desktop axes 30h to 39h.

use crate::rng::Rng;

pub type U = (u16, u16);

/// Logical ranges: bits, bytes, signed bytes and words, a hat's eight
/// directions from 0 or 1, one value, inverted, and one past signed.
pub const RANGES: [(i64, i64); 9] =
    [(0, 1), (0, 255), (-127, 127), (-32768, 32767), (0, 7), (1, 8), (0, 0), (5, 2), (-1, 6)];

pub fn usage_of(list: &[U], range: Option<(U, U)>, i: u32) -> Option<U> {
    if let Some(&u) = list.get(i as usize) {
        return Some(u);
    }
    if let Some(((page, lo), (hi_page, hi))) = range {
        return (hi_page == page && lo as u32 + i <= hi as u32).then(|| (page, lo + i as u16));
    }
    list.last().copied()
}

pub fn wanted((page, usage): U) -> bool {
    (page == 9 && (1..=32).contains(&usage)) || (page == 1 && (0x30..=0x39).contains(&usage))
}

pub fn pick_usage(r: &mut Rng, page: u16) -> u16 {
    match page {
        9 => r.pick(&[1, 2, 3, 8, 16, 31, 32, 33, 0]),
        1 => r.pick(&[0x30, 0x31, 0x32, 0x35, 0x39, 0x39, 0x2F, 0x3A]),
        _ => r.pick(&[0xE9, 0x30, 0x01]),
    }
}
