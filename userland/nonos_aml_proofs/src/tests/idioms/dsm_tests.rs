// NONOS Operating System (AGPL-3.0-or-later)
//! A _DSM as the specification's example writes it (ACPI 6.5, 9.1.1, and
//! Switch in 19.6.133; recalled, unverified): compare Arg0 with the UUID,
//! check the revision, then Switch on ToInteger(Arg2). Function 0 answers
//! a buffer of the functions supported; an unknown UUID, revision or
//! function falls through to Buffer() {0}. ASL compiles Switch to a While
//! whose body stores the value in a temporary, tests each Case with an
//! If/Else chain and ends with Break, which is what is encoded here.

use super::more::*;
use crate::enc::*;
use crate::object::Object;
use crate::rig::Rig;

pub fn uuid() -> Vec<u8> {
    (0..16).map(|i| 0x10 + i as u8).collect()
}

fn rig() -> Rig {
    let t0 = || name("_T_0");
    let case = |v: u64, body: &[u8], rest: &[u8]| {
        cat(&[&if_(&bin(LEQUAL, &t0(), &int(v)), body), &else_(rest)])
    };
    let echo = if_(&bin(LEQUAL, &t0(), &int(2)), &ret(&arg(3)));
    let chain = case(0, &ret(&buffer(1, &[0x07])), &case(1, &ret(&int(0x11)), &echo));
    let switch =
        while_(&int(1), &cat(&[&store(&e(0x99, &[&arg(2), &[0]]), &t0()), &chain, &[0xA5]]));
    let rev_ok = e(0x92, &[&e(0x95, &[&arg(1), &int(1)])]);
    let body = cat(&[
        &name_obj("_T_0", &int(0)),
        &if_(&bin(LEQUAL, &arg(0), &buffer(16, &uuid())), &if_(&rev_ok, &switch)),
        &ret(&buffer(1, &[0])),
    ]);
    Rig::new(&method("\\_DSM", 4, &body)).connected()
}

pub fn buf(v: &[u8]) -> Object {
    Object::Buffer(core::cell::RefCell::new(v.to_vec()).into())
}

fn dsm(r: &mut Rig, id: &[u8], rev: u64, f: Object) -> Object {
    let args = vec![buf(id), Object::Integer(rev), f, Object::Integer(0x77)];
    r.call("\\_DSM", args).expect("_DSM runs")
}

#[test]
fn each_function_and_every_way_to_miss() {
    let mut r = rig();
    let id = uuid();
    let is = |o: Object, want: &[u8]| matches!(o, Object::Buffer(b) if *b.borrow() == want);
    assert!(is(dsm(&mut r, &id, 1, Object::Integer(0)), &[0x07]), "functions 0 to 2");
    assert_eq!(crate::run::num(&dsm(&mut r, &id, 1, Object::Integer(1))), 0x11);
    assert_eq!(crate::run::num(&dsm(&mut r, &id, 2, Object::Integer(2))), 0x77);
    assert!(is(dsm(&mut r, &id, 1, Object::Integer(3)), &[0]), "past the mask: Break, then {{0}}");
    assert!(is(dsm(&mut r, &id, 0, Object::Integer(0)), &[0]), "revision 0");
    let mut other = id.clone();
    other[15] ^= 1;
    assert!(is(dsm(&mut r, &other, 1, Object::Integer(0)), &[0]), "one bit off the UUID");
    assert!(is(dsm(&mut r, &id[..15], 1, Object::Integer(0)), &[0]), "a short UUID");
    let f = Object::String(core::cell::RefCell::new(b"0x1".to_vec()).into());
    assert_eq!(crate::run::num(&dsm(&mut r, &id, 1, f)), 0x11, "ToInteger reads a hex string");
}
