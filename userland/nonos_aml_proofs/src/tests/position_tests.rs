// NONOS Operating System (AGPL-3.0-or-later)
//! What may stand where (section 20.2.5): objects and statements only in a
//! term list, names that refer where a SuperName stands, Debug only there.

use crate::enc::*;
use crate::error::ErrorKind;
use crate::flat::parse;
use crate::op::Op;

#[test]
fn a_statement_is_refused_as_an_operand() {
    let aml = cat(&[&[0x70], &[0xA3], &[LOCAL0]]);
    assert_eq!(parse(&aml).unwrap_err().kind, ErrorKind::Misplaced(0xA3));
    let aml = cat(&[&[0x72], &device("DEV0", &[]), &int(1), &[NULL]]);
    assert_eq!(parse(&aml).unwrap_err().kind, ErrorKind::Misplaced(0x5B82));
}

#[test]
fn a_constant_is_refused_as_a_target() {
    let aml = cat(&[&[0x70], &int(1), &int(2)]);
    assert_eq!(parse(&aml).unwrap_err().kind, ErrorKind::Misplaced(0x0A));
}

#[test]
fn debug_stands_only_where_a_supername_does() {
    let ok = cat(&[&[0x70], &int(1), &op(0x5B31)]);
    let (arena, root) = parse(&ok).unwrap();
    let store = arena.kids(root)[0];
    assert_eq!(arena.node(arena.kids(store)[1]).unwrap().op, Op::DEBUG);
    let bad = cat(&[&[0x72], &op(0x5B31), &int(1), &[NULL]]);
    assert_eq!(parse(&bad).unwrap_err().kind, ErrorKind::Misplaced(0x5B31));
}

#[test]
fn refof_derefof_and_index_are_supernames() {
    for target in [cat(&[&[0x71], &[LOCAL0]]), cat(&[&[0x88], &[LOCAL0], &int(1), &[NULL]])] {
        let aml = cat(&[&[0x70], &int(1), &target]);
        assert!(parse(&aml).is_ok());
    }
    let aml = cat(&[&[0x70], &int(1), &[0x72], &int(1), &int(1), &[NULL]]);
    assert_eq!(parse(&aml).unwrap_err().kind, ErrorKind::Misplaced(0x72));
}

#[test]
fn a_name_value_must_be_data() {
    assert!(parse(&name_obj("VAL0", &string("x"))).is_ok());
    let err = parse(&name_obj("VAL0", &name("VAL1"))).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Misplaced(Op::NAME_STRING.0));
    let err = parse(&name_obj("VAL0", &[LOCAL0])).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Misplaced(0x60));
}
