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
//! A bulk transfer through the bulk buffer, waited for; and an endpoint's
//! QH reset to DATA0.

use super::bulk::{arm, ended, BulkEp};
use super::failed::CtrlError;
use crate::env::{Clock, Mem, Regs};
use crate::facts::bounds::BULK_MS;
use crate::sched::layout::{BULK_BUF, BULK_MAX, BULK_QTD};
use crate::sched::queue::wait;
use crate::sched::take::take_back;

/// Move `len` bytes through the bulk buffer; returns the bytes moved.
pub fn bulk<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    op: u32,
    ep: &BulkEp,
    input: bool,
    len: usize,
) -> Result<usize, CtrlError> {
    if len > BULK_MAX {
        return Err(CtrlError::TooLong);
    }
    arm(m, ep, input, (BULK_QTD, BULK_BUF, len));
    let ran = wait((m, r, c), op, ep.ring, &[BULK_QTD], BULK_MS);
    ended(m, ep, ran, BULK_QTD, len)
}

/// DATA0 next, as the device is after CLEAR_FEATURE(ENDPOINT_HALT); also
/// what a slot's end does to its QHs.
pub fn reset<M: Mem, R: Regs, C: Clock>(mrc: (&M, &mut R, &C), op: u32, ep: &BulkEp) {
    take_back(mrc, op, ep.ring, false, None);
}
