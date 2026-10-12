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

//! CMD2 reads the CID, CMD3 asks the card its address, CMD9 reads the
//! CSD, CMD7 selects the card.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::mmc::cmds::{rca_arg, ALL_SEND_CID, SELECT_CARD, SEND_CSD};
use super::super::super::mmc::init::step::{r1_ok, step};
use super::super::super::mmc::r1::CARD_IS_LOCKED;
use super::super::super::mmc::regs::{bits, r2};
use super::super::super::mmc::Cid;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::SEND_RELATIVE_ADDR;
use super::super::regs::{sd_cid, sd_sectors};

/// R6 bits 15:13: COM_CRC_ERROR, ILLEGAL_COMMAND and ERROR (4.9.5, as recalled, unverified).
const R6_ERRORS: u32 = 0xe000;

/// The card's CID, the address it published and its size in sectors. A
/// password locked card fails here.
pub(super) fn identify<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
) -> EmmcResult<(Cid, u16, u64)> {
    let cid = sd_cid(r2(step(h, &Cmd::new(ALL_SEND_CID, 0, Resp::R2), b"CMD2 ALL_SEND_CID")?));
    // R6 has R1's format on the bus.
    let r = step(h, &Cmd::new(SEND_RELATIVE_ADDR, 0, Resp::R1), b"CMD3 SEND_RELATIVE_ADDR")?;
    if r[0] & R6_ERRORS != 0 {
        h.say(Line::new().s(b"CMD3 status error ").hex(r[0] as u64));
        return Err(EmmcError::Status { cmd: SEND_RELATIVE_ADDR, status: r[0] });
    }
    let rca = (r[0] >> 16) as u16;
    let csd = r2(step(h, &Cmd::new(SEND_CSD, rca_arg(rca), Resp::R2), b"CMD9 SEND_CSD")?);
    let Some(sectors) = sd_sectors(csd) else {
        h.say(
            Line::new().s(b"SD csd structure ").dec(bits(csd, 127, 126) as u64).s(b" not served"),
        );
        return Err(EmmcError::NoCapacity);
    };
    let r = step(h, &Cmd::new(SELECT_CARD, rca_arg(rca), Resp::R1), b"CMD7 SELECT_CARD")?;
    if r[0] & CARD_IS_LOCKED != 0 {
        h.say(Line::new().s(b"card is password locked"));
        return Err(EmmcError::Locked);
    }
    r1_ok(h, SELECT_CARD, r[0])?;
    h.say(
        Line::new()
            .s(b"SD card ")
            .ascii(&cid.pnm[..5])
            .s(b" mid ")
            .hex(cid.mid as u64)
            .s(b" psn ")
            .hex(cid.psn as u64)
            .s(b" rca ")
            .hex(rca as u64),
    );
    Ok((cid, rca, sectors))
}
