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

//! CMD1 with no voltage window: the card's OCR, or no card.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::power::EMMC_DUAL_OCR;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::SEND_OP_COND;
use super::super::ops::say_failed;
use super::super::regs::OCR_VOLTAGE_MASK;
use super::op_cond::OP_COND_POLL_MS;

/// Tries at the OCR query before the slot counts as holding no card.
pub const QUERY_TRIES: u32 = 3;

/// CMD1 with no voltage window: the card answers with its OCR and powers
/// up nothing. Silence on every try means no card.
pub(super) fn query_ocr<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>) -> EmmcResult<u32> {
    for _ in 0..QUERY_TRIES {
        match h.send(&Cmd::new(SEND_OP_COND, 0, Resp::R3)) {
            Ok(r) if r[0] & OCR_VOLTAGE_MASK != 0 => return Ok(r[0]),
            Ok(r) => {
                h.say(
                    Line::new()
                        .s(b"card ocr ")
                        .hex(r[0] as u64)
                        .s(b" names no voltage, assuming dual"),
                );
                return Ok(EMMC_DUAL_OCR | (r[0] & !OCR_VOLTAGE_MASK));
            }
            Err(EmmcError::CmdTimeout(_)) => {}
            Err(e) => {
                say_failed(h, b"CMD1 query", e);
                return Err(e);
            }
        }
        h.pause(OP_COND_POLL_MS);
    }
    say_failed(h, b"CMD1 query", EmmcError::NoCard);
    Err(EmmcError::NoCard)
}
