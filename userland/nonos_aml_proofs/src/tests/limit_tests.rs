// NONOS Operating System (AGPL-3.0-or-later)
//! The arena's bounds: nesting, node count and table size, each exact at its
//! edge.

use crate::arena::{Arena, Limits};
use crate::block::Block;
use crate::enc::*;
use crate::error::ErrorKind;
use crate::flat::{parse, parse_with, Flat};
use crate::parse::load;

fn arena(max_depth: u32, max_nodes: usize, max_table: usize) -> Arena {
    Arena::new(Limits { max_table, max_depth, max_nodes })
}

fn nested_scopes(n: usize) -> Vec<u8> {
    (0..n).fold(Vec::new(), |inner, _| scope("SCP0", &inner))
}

fn nested_adds(n: usize) -> Vec<u8> {
    (0..n).fold(int(1), |inner, _| cat(&[&[0x72], &inner, &int(1), &[NULL]]))
}

#[test]
fn nesting_stops_exactly_at_the_limit() {
    let mut flat = Flat::default();
    assert!(parse_with(&nested_scopes(8), &mut flat, arena(8, 1 << 20, 1 << 20)).is_ok());
    assert_eq!(flat.deepest, 8);
    let err = parse_with(&nested_scopes(9), &mut flat, arena(8, 1 << 20, 1 << 20)).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Depth);
    assert_eq!(flat.depth, 0, "every scope entered was left");
    // Seven Adds and the innermost One are eight levels.
    assert!(parse_with(&nested_adds(7), &mut flat, arena(8, 1 << 20, 1 << 20)).is_ok());
    let err = parse_with(&nested_adds(8), &mut flat, arena(8, 1 << 20, 1 << 20)).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Depth);
}

#[test]
fn the_default_depth_refuses_a_recursion_crafted_table() {
    assert_eq!(parse(&nested_scopes(64)).map(|_| ()), Ok(()));
    assert_eq!(parse(&nested_scopes(65)).unwrap_err().kind, ErrorKind::Depth);
    assert_eq!(parse(&nested_adds(2_000)).unwrap_err().kind, ErrorKind::Depth);
}

#[test]
fn node_count_stops_exactly_at_the_limit() {
    let aml = nested_scopes(5);
    let (whole, _) = parse(&aml).unwrap();
    let n = whole.len();
    assert!(parse_with(&aml, &mut Flat::default(), arena(64, n, 1 << 20)).is_ok());
    let err = parse_with(&aml, &mut Flat::default(), arena(64, n - 1, 1 << 20)).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Nodes);
}

#[test]
fn a_table_past_the_size_limit_is_refused() {
    let t = table(b"DSDT", 2, &[0xA3; 100]);
    let block = Block::new(&t, usize::MAX).unwrap();
    assert!(load(&block, &mut arena(64, 1 << 20, t.len()), &mut Flat::default()).is_ok());
    let err = load(&block, &mut arena(64, 1 << 20, t.len() - 1), &mut Flat::default());
    assert_eq!(err.unwrap_err().kind, ErrorKind::Size);
    assert_eq!(Block::new(&t, t.len() - 1).unwrap_err().kind, ErrorKind::Size);
}
