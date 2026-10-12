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

//! The polled bulk IN: a qTD left armed on the bulk IN QH, into its root
//! port's own buffer (`lane`, the port less one), so a network adapter's
//! frames are taken whenever they come. The toggle stays the QH's; a
//! reset or a slot's end takes the qTD back with the QH (bulk_run.rs).

use super::bulk::{arm as arm_qtd, ended, BulkEp};
use super::failed::CtrlError;
use crate::env::Mem;
use crate::sched::layout::{poll_buf, poll_qtd, BULK_MAX};
use crate::sched::queue::look as look_qtds;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Polled {
    Pending,
    /// The bytes now in the lane's buffer.
    Done(usize),
    Stalled,
    Error(u32),
}

pub fn arm<M: Mem>(m: &M, lane: usize, ep: &BulkEp, len: usize) -> Result<(), CtrlError> {
    if len == 0 || len > BULK_MAX {
        return Err(CtrlError::TooLong);
    }
    arm_qtd(m, ep, true, (poll_qtd(lane), poll_buf(lane), len));
    Ok(())
}

/// One look at the qTD armed for `len` bytes.
pub fn look<M: Mem>(m: &M, lane: usize, ep: &BulkEp, len: usize) -> Polled {
    let Some(ran) = look_qtds(m, &[poll_qtd(lane)]) else { return Polled::Pending };
    match ended(m, ep, ran, poll_qtd(lane), len) {
        Ok(n) => Polled::Done(n),
        Err(CtrlError::Stalled) => Polled::Stalled,
        Err(CtrlError::Bus(bits)) => Polled::Error(bits),
        Err(_) => Polled::Error(0),
    }
}
