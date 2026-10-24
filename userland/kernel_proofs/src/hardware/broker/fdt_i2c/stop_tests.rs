// NONOS Operating System (AGPL-3.0-or-later)
//! The stop routine against a scripted controller: IC_ENABLE cleared first,
//! then IC_ENABLE_STATUS read until the controller is off, within a bound.

use std::cell::RefCell;

use super::stop::{stop, DwRegs, IC_ENABLE, IC_ENABLE_STATUS};

/// A controller that reports itself on for `busy` reads after it is told
/// to stop, and every access it saw.
struct Dw {
    busy: RefCell<u32>,
    log: RefCell<Vec<(char, usize, u32)>>,
}

impl DwRegs for Dw {
    fn read32(&self, offset: usize) -> u32 {
        let mut b = self.busy.borrow_mut();
        let on = u32::from(*b > 0);
        *b = b.saturating_sub(1);
        self.log.borrow_mut().push(('r', offset, on));
        on
    }
    fn write32(&self, offset: usize, value: u32) {
        self.log.borrow_mut().push(('w', offset, value));
    }
    fn pause_us(&self, _us: u32) {}
}

fn dw(busy: u32) -> Dw {
    Dw { busy: RefCell::new(busy), log: RefCell::new(vec![]) }
}

#[test]
fn the_enable_is_cleared_then_watched_until_off() {
    let d = dw(3);
    assert!(stop(&d));
    let log = d.log.borrow();
    assert_eq!(log[0], ('w', IC_ENABLE, 0), "cleared first");
    assert!(log[1..].iter().all(|&(k, off, _)| k == 'r' && off == IC_ENABLE_STATUS));
    assert_eq!(log.len(), 1 + 4, "three looks on, the fourth off");
}

#[test]
fn a_controller_that_stays_on_is_reported_after_the_bound() {
    let d = dw(u32::MAX);
    assert!(!stop(&d));
    assert_eq!(d.log.borrow().len(), 1 + 100);
}
