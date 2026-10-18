// NONOS Operating System (AGPL-3.0-or-later)
//! Method calls: the argument count comes from the method's definition or an
//! External (section 20.2.5.4, MethodInvocation), and a name the namespace
//! does not know is a name.

use crate::enc::*;
use crate::error::ErrorKind;
use crate::flat::{parse, parse_with, Flat};
use crate::op::Op;

fn call(path: &str, args: &[&[u8]]) -> Vec<u8> {
    cat(&[&name(path), &args.concat()])
}

#[test]
fn a_declared_method_takes_its_arguments() {
    let aml = cat(&[
        &scope("\\_SB", &method("MTH2", 2, &[0xA3])),
        &if_(&call("\\_SB.MTH2", &[&int(1), &[LOCAL0]]), &[0xA3]),
    ]);
    let (arena, root) = parse(&aml).unwrap();
    let pred = arena.kids(arena.kids(root)[1])[0];
    assert_eq!(arena.node(pred).unwrap().op, Op::NAME_STRING);
    assert_eq!(arena.kids(pred).len(), 2);
}

#[test]
fn an_external_declares_a_method_in_another_table() {
    let store = cat(&[&[0x70], &call("EXT1", &[&int(5)]), &[LOCAL0]]);
    assert!(parse(&cat(&[&external_method("\\EXT1", 1), &store])).is_ok());
    // Without the External the 5 would be read as Store's target.
    assert_eq!(parse(&store).unwrap_err().kind, ErrorKind::Misplaced(0x0A));
}

#[test]
fn a_name_in_a_supername_is_never_called() {
    let aml = cat(&[
        &method("MTH1", 1, &[0xA3]),
        &if_(&cat(&[&op(0x5B12), &name("MTH1"), &[NULL]]), &[0xA3]),
    ]);
    let (arena, root) = parse(&aml).unwrap();
    let cond = arena.kids(arena.kids(root)[1])[0];
    let operand = arena.kids(cond)[0];
    assert_eq!(arena.kids(operand).len(), 0, "CondRefOf names MTH1, it does not call it");
}

#[test]
fn scopes_are_entered_and_left_in_step() {
    let mut flat = Flat::default();
    let aml = scope("\\_SB", &device("PCI0", &device("LPCB", &[])));
    parse_with(&aml, &mut flat, Default::default()).unwrap();
    assert_eq!((flat.deepest, flat.depth), (3, 0));
}

#[test]
fn a_namespace_answer_past_seven_is_held_to_seven() {
    let mut flat = Flat::default();
    flat.methods.insert(*b"BIG_", 200);
    let args: Vec<Vec<u8>> = (0..7).map(|_| int(1)).collect();
    let aml = cat(&[&[0x70], &call("BIG", &[&args.concat()]), &[LOCAL0]]);
    let (arena, root) = parse_with(&aml, &mut flat, Default::default()).unwrap();
    let store = arena.kids(root)[0];
    assert_eq!(arena.kids(arena.kids(store)[0]).len(), 7);
}
