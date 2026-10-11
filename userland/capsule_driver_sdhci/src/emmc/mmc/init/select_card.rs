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

//! CMD3 sets the address, CMD9 reads the CSD, CMD7 selects the card.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::{rca_arg, RCA, SELECT_CARD, SEND_CSD, SET_RELATIVE_ADDR};
use super::super::r1::CARD_IS_LOCKED;
use super::super::regs::{r2, Csd};
use super::step::{r1_ok, step};

/// Give the card its address, read its CSD and select it. A password
/// locked card fails here.
pub(super) fn select_card<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>) -> EmmcResult<Csd> {
    let r =
        step(h, &Cmd::new(SET_RELATIVE_ADDR, rca_arg(RCA), Resp::R1), b"CMD3 SET_RELATIVE_ADDR")?;
    r1_ok(h, SET_RELATIVE_ADDR, r[0])?;
    let csd =
        Csd::parse(r2(step(h, &Cmd::new(SEND_CSD, rca_arg(RCA), Resp::R2), b"CMD9 SEND_CSD")?));
    h.say(
        Line::new()
            .s(b"csd structure ")
            .dec(csd.structure as u64)
            .s(b" spec ")
            .dec(csd.spec_vers as u64)
            .s(b" c_size ")
            .hex(csd.c_size as u64),
    );
    let r = step(h, &Cmd::new(SELECT_CARD, rca_arg(RCA), Resp::R1), b"CMD7 SELECT_CARD")?;
    if r[0] & CARD_IS_LOCKED != 0 {
        h.say(Line::new().s(b"card is password locked"));
        return Err(EmmcError::Locked);
    }
    r1_ok(h, SELECT_CARD, r[0])?;
    Ok(csd)
}
