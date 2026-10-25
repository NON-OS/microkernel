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

//! The SD bring-up, in the SD Physical Layer's initialisation order (as
//! recalled, unverified): power, CMD0, CMD8, ACMD41 until ready, CID,
//! address, CSD, select; then block length, bus width and clock.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::mmc::Card;
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::regs::ccs;
use super::finish::finish;
use super::identify::identify;
use super::op_cond::op_cond;
use super::start::start;

/// Bring an SD memory card up. None when the slot holds no card that
/// answers as one, so the caller may try it as MMC; an error when an SD
/// card answered and a later step failed.
pub fn bring_up<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>) -> EmmcResult<Option<Card>> {
    let Some(v2) = start(h)? else {
        return Ok(None);
    };
    let Some(ocr) = op_cond(h, v2)? else {
        h.say(Line::new().s(b"no SD card answered ACMD41"));
        return Ok(None);
    };
    // CCS means something only to a host that set HCS.
    let sector_mode = v2 && ccs(ocr);
    let (cid, rca, sectors) = identify(h, sector_mode)?;
    let mut card = Card {
        rca,
        ocr,
        sector_mode,
        cid,
        ext: None,
        sectors,
        // SET_BLOCK_COUNT is optional for SD (SCR CMD_SUPPORT): a
        // multi-block transfer ends with the host's own CMD12.
        cmd23: false,
        width: 1,
        hs: false,
        hz: h.sd_hz,
        sd: true,
    };
    finish(h, &mut card)?;
    Ok(Some(card))
}
