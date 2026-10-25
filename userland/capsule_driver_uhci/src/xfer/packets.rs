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

//! A bulk transfer cut into packets of the endpoint's size, their toggles
//! alternating from the one the endpoint's next packet carries; and the
//! toggle moved on by the packets that ran.

use super::bulk::BulkEp;
use super::chain::Step;
use super::control::CtrlError;
use crate::facts::td::{PID_IN, PID_OUT};
use crate::sched::layout::{BULK_MAX, BULK_TDS};

/// The steps for `len` bytes from `buf`, and how many there are.
pub fn packets(
    ep: &BulkEp,
    input: bool,
    buf: usize,
    len: usize,
) -> Result<([Step; BULK_TDS], usize), CtrlError> {
    // Full speed bulk packets are 8, 16, 32 or 64 bytes (USB 2.0 5.8.3, as
    // recalled, unverified); anything smaller would overrun the chain.
    let mps = ep.mps.clamp(8, 64) as usize;
    let count = len.div_ceil(mps).max(1);
    if len > BULK_MAX || count > BULK_TDS {
        return Err(CtrlError::TooLong);
    }
    let pid = if input { PID_IN } else { PID_OUT };
    let mut steps = [Step { pid, toggle: false, buf, len: 0, spd: input }; BULK_TDS];
    for (i, s) in steps[..count].iter_mut().enumerate() {
        s.toggle = ep.toggle ^ (i % 2 == 1);
        s.buf = buf + i * mps;
        s.len = mps.min(len.saturating_sub(i * mps));
    }
    Ok((steps, count))
}

/// The packets `0..=upto` were acknowledged: each flipped the toggle once.
pub fn advance(ep: &mut BulkEp, upto: usize) {
    ep.toggle ^= (upto + 1) % 2 == 1;
}
