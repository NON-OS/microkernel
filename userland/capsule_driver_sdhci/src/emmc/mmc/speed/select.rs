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

//! The speed choice: High Speed when both sides take it, then the width.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::clock::LEGACY_HZ;
use super::super::super::sdhci::Host;
use super::super::card::Card;
use super::bus_width::bus_width;
use super::high_speed::high_speed;

/// The widths to try, widest first. One line always ends the list: every
/// eMMC device and host drives it.
pub fn widths(host_bus8: bool) -> &'static [u8] {
    if host_bus8 {
        &[8, 4, 1]
    } else {
        &[4, 1]
    }
}

pub fn select<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &mut Card,
    data: DmaBuf,
) -> EmmcResult<()> {
    let Some(ext) = card.ext else {
        // Before version 4: no SWITCH, one line, legacy timing.
        card.hz = h.set_clock(LEGACY_HZ)?;
        return Ok(());
    };
    let hs_hz = ext.hs_hz();
    if hs_hz != 0 && h.can_hs() {
        high_speed(h, card, hs_hz)?;
    }
    if !card.hs {
        card.hz = h.set_timing(false, LEGACY_HZ)?;
    }
    bus_width(h, card, data)
}
