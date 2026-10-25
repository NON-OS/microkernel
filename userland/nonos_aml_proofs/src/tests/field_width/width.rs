// NONOS Operating System (AGPL-3.0-or-later)
//! The model's access width (`model.rs` states the rule) and a bit mask.

use super::gen::Case;

pub fn width(c: &Case) -> u64 {
    if c.space == 3 {
        return 1;
    }
    let (first, last) = (u64::from(c.off), u64::from(c.off + c.bits - 1));
    match c.access {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        _ => [1, 2, 4, 8]
            .into_iter()
            .find(|w| first / (8 * w) == last / (8 * w) && (first / (8 * w) + 1) * w <= c.len)
            .unwrap_or(1),
    }
}

pub fn ones(n: u64) -> u128 {
    (1u128 << n) - 1
}
