// NONOS Operating System (AGPL-3.0-or-later)
//! The `MkAttestPolicy` record: the kernel's encoder against libc's parser.

use crate::kernel_policy_record::{encode, Tree, RECORD_LEN};
use crate::libc_policy_record::{parse_attest_policy, PolicyTree, ATTEST_POLICY_LEN};

fn kernel() -> Tree {
    Tree { root: [0x11; 32], epoch: 7, depth: 8 }
}

fn capsule() -> Tree {
    Tree { root: [0x22; 32], epoch: 9, depth: 8 }
}

fn seen(t: &Tree) -> PolicyTree {
    PolicyTree { root: t.root, epoch: t.epoch, depth: t.depth }
}

#[test]
fn the_two_sides_agree_on_the_length() {
    assert_eq!(RECORD_LEN, ATTEST_POLICY_LEN);
}

#[test]
fn both_trees_round_trip() {
    let r = encode(Some(&kernel()), Some(&capsule()));
    let p = parse_attest_policy(&r).expect("parses");
    assert_eq!(p.kernel, Some(seen(&kernel())));
    assert_eq!(p.capsule, Some(seen(&capsule())));
}

#[test]
fn an_absent_tree_reads_as_absent_not_as_a_root() {
    let r = encode(None, Some(&capsule()));
    let p = parse_attest_policy(&r).expect("parses");
    assert_eq!(p.kernel, None);
    assert_eq!(p.capsule, Some(seen(&capsule())));
}

/*
 * A zero root was never enrolled. The kernel must not report it as checked,
 * or a boot that skipped the gate would read as one that passed against zero.
 */
#[test]
fn a_zero_root_is_reported_as_unchecked() {
    let zero = Tree { root: [0; 32], epoch: 1, depth: 8 };
    let r = encode(Some(&zero), None);
    assert_eq!(r[1], 0);
    assert_eq!(parse_attest_policy(&r).expect("parses").kernel, None);
}

#[test]
fn a_set_flag_over_a_zero_root_is_refused() {
    let mut r = encode(None, None);
    r[1] = 1;
    assert!(parse_attest_policy(&r).is_none());
}

#[test]
fn a_value_under_a_clear_flag_is_refused() {
    let mut r = encode(None, None);
    r[8] = 1;
    assert!(parse_attest_policy(&r).is_none());
    let mut r = encode(None, None);
    r[70] = 1;
    assert!(parse_attest_policy(&r).is_none());
}

#[test]
fn another_version_an_unknown_flag_or_reserved_byte_is_refused() {
    let base = encode(Some(&kernel()), Some(&capsule()));
    for (at, v) in [(0usize, 2u8), (1, 0x04), (4, 1), (7, 1)] {
        let mut r = base;
        r[at] = v;
        assert!(parse_attest_policy(&r).is_none(), "byte {at} = {v:#x} accepted");
    }
}

#[test]
fn a_short_or_long_record_is_refused() {
    let r = encode(Some(&kernel()), None);
    assert!(parse_attest_policy(&r[..RECORD_LEN - 1]).is_none());
    let mut long = r.to_vec();
    long.push(0);
    assert!(parse_attest_policy(&long).is_none());
}
