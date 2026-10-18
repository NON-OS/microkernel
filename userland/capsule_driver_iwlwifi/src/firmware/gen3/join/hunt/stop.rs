// NONOS Operating System (AGPL-3.0-or-later)

//! Stopping a hunt's sweep in flight.

use super::super::super::dev::Dev;
use super::super::super::region::{Clock, Region};
use super::super::super::rx_frame::RxFrame;
use super::super::super::sweep::{Sweep, Tick};
use super::kinds::{HuntEnd, ABORT_PUMPS, PUMP_US};
use crate::regs::Mmio;

// Stop a sweep in flight and wait, bounded, for its completion.
pub(super) fn stop<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    sweep: &mut Sweep,
    on_frame: &mut dyn FnMut(RxFrame),
) -> Result<(), HuntEnd> {
    if !sweep.running() {
        return Ok(());
    }
    sweep.abort(dev, c, on_frame).map_err(|_| HuntEnd::Radio)?;
    for _ in 0..ABORT_PUMPS {
        if !sweep.running() {
            return Ok(());
        }
        c.delay_us(PUMP_US);
        // The budget is not the bound here: the completion either comes or
        // the radio is not the join's to use.
        if sweep.pump(dev, 0, on_frame) == Tick::Failed {
            return Err(HuntEnd::Radio);
        }
    }
    if sweep.running() {
        Err(HuntEnd::Radio)
    } else {
        Ok(())
    }
}
