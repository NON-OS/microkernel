// NONOS Operating System (AGPL-3.0-or-later)
//! A local or argument that refers to itself, or to another slot of its
//! frame, is freed when the frame ends. The first was found by the `aml`
//! fuzz target (LeakSanitizer: a slot made in Frame::new during load-time
//! code, kept alive by its own reference).

use crate::enc::*;
use crate::exec::{Frame, FaultKind};
use crate::object::{Object, Ref};
use crate::run::{boot, call};
use alloc::rc::Rc;

#[test]
fn a_slot_holding_a_reference_to_itself_is_freed_with_its_frame() {
    let f = Frame::new(0, Default::default());
    let weak = Rc::downgrade(&f.locals[0]);
    *f.locals[0].borrow_mut() = Object::Ref(Ref::Slot(f.locals[0].clone()));
    drop(f);
    assert!(weak.upgrade().is_none(), "the cycle through Local0 is broken");
}

#[test]
fn two_slots_referring_to_each_other_are_freed_with_their_frame() {
    let f = Frame::new(0, Default::default());
    let (a, b) = (Rc::downgrade(&f.args[5]), Rc::downgrade(&f.locals[7]));
    *f.args[5].borrow_mut() = Object::Ref(Ref::Slot(f.locals[7].clone()));
    *f.locals[7].borrow_mut() = Object::Ref(Ref::Slot(f.args[5].clone()));
    drop(f);
    assert!(a.upgrade().is_none() && b.upgrade().is_none());
}

#[test]
fn a_method_storing_a_reference_to_its_own_local_still_runs() {
    // Store (RefOf (Local0), Local0); Return (One)
    let body = cat(&[&store(&e(0x71, &[&local(0)]), &local(0)), &ret(&int(1))]);
    let (mut aml, _) = boot(&method("\\M", 0, &body), 2);
    assert!(matches!(call(&mut aml, "\\M", vec![]), Ok(Object::Integer(1))));
    let again = call(&mut aml, "\\M", vec![]);
    assert!(!matches!(again, Err(ref f) if f.kind == FaultKind::Steps));
}
