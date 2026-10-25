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

//! Power at 3.3 V, the identification clock, CMD0, and CMD8: whether the
//! card is version 2.00 or later and takes the host's supply.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::mmc::cmds::GO_IDLE_STATE;
use super::super::super::mmc::init::step::step;
use super::super::super::sdhci::clock::IDENT_HZ;
use super::super::super::sdhci::host::SETTLE_MS;
use super::super::super::sdhci::power::Vdd;
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::cmds::{if_cond_echoed, IF_COND_ARG, SEND_IF_COND};

/// On a host `EmmcDisk::reset` readied: Some(true) for a card that echoed
/// CMD8 (version 2.00 or later), Some(false) for one that did not answer it
/// (version 1.x, or no SD card at all), None when the host has no 3.3 V to
/// power an SD card with. A card that answers with another range or pattern
/// cannot run on this supply.
pub(super) fn start<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>) -> EmmcResult<Option<bool>> {
    if !h.caps.v330() {
        h.say(Line::new().s(b"slot has no 3.3 V supply: not trying an SD card"));
        return Ok(None);
    }
    h.power_up(Vdd::V33)?;
    let hz = h.set_clock(IDENT_HZ)?;
    h.say(Line::new().s(b"identification clock ").dec(hz as u64).s(b" Hz"));
    // At least 74 clocks before the first command, with margin.
    h.pause(SETTLE_MS);
    step(h, &Cmd::new(GO_IDLE_STATE, 0, Resp::None), b"CMD0 GO_IDLE_STATE")?;
    h.pause(1);
    // R7 has R1's format on the bus: index and CRC checked.
    match h.send(&Cmd::new(SEND_IF_COND, IF_COND_ARG, Resp::R1)) {
        Ok(r) if if_cond_echoed(r[0]) => Ok(Some(true)),
        Ok(r) => {
            h.say(Line::new().s(b"CMD8 echoed ").hex(r[0] as u64).s(b", not 1aa"));
            Err(EmmcError::NoVoltage)
        }
        Err(EmmcError::CmdTimeout(_)) => Ok(Some(false)),
        Err(e) => Err(e),
    }
}
