// NONOS Operating System (AGPL-3.0-or-later)
//! A TD retired: its condition code written, onto the done queue, and its
//! ED's HeadP moved on, halted on an error, with the toggle carried when
//! the TD took it from the ED.

use super::td::Step;
use super::world::World;
use crate::facts::ed::{HEAD_C, HEAD_H, PTR_MASK};
use crate::facts::regs::INT_WDH;

/// The TD retired with `cc` onto the done head; HeadP moves on, halted on
/// an error, with `carry` as the ED's toggle carry when one is given.
pub fn retire(w: &mut World, ed: u32, td: u32, cc: u32, carry: Option<bool>) -> Step {
    w.wr32(td, w.rd32(td) & 0x0FFF_FFFF | cc << 28);
    let (head, next) = (w.rd32(ed + 8), w.rd32(td + 8) & PTR_MASK);
    let c = carry.map_or(head & HEAD_C, |c| if c { HEAD_C } else { 0 });
    let h = if cc == 0 { 0 } else { HEAD_H };
    w.wr32(ed + 8, next | c | h);
    w.done(td);
    if cc == 0 {
        Step::Retired
    } else {
        Step::Halted
    }
}

impl World {
    /// A retired TD onto the done queue; the HCCA's done head is written
    /// once the driver has cleared the last writeback (WDH).
    pub fn done(&mut self, td: u32) {
        if self.int_status & INT_WDH == 0 && self.hcca != 0 {
            self.wr32(self.hcca + 0x84, td);
            self.int_status |= INT_WDH;
        }
    }
}
