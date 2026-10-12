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

//! The polled bulk IN: a chain left armed on its own queue, which the
//! controller retries every frame while the device NAKs, so a network
//! adapter's frames are taken whenever they come. A look is one read of
//! the chain; once it ended the queue is emptied and the toggle moved on.

use super::bulk::BulkEp;
use super::chain::{moved, write};
use super::control::CtrlError;
use super::packets::{advance, packets};
use super::run::{look as look_chain, Ran};
use crate::env::Mem;
use crate::sched::layout::{POLL_BUF, POLL_QH, POLL_TD};
use crate::sched::qh::{start, stop};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Polled {
    Pending,
    /// The bytes now at POLL_BUF.
    Done(usize),
    Stalled,
    Error(u32),
}

/// Arm `len` bytes; returns how many TDs the chain has.
pub fn arm<M: Mem>(m: &M, ep: &BulkEp, len: usize) -> Result<usize, CtrlError> {
    let (steps, count) = packets(ep, true, POLL_BUF, len)?;
    let n = write(m, POLL_TD, ep.t, &steps[..count]);
    start(m, POLL_QH, POLL_TD);
    Ok(n)
}

/// One look at an armed chain of `n` TDs.
pub fn look<M: Mem>(m: &M, ep: &mut BulkEp, n: usize) -> Polled {
    let Some(ran) = look_chain(m, POLL_TD, n) else { return Polled::Pending };
    stop(m, POLL_QH);
    match ran {
        Ran::Ended { upto, .. } => {
            advance(ep, upto);
            Polled::Done(moved(m, POLL_TD, 0, upto))
        }
        Ran::Stalled => Polled::Stalled,
        Ran::Error(bits) => Polled::Error(bits),
        // A single look has no deadline; look_chain never says this.
        Ran::TimedOut => Polled::Error(0),
    }
}

/// Take the chain back unrun, as an endpoint reset or a slot's end does.
pub fn disarm<M: Mem>(m: &M) {
    stop(m, POLL_QH);
}
