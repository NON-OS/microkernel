// NONOS Operating System (AGPL-3.0-or-later)
//! The scripted bus's interface holds and pauses: the host's claim table
//! as a list of interfaces another driver holds, and every pause counted,
//! never slept, so a wait's whole length is known at once.

use std::cell::{Cell, RefCell};

use nonos_usbnet::mock::MockBus;

use crate::rtl::{Claim, Pause};

thread_local! {
    pub static HELD: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    pub static ASKED: RefCell<Vec<(u8, u8)>> = const { RefCell::new(Vec::new()) };
    pub static PAUSED_MS: Cell<u64> = const { Cell::new(0) };
}

impl Claim for MockBus {
    fn claim(&mut self, port: u8, interface: u8) -> Result<(), i32> {
        ASKED.with(|a| a.borrow_mut().push((port, interface)));
        if HELD.with(|h| h.borrow().contains(&interface)) {
            Err(-16)
        } else {
            Ok(())
        }
    }
}

impl Pause for MockBus {
    fn pause_ms(&mut self, ms: u16) {
        PAUSED_MS.with(|p| p.set(p.get() + ms as u64));
    }
}

/// Milliseconds paused on this thread since the last call.
pub fn paused() -> u64 {
    PAUSED_MS.with(|p| p.replace(0))
}
