// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The polled bulk IN: a TD left armed on the bulk IN endpoint's ED, which
//! the controller tries every frame while the device NAKs, so a network
//! adapter's frames are taken whenever they come. A look is one read of
//! the ED; the toggle stays the ED's carry, shared with any bulk IN.

use super::bulk::{arm as arm_td, ended, BulkEp};
use super::failed::CtrlError;
use crate::env::{Clock, Mem, Regs};
use crate::sched::layout::{bulk_ed, bulk_td, poll_buf, BULK_MAX};
use crate::sched::queue::look as look_ed;
use crate::sched::take::{retired, settle};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Polled {
    Pending,
    /// The bytes now in the lane's buffer.
    Done(usize),
    Stalled,
    Error(u32),
}

/// Arm `len` bytes into root port `lane + 1`'s buffer.
pub fn arm<M: Mem, R: Regs>(
    m: &M,
    r: &mut R,
    lane: usize,
    ep: &BulkEp,
    len: usize,
) -> Result<(), CtrlError> {
    if len == 0 || len > BULK_MAX {
        return Err(CtrlError::TooLong);
    }
    arm_td(m, r, ep, true, poll_buf(lane), len);
    Ok(())
}

/// One look at the TD armed for `len` bytes.
pub fn look<M: Mem, R: Regs>(m: &M, r: &mut R, lane: usize, ep: &mut BulkEp, len: usize) -> Polled {
    let Some(ran) = look_ed(m, bulk_ed(ep.index), bulk_td(ep.index, 1 - ep.at)) else {
        return Polled::Pending;
    };
    retired(r);
    match ended(m, ep, ran, poll_buf(lane), len) {
        Ok(n) => Polled::Done(n),
        Err(CtrlError::Stalled) => Polled::Stalled,
        Err(CtrlError::Bus(cc)) => Polled::Error(cc),
        Err(_) => Polled::Error(0),
    }
}

/// Take the armed TD back unrun, as an endpoint reset or a slot's end does.
pub fn disarm<M: Mem, R: Regs, C: Clock>(m: &M, r: &mut R, c: &C, ep: &mut BulkEp) {
    settle(m, r, c, bulk_ed(ep.index), true);
    ep.at = 1 - ep.at;
}
