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

//! CMD2 ALL_SEND_CID: the card's identity, logged.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::ALL_SEND_CID;
use super::super::regs::{r2, Cid};
use super::step::step;

/// Read the CID and log the part, its ids and the access mode.
pub(super) fn read_cid<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    sector_mode: bool,
) -> EmmcResult<Cid> {
    let cid = Cid::parse(r2(step(h, &Cmd::new(ALL_SEND_CID, 0, Resp::R2), b"CMD2 ALL_SEND_CID")?));
    h.say(
        Line::new()
            .s(b"card ")
            .ascii(&cid.pnm)
            .s(b" mid ")
            .hex(cid.mid as u64)
            .s(b" oid ")
            .hex(cid.oid as u64)
            .s(b" prv ")
            .hex(cid.prv as u64)
            .s(b" psn ")
            .hex(cid.psn as u64)
            .s(if sector_mode { b" sector mode" as &[u8] } else { b" byte mode" }),
    );
    Ok(cid)
}
