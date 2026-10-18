// NONOS Operating System (AGPL-3.0-or-later)
//! The layout by hand: the boot CPU first wherever the tree lists it, the
//! others in tree order, gaps in MPIDR kept as they are, a duplicate once,
//! a full roster cut after the boot CPU, and a foreign list refused.

use crate::roster_order::arrange;

fn run(tree: &[u64], own: u64, capacity: usize) -> (Vec<u64>, Vec<Option<usize>>) {
    let (mut dense, mut at) = (vec![0; capacity], vec![None; capacity]);
    let n = arrange(tree, own, &mut dense, &mut at);
    (dense[..n].to_vec(), at[..n].to_vec())
}

#[test]
fn the_boot_cpu_comes_first_and_the_rest_keep_tree_order() {
    let tree = [0x0, 0x1, 0x100, 0x101];
    assert_eq!(run(&tree, 0x0, 8), (tree.to_vec(), vec![Some(0), Some(1), Some(2), Some(3)]));
    let (dense, at) = run(&tree, 0x100, 8);
    assert_eq!(dense, vec![0x100, 0x0, 0x1, 0x101]);
    assert_eq!(at, vec![Some(2), Some(0), Some(1), Some(3)]);
}

#[test]
fn gaps_and_aff3_are_kept_and_a_duplicate_counts_once() {
    let tree = [0x1_0000_0000, 0x3, 0x3, 0x10_0200];
    let (dense, at) = run(&tree, 0x3, 8);
    assert_eq!(dense, vec![0x3, 0x1_0000_0000, 0x10_0200]);
    assert_eq!(at, vec![Some(1), Some(0), Some(3)]);
}

#[test]
fn a_full_roster_keeps_the_boot_cpu_wherever_it_was_listed() {
    let tree: Vec<u64> = (0..10).map(|i| i << 8).collect();
    let (dense, at) = run(&tree, 9 << 8, 4);
    assert_eq!(dense, vec![9 << 8, 0, 1 << 8, 2 << 8]);
    assert_eq!(at, vec![Some(9), Some(0), Some(1), Some(2)]);
}

#[test]
fn a_list_without_the_boot_cpu_leaves_it_alone() {
    assert_eq!(run(&[0x1, 0x2], 0x0, 8), (vec![0x0], vec![None]));
    assert_eq!(run(&[], 0x7, 8), (vec![0x7], vec![None]));
    assert_eq!(run(&[0x7], 0x7, 0), (vec![], vec![]));
}
