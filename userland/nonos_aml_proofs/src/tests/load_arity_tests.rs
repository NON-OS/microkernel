// NONOS Operating System (AGPL-3.0-or-later)
//! How the parser counts a call's arguments once a namespace is loaded: from
//! methods in earlier tables, by the search rules for a lone NameSeg, and from
//! the method's own scope when a body is read.

use crate::arena::Imm;
use crate::consts::{at, load};
use crate::enc::*;
use crate::load::ParseScopes;
use crate::op::Op;
use crate::parse::body;

fn call(path: &str, args: &[Vec<u8>]) -> Vec<u8> {
    cat(&[&name(path), &args.concat()])
}

#[test]
fn an_ssdt_calls_a_dsdt_method_found_by_the_search_rules() {
    let dsdt = scope("\\_SB", &cat(&[&method("MTH2", 2, &[0xA3]), &device("PCI0", &[])]));
    let store = cat(&[&[0x70], &call("MTH2", &[int(1), int(2)]), &name("RES0")]);
    let ssdt = scope("\\_SB.PCI0", &cat(&[&name_obj("RES0", &int(0)), &store]));
    let (aml, exec) = load(&dsdt, &[ssdt]);
    assert_eq!(exec.ran, [Op::STORE]);
    assert!(aml.notes.list.is_empty());
}

#[test]
fn a_nearer_name_shadows_a_method_further_up() {
    let dsdt = scope(
        "\\_SB",
        &cat(&[&method("MTH1", 1, &[0xA3]), &device("PCI0", &name_obj("MTH1", &int(3)))]),
    );
    // From inside PCI0, MTH1 is the Name, so `Store(MTH1, Local0)` reads as
    // a Store of a name and a target, not a call that swallows Local0.
    let ssdt = scope("\\_SB.PCI0", &method("USE0", 0, &cat(&[&[0x70], &name("MTH1"), &[LOCAL0]])));
    let (mut aml, _) = load(&dsdt, &[ssdt]);
    let use0 = at(&aml, "\\_SB.PCI0.USE0").unwrap();
    let crate::namespace::Origin::Table { table, node } = aml.ns.get(use0).unwrap().origin else {
        panic!("from a table")
    };
    let t = &aml.tables[table as usize];
    let deferred = *t.arena.node(t.arena.kids(node)[2]).unwrap();
    let Imm::Bytes(span) = deferred.imm else { panic!("deferred body") };
    let mut scopes = ParseScopes::new(&aml.ns, use0);
    let t = &mut aml.tables[table as usize];
    let block = crate::block::Block { bytes: &t.bytes };
    let terms = body(&block, &mut t.arena, &mut scopes, span).expect("body parses");
    let store = t.arena.kids(terms)[0];
    let source = t.arena.kids(store)[0];
    assert_eq!(t.arena.kids(source).len(), 0, "MTH1 here is the Name");
}
