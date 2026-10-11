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

//! CMD1 with the voltage window, polled until the card finishes power up.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::cmds::SEND_OP_COND;
use super::super::ops::say_failed;
use super::super::regs::ocr_ready;

/// The card finishes power up within a second (Linux mmc_send_op_cond: 100
/// tries 10 ms apart).
pub const OP_COND_MS: u64 = 1_000;
pub const OP_COND_POLL_MS: u64 = 10;

/// CMD1 with the voltage window and sector mode until the card reports
/// power up done.
pub(super) fn op_cond<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    arg: u32,
) -> EmmcResult<u32> {
    let d = super::super::super::env::Deadline::after(&h.clock, OP_COND_MS);
    let mut answered = false;
    loop {
        match h.send(&Cmd::new(SEND_OP_COND, arg, Resp::R3)) {
            Ok(r) if ocr_ready(r[0]) => return Ok(r[0]),
            Ok(_) => answered = true,
            Err(EmmcError::CmdTimeout(_)) => {}
            Err(e) => {
                say_failed(h, b"CMD1", e);
                return Err(e);
            }
        }
        if d.passed(&h.clock) {
            let e = if answered { EmmcError::CardBusy } else { EmmcError::NoCard };
            say_failed(h, b"CMD1", e);
            return Err(e);
        }
        h.pause(OP_COND_POLL_MS);
    }
}
