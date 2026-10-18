// NONOS Operating System (AGPL-3.0-or-later)
//! W4: a device whose bus master never reports stopped is said so, so a
//! restart never turns Bus Master on for it, and it is still reset.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::gen3::regs::{CSR_RESET, RESET_STOP_MASTER, RESET_SW_RESET};
use crate::gen3::start::stop_device;
use crate::gen3_model::Polls;
use crate::regs::Mmio;

/// A device whose bus master never reports stopped.
struct Stuck(RefCell<HashMap<usize, u32>>);

impl Mmio for Stuck {
    fn read32(&self, off: usize) -> u32 {
        self.0.borrow().get(&off).copied().unwrap_or(0)
    }
    fn write32(&self, off: usize, val: u32) {
        self.0.borrow_mut().insert(off, val);
    }
}

#[test]
fn a_bus_master_that_never_stops_is_reported_and_still_reset() {
    let m = Stuck(RefCell::new(HashMap::new()));
    assert!(!stop_device(&m, &mut Polls { per_wait: 64, delays_us: 0 }), "no Bus Master for it");
    assert_eq!(
        m.read32(CSR_RESET) & (RESET_STOP_MASTER | RESET_SW_RESET),
        RESET_STOP_MASTER | RESET_SW_RESET
    );
}
