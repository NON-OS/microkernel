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

//! Power on: host init, first voltage, OCR query, shared voltage, CMD1.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::power::{first_vdd, host_ocr};
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::regs::OCR_SECTOR_MODE;
use super::op_cond::op_cond;
use super::power::{go_idle, power_and_clock};
use super::query_ocr::query_ocr;
use super::voltage::settle_voltage;

/// From a reset host to a card that has finished power up at the lowest
/// voltage both sides share. Returns the card's OCR.
pub(super) fn power_on<M: Mmio, C: Clock, L: Log>(h: &mut Host<M, C, L>) -> EmmcResult<u32> {
    h.init()?;
    let host = host_ocr(&h.caps);
    let Some(first) = first_vdd(host) else {
        h.say(Line::new().s(b"host offers no bus voltage"));
        return Err(EmmcError::NoVoltage);
    };
    power_and_clock(h, first)?;
    go_idle(h)?;
    let card_ocr = query_ocr(h)?;
    let window = settle_voltage(h, host, card_ocr)?;
    go_idle(h)?;
    let ocr = op_cond(h, window | OCR_SECTOR_MODE)?;
    Ok(ocr)
}
