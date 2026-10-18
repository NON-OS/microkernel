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

//! The TDs of a control transfer: SETUP (toggle 0, the eight request
//! bytes), the data stage in packets of the endpoint's size with toggles
//! from 1, then STATUS in the opposite direction (IN when there is no data)
//! with toggle 1 (USB 2.0 control transfers, as recalled). Pure.

use super::chain::Step;
use crate::facts::td::{PID_IN, PID_OUT, PID_SETUP};
use crate::sched::layout::{CTRL_BUF, CTRL_MAX, CTRL_TDS, SETUP_BUF};

/// The steps, how many, and how many carry data; None when the request is
/// longer than the buffer or needs more TDs than the chain holds.
pub fn control_plan(setup: [u8; 8], mps: u16) -> Option<([Step; CTRL_TDS], usize, usize)> {
    let len = u16::from_le_bytes([setup[6], setup[7]]) as usize;
    let input = setup[0] & 0x80 != 0;
    let mps = mps.max(8) as usize;
    let packets = len.div_ceil(mps);
    if len > CTRL_MAX || packets + 2 > CTRL_TDS {
        return None;
    }
    let first = Step { pid: PID_SETUP, toggle: false, buf: SETUP_BUF, len: 8, spd: false };
    let mut steps = [first; CTRL_TDS];
    let data_pid = if input { PID_IN } else { PID_OUT };
    for (i, s) in steps[1..=packets].iter_mut().enumerate() {
        let len = mps.min(len - i * mps);
        let buf = CTRL_BUF + i * mps;
        *s = Step { pid: data_pid, toggle: i % 2 == 0, buf, len, spd: input };
    }
    let status_pid = if input && len > 0 { PID_OUT } else { PID_IN };
    steps[packets + 1] = Step { pid: status_pid, toggle: true, buf: 0, len: 0, spd: false };
    Some((steps, packets + 2, packets))
}
