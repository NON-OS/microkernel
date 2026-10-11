// NONOS Operating System (AGPL-3.0-or-later)
//! If, Else, While, Break, Continue and Return; arguments and locals.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::run::{eval, num};

fn add(a: &[u8], b: &[u8], t: &[u8]) -> Vec<u8> {
    e(0x72, &[a, b, t])
}

#[test]
fn if_and_else_choose() {
    let body = cat(&[&if_(&arg(0), &ret(&int(1))), &else_(&ret(&int(2)))]);
    assert_eq!(num(&eval(&body, vec![Object::Integer(5)]).unwrap()), 1);
    assert_eq!(num(&eval(&body, vec![Object::Integer(0)]).unwrap()), 2);
}

#[test]
fn while_with_break_and_continue() {
    // Sum 0..10, skipping 5, stopping at 8: 0+1+2+3+4+6+7 = 23.
    let lt = e(0x95, &[&local(0), &int(10)]);
    let body = cat(&[
        &store(&int(0), &local(0)),
        &store(&int(0), &local(1)),
        &pkg(
            0xA2,
            &cat(&[
                &lt,
                &if_(&e(0x93, &[&local(0), &int(8)]), &[0xA5]),
                &if_(&e(0x93, &[&local(0), &int(5)]), &cat(&[&e(0x75, &[&local(0)]), &[0x9F]])),
                &add(&local(1), &local(0), &local(1)),
                &e(0x75, &[&local(0)]),
            ]),
        ),
        &ret(&local(1)),
    ]);
    assert_eq!(num(&eval(&body, vec![]).unwrap()), 23);
}

#[test]
fn all_seven_arguments_and_eight_locals() {
    let mut body = Vec::new();
    for i in 0..8 {
        body.extend(store(&int(i as u64 * 10), &local(i)));
    }
    body.extend(ret(&add(&arg(6), &local(7), &[0])));
    let args = (0..7).map(Object::Integer).collect();
    assert_eq!(num(&eval(&body, args).unwrap()), 6 + 70);
}

#[test]
fn reading_a_local_never_stored_is_a_fault() {
    assert_eq!(eval(&ret(&local(3)), vec![]).unwrap_err().kind, FaultKind::Uninit);
    assert_eq!(eval(&ret(&arg(0)), vec![]).unwrap_err().kind, FaultKind::Uninit);
}
