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

//! The bring-up sequence: each step in order, then the card as found.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::card::Card;
use super::super::cmds::{RCA, SET_BLOCKLEN};
use super::super::regs::ocr_sector_mode;
use super::capacity::capacity;
use super::check_ready::check_ready;
use super::power_on::power_on;
use super::read_cid::read_cid;
use super::read_ext::read_ext;
use super::select_card::select_card;
use super::step::{r1_ok, step};
use super::user_area::user_area;

/// Bring the card up. Fails, having logged why, when any step does; the
/// caller then quiesces the host and may try again from the start.
pub fn bring_up<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    data: DmaBuf,
) -> EmmcResult<Card> {
    let ocr = power_on(h)?;
    let sector_mode = ocr_sector_mode(ocr);

    let cid = read_cid(h, sector_mode)?;
    let csd = select_card(h)?;

    let ext = read_ext(h, &csd, data)?;
    let sectors = capacity(sector_mode, &csd, ext.as_ref());
    if sectors == 0 {
        h.say(Line::new().s(b"card reports no capacity"));
        return Err(EmmcError::NoCapacity);
    }
    if !sector_mode {
        let r = step(h, &Cmd::new(SET_BLOCKLEN, 512, Resp::R1), b"CMD16 SET_BLOCKLEN")?;
        r1_ok(h, SET_BLOCKLEN, r[0])?;
    }
    let mut card = Card {
        rca: RCA,
        ocr,
        sector_mode,
        cid,
        ext,
        sectors,
        cmd23: csd.spec_vers >= 3,
        width: 1,
        hs: false,
        hz: h.sd_hz,
    };
    user_area(h, &mut card)?;
    super::super::speed::select(h, &mut card, data)?;
    check_ready(h, &card)?;
    Ok(card)
}
