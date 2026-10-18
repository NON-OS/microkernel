// NONOS Operating System (AGPL-3.0-or-later)
//! Method calls, what a method leaves behind, and names in packages.

use crate::consts::at;
use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::run::{boot, call, eval, num};

fn add(a: &[u8], b: &[u8], t: &[u8]) -> Vec<u8> {
    e(0x72, &[a, b, t])
}

#[test]
fn methods_call_methods_and_return_values() {
    let sq = method("SQ", 1, &ret(&e(0x77, &[&arg(0), &arg(0), &[0]])));
    let m = method("\\M", 1, &ret(&add(&cat(&[&name("SQ"), &arg(0)]), &int(1), &[0])));
    let (mut aml, _) = boot(&cat(&[&sq, &m]), 2);
    assert_eq!(num(&call(&mut aml, "\\M", vec![Object::Integer(7)]).unwrap()), 50);
}

#[test]
fn what_a_method_defines_is_gone_when_it_returns() {
    let m = method("\\M", 0, &cat(&[&name_obj("TMP", &int(5)), &ret(&name("TMP"))]));
    let (mut aml, _) = boot(&m, 2);
    for _ in 0..3 {
        assert_eq!(
            num(&call(&mut aml, "\\M", vec![]).unwrap()),
            5,
            "no redefinition on a second run"
        );
    }
    assert!(at(&aml, "\\M.TMP_").is_none());
}

#[test]
fn a_serialized_method_may_call_itself() {
    let dec = e(0x74, &[&arg(0), &int(1), &[0]]);
    let rec = pkg(
        0x14,
        &cat(&[
            &name("\\R"),
            &[0x09],
            &if_(&arg(0), &ret(&add(&cat(&[&name("R"), &dec]), &int(1), &[0]))),
            &ret(&int(0)),
        ]),
    );
    let (mut aml, _) = boot(&rec, 2);
    assert_eq!(num(&call(&mut aml, "\\R", vec![Object::Integer(5)]).unwrap()), 5);
}

#[test]
fn break_outside_a_loop_and_no_return() {
    assert_eq!(eval(&[0xA5], vec![]).unwrap_err().kind, FaultKind::Control);
    assert!(matches!(eval(&[0xA3], vec![]).unwrap(), Object::Uninit));
}

#[test]
fn package_names_are_references_or_their_text() {
    let defs = name_obj("PKG0", &package(2, &[name("\\_SB"), name("NONE")]));
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &ret(&name("PKG0")))]), 2);
    let Object::Package(p) = call(&mut aml, "\\M", vec![]).unwrap() else { panic!("package") };
    assert!(matches!(p.borrow()[0], Object::Ref(_)));
    assert_eq!(crate::run::raw(&p.borrow()[1]), b"NONE");
}
