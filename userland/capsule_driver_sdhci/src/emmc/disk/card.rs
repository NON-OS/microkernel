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

//! Which card protocol a slot is brought up with.

use super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::error::EmmcResult;
use super::super::mmc::{self, Card};
use super::super::sd;
use super::super::sdhci::regs::{CAPABILITIES, SLOT_TYPE_REMOVABLE};
use super::super::sdhci::{Caps, Host};

/// A removable slot holds whatever card a person put in it: an SD card is
/// tried first, then MMC, which answers none of SD's identify steps. An
/// embedded or shared slot is wired to the board's eMMC, and so is a known
/// Intel eMMC host whatever its capabilities say: those are brought up as
/// MMC alone.
pub(super) fn card<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    data: DmaBuf,
) -> EmmcResult<Card> {
    let caps = Caps { caps: h.io.r32(CAPABILITIES), caps1: 0, version: 0 };
    if caps.slot_type() == SLOT_TYPE_REMOVABLE && !h.intel_emmc {
        if let Some(card) = sd::bring_up(h)? {
            return Ok(card);
        }
        // No SD card answered: MMC starts from a host reset again, power
        // and clock off, as the SD attempt did.
        h.init()?;
    }
    mmc::bring_up(h, data)
}
