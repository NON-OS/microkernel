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

//! ACMD41: the card's OCR, then the power up it finishes within a second.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::mmc::ops::say_failed;
use super::super::super::mmc::regs::ocr_ready;
use super::super::super::sdhci::power::OCR_32_34;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::{HCS, OP_COND_MS, OP_COND_POLL_MS, SD_SEND_OP_COND};
use super::app::app;

/// The card's OCR once power up is done, or None when nothing answered as
/// an SD card: an MMC card ignores CMD55. `v2` asks for high capacity
/// (HCS), which a version 1.x card must not be offered.
pub(super) fn op_cond<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    v2: bool,
) -> EmmcResult<Option<u32>> {
    let card = match app(h, 0, &Cmd::new(SD_SEND_OP_COND, 0, Resp::R3)) {
        Ok(r) => r[0],
        Err(EmmcError::CmdTimeout(_) | EmmcError::Status { .. }) => return Ok(None),
        Err(e) => {
            say_failed(h, b"ACMD41 query", e);
            return Err(e);
        }
    };
    // The bus runs at 3.3 V: the window is the card's 3.2-3.4 V bits.
    let window = card & OCR_32_34;
    if window == 0 {
        h.say(Line::new().s(b"SD card ocr ").hex(card as u64).s(b" takes no 3.3 V"));
        return Err(EmmcError::NoVoltage);
    }
    let arg = window | if v2 { HCS } else { 0 };
    let d = Deadline::after(&h.clock, OP_COND_MS);
    loop {
        match app(h, 0, &Cmd::new(SD_SEND_OP_COND, arg, Resp::R3)) {
            Ok(r) if ocr_ready(r[0]) => return Ok(Some(r[0])),
            Ok(_) => {}
            Err(e) => {
                say_failed(h, b"ACMD41", e);
                return Err(e);
            }
        }
        if d.passed(&h.clock) {
            say_failed(h, b"ACMD41", EmmcError::CardBusy);
            return Err(EmmcError::CardBusy);
        }
        h.pause(OP_COND_POLL_MS);
    }
}
