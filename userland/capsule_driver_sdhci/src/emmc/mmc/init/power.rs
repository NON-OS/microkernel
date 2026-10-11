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

//! Bus power with the identification clock, and CMD0 to idle.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::clock::IDENT_HZ;
use super::super::super::sdhci::host::SETTLE_MS;
use super::super::super::sdhci::power::Vdd;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::GO_IDLE_STATE;
use super::step::step;

pub(super) fn power_and_clock<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    vdd: Vdd,
) -> EmmcResult<()> {
    h.power_up(vdd)?;
    let hz = h.set_clock(IDENT_HZ)?;
    h.say(Line::new().s(b"identification clock ").dec(hz as u64).s(b" Hz"));
    // At least 74 clocks before the first command, with margin.
    h.pause(SETTLE_MS);
    Ok(())
}

pub(super) fn go_idle<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>) -> EmmcResult<()> {
    step(h, &Cmd::new(GO_IDLE_STATE, 0, Resp::None), b"CMD0 GO_IDLE_STATE")?;
    h.pause(1);
    Ok(())
}
