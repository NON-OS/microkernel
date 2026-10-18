// NONOS Operating System (AGPL-3.0-or-later)
//! What a user root maps of the kernel once it carries the trampoline:
//! walked from the root, exactly the code page and the data page, both out
//! of EL0's reach, the code runnable at EL1 only and the data at neither.

use std::collections::BTreeMap;

use crate::arch::aarch64::mmu::attributes::*;
use crate::arch::aarch64::mmu::geometry::{chain, BASE, SLOT};
use crate::arch::aarch64::mmu::leaves::bits;
use crate::walk::leaves;

const L1: u64 = 0x4100_0000;
const L2: u64 = 0x4100_1000;
const L3: u64 = 0x4100_2000;
const CODE: u64 = 0x4020_3000;
const DATA: u64 = 0x4555_6000;

/// The tables a seeded user root reaches, by physical address.
fn tables(guarded: bool) -> (Vec<u64>, BTreeMap<u64, Vec<u64>>) {
    let c = chain([L1, L2, L3], CODE, DATA, bits(guarded));
    let mut root = vec![0u64; 512];
    root[SLOT] = c.root;
    let mut at = BTreeMap::new();
    for (pa, words) in
        [(L1, vec![(0, c.l1)]), (L2, vec![(0, c.l2)]), (L3, vec![(0, c.code), (1, c.data)])]
    {
        let mut t = vec![0u64; 512];
        words.into_iter().for_each(|(i, w)| t[i] = w);
        at.insert(pa, t);
    }
    (root, at)
}

#[test]
fn a_user_root_maps_two_pages_of_the_kernel_and_no_more() {
    for guarded in [false, true] {
        let (root, at) = tables(guarded);
        assert_eq!(root.iter().filter(|w| **w != 0).count(), 1);
        let leaves = leaves(&root, &at);
        let pages: Vec<_> = leaves.iter().map(|(va, w)| (*va, w & PTE_ADDR_MASK)).collect();
        assert_eq!(pages, [(BASE, CODE), (BASE + 0x1000, DATA)]);
        let (code, data) = (leaves[0].1, leaves[1].1);
        // Neither page is EL0's nor writable: AP 0b10, never run at EL0.
        for w in [code, data] {
            assert_eq!(w & 0b11 << 6, PTE_AP_RO_EL1);
            assert_ne!(w & PTE_UXN, 0);
            assert_ne!(w & PTE_AF, 0);
            assert_eq!(w & PTE_NG, 0, "global, as every kernel mapping is");
        }
        assert_eq!(code & PTE_PXN, 0, "the code runs at EL1");
        assert_eq!(code & PTE_GP != 0, guarded);
        assert_ne!(data & PTE_PXN, 0, "the data runs nowhere");
    }
}
