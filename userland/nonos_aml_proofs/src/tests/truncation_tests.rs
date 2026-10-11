// NONOS Operating System (AGPL-3.0-or-later)
//! Truncation at every byte. The catalogue is cut at each length and wrapped
//! in a Scope whose PkgLength fits the cut, so the cut lands inside whatever
//! object it hits: the parse must succeed exactly where the cut falls between
//! two whole objects, and be refused everywhere else.

use super::catalog::laid_out;
use crate::arena::{Arena, Imm};
use crate::block::Block;
use crate::enc::*;
use crate::flat::{parse, Flat};
use crate::op::Op;
use crate::parse::{body, load};

#[test]
fn every_cut_of_the_catalogue_is_refused_unless_it_falls_between_objects() {
    let (aml, cases) = laid_out();
    let mut whole: Vec<usize> = cases.iter().map(|(at, _)| *at).collect();
    whole.push(aml.len());
    for cut in 0..=aml.len() {
        let r = parse(&scope("\\_SB", &aml[..cut]));
        assert_eq!(r.is_ok(), whole.contains(&cut), "cut at {cut}: {:?}", r.err());
    }
}

#[test]
fn a_table_shorter_than_its_header_says_is_refused() {
    let t = table(b"DSDT", 2, &scope("\\_SB", &[0xA3]));
    for cut in 0..t.len() {
        assert!(Block::new(&t[..cut], usize::MAX).is_err(), "cut at {cut}");
    }
}

#[test]
fn every_cut_of_a_method_body_is_refused_unless_it_falls_between_statements() {
    let stmts = [
        cat(&[&[0x70], &int(0x1234), &[LOCAL0]]),
        cat(&[&if_(&cat(&[&[0x93], &[ARG0], &int(1)]), &[0xA3]), &else_(&[0xA5])]),
        cat(&[&[0xA4], &string("done")]),
    ];
    let src = stmts.concat();
    let mut whole = vec![0];
    for s in &stmts {
        whole.push(whole.last().unwrap() + s.len());
    }
    // The If without its Else is also a whole statement.
    whole.push(stmts[0].len() + if_(&cat(&[&[0x93], &[ARG0], &int(1)]), &[0xA3]).len());
    for cut in 0..=src.len() {
        let t = table(b"SSDT", 2, &method("MTH0", 1, &src[..cut]));
        let block = Block::new(&t, usize::MAX).unwrap();
        let mut arena = Arena::default();
        let mut flat = Flat::default();
        let root = load(&block, &mut arena, &mut flat).unwrap();
        let m = arena.kids(arena.kids(root)[0])[2];
        let Imm::Bytes(span) = arena.node(m).unwrap().imm else { panic!("deferred body") };
        assert_eq!(arena.node(m).unwrap().op, Op::DEFERRED);
        let before = arena.len();
        let r = body(&block, &mut arena, &mut flat, span);
        assert_eq!(r.is_ok(), whole.contains(&cut), "cut at {cut}: {:?}", r.err());
        if r.is_err() {
            assert_eq!(arena.len(), before, "a failed body leaves nothing behind");
        }
    }
}
