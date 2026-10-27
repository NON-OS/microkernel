// NONOS Operating System (AGPL-3.0-or-later)
//! A DAC code that does not read back is reported even when the calibration
//! itself finished: step 1's done flags set, step 3's codes not there.

use crate::model::Chip;
use crate::phy::dack::run;
use crate::phy::rf::set_paths;

/// Each path's step-1 done flags (read base + 0x08 and + 0x34, bits 22:7).
const DONE: [usize; 4] = [0x2808, 0x2834, 0x4508, 0x4534];

#[test]
fn codes_that_do_not_read_back_are_reported() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    *c.hw.sample.borrow_mut() = Some(Box::new(|| 0x10 << 12 | 0x10));
    for r in DONE {
        c.set32(r, 0xFFFF << 7);
    }
    assert!(!run(&c), "step 1 done, step 3's codes absent");
}
