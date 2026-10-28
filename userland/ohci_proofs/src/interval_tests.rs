// NONOS Operating System (AGPL-3.0-or-later)
//! The frame interval read from the controller before its reset: kept
//! when it is a full-speed frame, else the reset value, and the largest
//! packet worked out from it without wrapping.

use std::cell::RefCell;
use std::rc::Rc;

use crate::facts::regs::{FI_DEFAULT, FI_MASK, FSMPS_SHIFT};
use crate::hc::interval::{largest_packet, FI_SLACK};
use crate::hc::start::start;
use crate::model::{ModelClock, ModelMem, ModelRegs, World};
use crate::server::context::Ctx;

#[test]
fn a_frame_interval_no_full_speed_frame_has_is_the_reset_value_and_never_underflows() {
    for (read, kept) in
        [(100, FI_DEFAULT), (209, FI_DEFAULT), (0x3FFF, FI_DEFAULT), (0, FI_DEFAULT)]
            .into_iter()
            .chain([
                (FI_DEFAULT + FI_SLACK, FI_DEFAULT + FI_SLACK),
                (FI_DEFAULT - 2, FI_DEFAULT - 2),
            ])
    {
        let w = Rc::new(RefCell::new(World { fm_interval: read, ..World::new() }));
        let mut cx = Ctx::new(ModelRegs(w.clone()), ModelClock(w.clone()), ModelMem(w.clone()));
        assert_eq!(start(&mut cx.io, &cx.clock, &cx.mem, 0x1000), Ok(()), "FI {read}");
        let fm = w.borrow().fm_interval;
        assert_eq!((fm & FI_MASK, fm >> FSMPS_SHIFT & 0x7FFF), (kept, largest_packet(kept)));
        assert_eq!(w.borrow().periodic_start, kept * 9 / 10);
    }
    assert_eq!(largest_packet(100), 0, "no packet, no wrap");
}
