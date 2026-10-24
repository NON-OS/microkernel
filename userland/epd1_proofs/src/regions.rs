// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's high-half layout, read from its constants' sources.

use crate::source::{number, LAYOUT};

/// Every high-half address the kernel's layout names, in TTBR1.
pub fn layout_addresses() -> Vec<u64> {
    let words =
        LAYOUT.iter().flat_map(|f| f.split(|c: char| !c.is_ascii_alphanumeric() && c != '_'));
    let mut out: Vec<u64> = words
        .filter(|w| w.starts_with("0x") || w.starts_with("0X"))
        .filter_map(number)
        .filter(|v| *v >= 0xFFFF_0000_0000_0000)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Every `NAME_BASE` with a `NAME_SIZE` beside it, as (start, end).
pub fn sized_regions() -> Vec<(u64, u64)> {
    let consts: std::collections::BTreeMap<&str, u64> = LAYOUT
        .iter()
        .flat_map(|f| f.lines())
        .filter_map(|l| l.trim().strip_prefix("pub const ")?.split_once(": u64 = "))
        .filter_map(|(name, v)| Some((name, number(v.trim_end_matches(';'))?)))
        .collect();
    let bases = consts.iter().filter_map(|(n, b)| Some((n.strip_suffix("_BASE")?, *b)));
    bases
        .filter_map(|(n, b)| Some((b, b.checked_add(*consts.get(&*format!("{n}_SIZE"))?)?)))
        .collect()
}
