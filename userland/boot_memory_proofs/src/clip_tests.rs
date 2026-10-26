// NONOS Operating System (AGPL-3.0-or-later)
//! Free runs held to the tree's RAM and below the end of the direct map: a
//! run the EFI map names past either is cut, never handed out whole.

use crate::clip::within;
use crate::usable::Usable;

fn free(runs: &[(u64, u64)]) -> Usable {
    let mut u = Usable::EMPTY;
    runs.iter().for_each(|&(a, b)| u.add(a, b));
    u
}

#[test]
fn a_run_past_the_trees_ram_is_cut_to_it() {
    // The tree caps RAM at 3 GiB above 1 GiB; the EFI map names 4 GiB.
    let got = within(
        &free(&[(0x4000_0000, 0x1_4000_0000)]),
        [(0x4000_0000, 0x1_0000_0000)].into_iter(),
        u64::MAX,
    );
    assert_eq!(got.runs(), [(0x4000_0000, 0x1_0000_0000)]);
}

#[test]
fn a_run_across_two_regions_keeps_only_their_parts() {
    let ram = [(0x4000_0000, 0x5000_0000), (0x6000_0000, 0x7000_0000)];
    let got = within(&free(&[(0x4800_0000, 0x6800_0000)]), ram.into_iter(), u64::MAX);
    assert_eq!(got.runs(), [(0x4800_0000, 0x5000_0000), (0x6000_0000, 0x6800_0000)]);
}

#[test]
fn nothing_past_the_direct_map_is_kept() {
    let ram = [(0x4000_0000, 0x2_0000_0000)];
    let got = within(
        &free(&[(0x4000_0000, 0x8000_0000), (0x1_8000_0000, 0x2_0000_0000)]),
        ram.into_iter(),
        0x1_0000_0000,
    );
    assert_eq!(got.runs(), [(0x4000_0000, 0x8000_0000)]);
    assert_eq!(got.largest(), Some((0x4000_0000, 0x8000_0000)), "the largest that is reachable");
}

#[test]
fn unaligned_tree_bounds_are_cut_inward() {
    let got = within(&free(&[(0x1000, 0x10_0000)]), [(0x1800, 0x8_0800)].into_iter(), u64::MAX);
    assert_eq!(got.runs(), [(0x2000, 0x8_0000)]);
}
