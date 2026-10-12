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

//! A bulk transfer on the bulk queue: packets of the endpoint's size from
//! the bulk buffer, toggles continuing from where the endpoint's last
//! transfer left them, and a short IN packet ending the transfer (SPD).

use super::chain::{moved, write};
use super::control::CtrlError;
use super::packets::{advance, packets};
use super::run::{run, Ran};
use crate::env::{Clock, Mem};
use crate::facts::bounds::BULK_MS;
use crate::sched::layout::{BULK_BUF, BULK_QH, BULK_TD};
use crate::sched::td::Target;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BulkEp {
    pub t: Target,
    pub mps: u16,
    /// The toggle the next packet carries.
    pub toggle: bool,
}

/// Move `len` bytes through the bulk buffer; returns the bytes moved.
pub fn bulk<M: Mem, C: Clock>(
    m: &M,
    c: &C,
    ep: &mut BulkEp,
    input: bool,
    len: usize,
) -> Result<usize, CtrlError> {
    let (steps, count) = packets(ep, input, BULK_BUF, len)?;
    let n = write(m, BULK_TD, ep.t, &steps[..count]);
    match run(m, c, BULK_QH, BULK_TD, n, BULK_MS) {
        Ran::Ended { upto, .. } => {
            advance(ep, upto);
            Ok(moved(m, BULK_TD, 0, upto))
        }
        Ran::Stalled => Err(CtrlError::Stalled),
        Ran::Error(bits) => Err(CtrlError::Bus(bits)),
        Ran::TimedOut => Err(CtrlError::TimedOut),
    }
}
