// NONOS Operating System (AGPL-3.0-or-later)
//! The model port this thread's register accesses go to, if any.

use std::cell::RefCell;
use std::rc::Rc;

use super::hba::Hba;

thread_local! {
    static PORT: RefCell<Option<Rc<RefCell<Hba>>>> = const { RefCell::new(None) };
}

/// Send this thread's register accesses to `hba`, on model time.
pub(super) fn plug(hba: Rc<RefCell<Hba>>) {
    PORT.with(|p| *p.borrow_mut() = Some(hba));
    crate::clock::run_on_model_time(true);
}

/// Back to host memory and the host clock.
pub(super) fn unplug() {
    PORT.with(|p| *p.borrow_mut() = None);
    crate::clock::run_on_model_time(false);
}

/// A register read, answered by the plugged-in port.
pub fn read(off: u32) -> Option<u32> {
    PORT.with(|p| p.borrow().as_ref().map(|h| h.borrow_mut().read(off)))
}

/// A register write, taken by the plugged-in port. False with none.
pub fn write(off: u32, value: u32) -> bool {
    PORT.with(|p| p.borrow().as_ref().map(|h| h.borrow_mut().write(off, value)).is_some())
}
