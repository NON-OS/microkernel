// NONOS Operating System (AGPL-3.0-or-later)
//! Every object in the catalogue parses to one node with the right opcode,
//! offset and operand count, and the operands are the kinds the grammar names.

use super::catalog::laid_out;
use crate::enc::*;
use crate::flat::parse;
use crate::op::Op;

#[test]
fn every_catalogued_object_parses_to_its_node() {
    let (aml, cases) = laid_out();
    let (arena, root) = parse(&aml).expect("catalogue parses");
    let top = arena.kids(root);
    assert_eq!(top.len(), cases.len());
    for (node, (at, case)) in top.iter().zip(&cases) {
        let n = arena.node(*node).unwrap();
        assert_eq!(n.op, case.op, "opcode at {at}");
        assert_eq!(n.at as usize, at + 36, "offset of {:?}", case.op);
        assert_eq!(arena.kids(*node).len(), case.kids, "operands of {:?}", case.op);
    }
}

#[test]
fn operands_take_the_kinds_the_grammar_names() {
    // Method: NameString, MethodFlags, then the body kept as bytes.
    let (arena, root) = parse(&method("MTH0", 3, &[0xA3])).unwrap();
    let m = arena.kids(root)[0];
    let ops: Vec<Op> = arena.kids(m).iter().map(|k| arena.node(*k).unwrap().op).collect();
    assert_eq!(ops, [Op::NAME_STRING, Op::IMM, Op::DEFERRED]);
    // Field: NameString, FieldFlags, FieldList.
    let f = pkg(0x5B81, &cat(&[&name("REG0"), &[0x01], &name("FLD0"), &[0x08]]));
    let (arena, root) = parse(&f).unwrap();
    let ops: Vec<Op> =
        arena.kids(arena.kids(root)[0]).iter().map(|k| arena.node(*k).unwrap().op).collect();
    assert_eq!(ops, [Op::NAME_STRING, Op::IMM, Op::FIELD_LIST]);
}

#[test]
fn else_belongs_to_the_if_before_it() {
    let aml = cat(&[&if_(&int(1), &[0xA3]), &else_(&[0xA3]), &[0xA3]]);
    let (arena, root) = parse(&aml).unwrap();
    let top = arena.kids(root);
    assert_eq!(top.len(), 2, "If with its Else, then Noop");
    let if_kids = arena.kids(top[0]);
    assert_eq!(if_kids.len(), 3);
    assert_eq!(arena.node(if_kids[2]).unwrap().op, Op::ELSE);
}

#[test]
fn else_without_an_if_is_refused() {
    let err = parse(&else_(&[0xA3])).unwrap_err();
    assert_eq!(err.kind, crate::error::ErrorKind::Misplaced(0xA1));
}

#[test]
fn constants_carry_their_values() {
    let aml = cat(&[&int(0x12), &int(0x1234), &int(0x1234_5678), &int(0x1_0000_0000)]);
    let (arena, root) = parse(&aml).unwrap();
    let vals: Vec<_> = arena.kids(root).iter().map(|k| arena.node(*k).unwrap().imm).collect();
    use crate::arena::Imm::Int;
    assert_eq!(vals, [Int(0x12), Int(0x1234), Int(0x1234_5678), Int(0x1_0000_0000)]);
}

#[test]
fn a_string_keeps_its_characters_without_the_nul() {
    let (arena, root) = parse(&string("PNP0A08")).unwrap();
    let crate::arena::Imm::Bytes(s) = arena.node(arena.kids(root)[0]).unwrap().imm else {
        panic!("string carries bytes")
    };
    assert_eq!((s.at, s.len), (37, 7));
}
