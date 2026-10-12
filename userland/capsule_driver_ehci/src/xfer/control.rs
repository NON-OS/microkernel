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

//! A control transfer on a control QH (the toggle from each qTD): SETUP
//! (DATA0), the data stage as one qTD from DATA1, a short IN packet taking
//! its alternate link straight to STATUS, then STATUS the other way (IN
//! with no data), DATA1.

use super::failed::{failed, CtrlError};
use crate::env::{Clock, Mem, Regs};
use crate::facts::bounds::CONTROL_MS;
use crate::facts::qtd::{PID_IN, PID_OUT, PID_SETUP, QTD_LEN};
use crate::sched::layout::{qh, CTRL_BUF, CTRL_MAX, CTRL_QTD, SETUP_BUF};
use crate::sched::qtd::{moved, write, Qtd};
use crate::sched::queue::{start, wait, Ran};
use crate::sched::take::clear_halt;

/// Run `setup` on ring QH `ring`; an IN request's data lands in the
/// control buffer, an OUT request's is taken from it.
pub fn control<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    op: u32,
    ring: usize,
    setup: [u8; 8],
) -> Result<usize, CtrlError> {
    let len = u16::from_le_bytes([setup[6], setup[7]]) as usize;
    let input = setup[0] & 0x80 != 0;
    if len > CTRL_MAX {
        return Err(CtrlError::TooLong);
    }
    m.write_bytes(SETUP_BUF, &setup);
    let (q0, q1, q2) = (CTRL_QTD, CTRL_QTD + QTD_LEN, CTRL_QTD + 2 * QTD_LEN);
    let setup_td = Qtd { pid: PID_SETUP, toggle: false, buf: SETUP_BUF, len: 8 };
    let status_pid = if input && len > 0 { PID_OUT } else { PID_IN };
    let status = Qtd { pid: status_pid, toggle: true, buf: 0, len: 0 };
    let qtds: &[usize] = if len > 0 {
        let data_pid = if input { PID_IN } else { PID_OUT };
        let data = Qtd { pid: data_pid, toggle: true, buf: CTRL_BUF, len };
        write(m, q2, status, None, None);
        write(m, q1, data, Some(q2), input.then_some(q2));
        &[q0, q1, q2]
    } else {
        write(m, q1, status, None, None);
        &[q0, q1]
    };
    write(m, q0, setup_td, Some(q1), None);
    start(m, qh(ring), q0);
    match wait((m, r, c), op, ring, qtds, CONTROL_MS) {
        Ran::Done if len > 0 => Ok(moved(m, q1, len)),
        Ran::Done => Ok(0),
        Ran::Halted(status) => {
            clear_halt(m, qh(ring), false);
            Err(failed(status))
        }
        Ran::TimedOut => Err(CtrlError::TimedOut),
    }
}
