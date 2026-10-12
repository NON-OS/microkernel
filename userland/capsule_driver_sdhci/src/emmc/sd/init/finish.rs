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

//! A selected card made ready for data: the block length of a byte-mode
//! card, four data lines, the Default Speed clock, and transfer state.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::mmc::cmds::SET_BLOCKLEN;
use super::super::super::mmc::init::check_ready::check_ready;
use super::super::super::mmc::init::step::{r1_ok, step};
use super::super::super::mmc::ops::say_failed;
use super::super::super::mmc::Card;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::{BUS_WIDTH_4, DEFAULT_SPEED_HZ, SET_BUS_WIDTH};
use super::app::app;

pub(super) fn finish<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &mut Card,
) -> EmmcResult<()> {
    if !card.sector_mode {
        let r = step(h, &Cmd::new(SET_BLOCKLEN, 512, Resp::R1), b"CMD16 SET_BLOCKLEN")?;
        r1_ok(h, SET_BLOCKLEN, r[0])?;
    }
    // Every SD memory card drives one and four data lines (SCR
    // SD_BUS_WIDTHS, 5.6 as recalled), and every SDHCI host four.
    let set = Cmd::new(SET_BUS_WIDTH, BUS_WIDTH_4, Resp::R1);
    let r = app(h, card.rca, &set).inspect_err(|&e| say_failed(h, b"ACMD6 SET_BUS_WIDTH", e))?;
    r1_ok(h, SET_BUS_WIDTH, r[0])?;
    h.set_width(4);
    card.width = 4;
    card.hz = h.set_timing(false, DEFAULT_SPEED_HZ)?;
    h.say(Line::new().s(b"bus width 4 bits"));
    check_ready(h, card)
}
