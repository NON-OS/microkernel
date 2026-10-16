// NONOS Operating System (AGPL-3.0-or-later)
//! A walk of a modelled root, as the MMU would make it at stage 1 with a
//! 4 KiB granule and four levels (Arm ARM, DDI 0487, D8).

use std::collections::BTreeMap;

use crate::arch::aarch64::mmu::attributes::{PTE_ADDR_MASK, PTE_VALID};
use crate::arch::aarch64::mmu::geometry::{NO_EL0, NO_EL0_EXEC};

const TABLE: u64 = 1 << 1;
const PXN_TABLE: u64 = 1 << 59;

/// Every leaf under `root`: (virtual address, descriptor), with each table
/// descriptor on the way checked to keep EL0 out and EL1 able to run code.
pub fn leaves(root: &[u64], at: &BTreeMap<u64, Vec<u64>>) -> Vec<(u64, u64)> {
    fn walk(
        t: &[u64],
        level: u32,
        va: u64,
        at: &BTreeMap<u64, Vec<u64>>,
        out: &mut Vec<(u64, u64)>,
    ) {
        for (i, &w) in t.iter().enumerate().filter(|(_, w)| **w & PTE_VALID != 0) {
            let va = va | (i as u64) << (39 - 9 * level);
            if level == 3 {
                out.push((va, w));
                continue;
            }
            assert_ne!(w & TABLE, 0, "a block where only tables belong");
            assert_eq!(w & (NO_EL0 | NO_EL0_EXEC | PXN_TABLE), NO_EL0 | NO_EL0_EXEC);
            walk(&at[&(w & PTE_ADDR_MASK)], level + 1, va, at, out);
        }
    }
    let mut out = Vec::new();
    walk(root, 0, 0, at, &mut out);
    out
}
