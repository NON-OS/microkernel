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

//! A control transfer: SETUP (DATA0), the data stage as one TD from DATA1
//! (short IN allowed), STATUS the other way (IN with no data), DATA1.

use super::failed::{failed, CtrlError};
use crate::env::{Clock, Mem, Regs};
use crate::facts::bounds::CONTROL_MS;
use crate::facts::regs::CS_CLF;
use crate::facts::td::{DP_IN, DP_OUT, DP_SETUP, TD_LEN};
use crate::sched::ed::{control as ed_word, Target};
use crate::sched::layout::{ctrl_td, CTRL_BUF, CTRL_ED, CTRL_MAX, SETUP_BUF};
use crate::sched::queue::{start, wait, Ran};
use crate::sched::take::clear_halt;
use crate::sched::td::{moved, write, Td};

/// Run `setup`; an IN request's data lands in the control buffer, an OUT
/// request's is taken from it. `at` is the place the queue rests on.
pub fn control<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    at: &mut u8,
    t: Target,
    mps: u16,
    setup: [u8; 8],
) -> Result<usize, CtrlError> {
    let len = u16::from_le_bytes([setup[6], setup[7]]) as usize;
    let input = setup[0] & 0x80 != 0;
    if len > CTRL_MAX {
        return Err(CtrlError::TooLong);
    }
    let (here, tail) = (ctrl_td(*at), ctrl_td(1 - *at));
    m.wr(CTRL_ED, ed_word(t, mps.max(8), false));
    m.write_bytes(SETUP_BUF, &setup);
    let mut tds =
        [Td { dp: DP_SETUP, toggle: Some(false), round: false, buf: SETUP_BUF, len: 8 }; 3];
    let data_dp = if input { DP_IN } else { DP_OUT };
    let mut n = 1;
    if len > 0 {
        tds[1] = Td { dp: data_dp, toggle: Some(true), round: input, buf: CTRL_BUF, len };
        n = 2;
    }
    let status_dp = if input && len > 0 { DP_OUT } else { DP_IN };
    tds[n] = Td { dp: status_dp, toggle: Some(true), round: false, buf: 0, len: 0 };
    n += 1;
    for (i, td) in tds[..n].iter().enumerate() {
        let next = if i + 1 < n { here + (i + 1) * TD_LEN } else { tail };
        write(m, here + i * TD_LEN, *td, next);
    }
    start(m, r, CTRL_ED, tail, CS_CLF);
    let ran = wait(m, r, c, CTRL_ED, tail, CONTROL_MS);
    *at = 1 - *at;
    match ran {
        Ran::Done if len > 0 => Ok(moved(m, here + TD_LEN, CTRL_BUF, len)),
        Ran::Done => Ok(0),
        Ran::Halted => {
            clear_halt(m, CTRL_ED, false);
            Err(failed(m, here, n))
        }
        Ran::TimedOut => Err(CtrlError::TimedOut),
    }
}
