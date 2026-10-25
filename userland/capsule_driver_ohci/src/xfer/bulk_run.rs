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
//! ED reset to DATA0 or retired.

use super::bulk::{arm, ended, BulkEp};
use super::failed::CtrlError;
use crate::env::{Clock, Mem, Regs};
use crate::facts::bounds::BULK_MS;
use crate::sched::layout::{bulk_ed, bulk_td, BULK_BUF, BULK_MAX};
use crate::sched::queue::wait;
use crate::sched::take::{park, settle};

/// Move `len` bytes through the bulk buffer; returns the bytes moved.
pub fn bulk<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    ep: &mut BulkEp,
    input: bool,
    len: usize,
) -> Result<usize, CtrlError> {
    if len > BULK_MAX {
        return Err(CtrlError::TooLong);
    }
    arm(m, r, ep, input, BULK_BUF, len);
    let ran = wait(m, r, c, bulk_ed(ep.index), bulk_td(ep.index, 1 - ep.at), BULK_MS);
    ended(m, ep, ran, BULK_BUF, len)
}

/// DATA0 next, as the device is after CLEAR_FEATURE(ENDPOINT_HALT).
pub fn reset<M: Mem, R: Regs, C: Clock>(m: &M, r: &mut R, c: &C, ep: &BulkEp) {
    settle(m, r, c, bulk_ed(ep.index), false);
}

/// The ED retired, skipped and empty on its first place.
pub fn retire<M: Mem, R: Regs, C: Clock>(m: &M, r: &mut R, c: &C, index: usize) {
    park(m, r, c, bulk_ed(index), bulk_td(index, 0));
}
