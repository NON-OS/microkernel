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

//! The lowest voltage card and host share, and the power cycle to it.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::host::SETTLE_MS;
use super::super::super::sdhci::power::select;
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::power::power_and_clock;

/// Settle on the lowest voltage the card's OCR and the host share, powering
/// the bus again when it differs from the one it runs at. Returns the OCR
/// voltage window for CMD1.
pub(super) fn settle_voltage<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    host: u32,
    card_ocr: u32,
) -> EmmcResult<u32> {
    let Some((window, vdd)) = select(card_ocr, host) else {
        h.say(
            Line::new()
                .s(b"no shared voltage: card ocr ")
                .hex(card_ocr as u64)
                .s(b" host ocr ")
                .hex(host as u64),
        );
        return Err(EmmcError::NoVoltage);
    };
    h.say(
        Line::new()
            .s(b"card ocr ")
            .hex(card_ocr as u64)
            .s(b" host ocr ")
            .hex(host as u64)
            .s(b": ")
            .dec(vdd.millivolts() as u64)
            .s(b" mV, window ")
            .hex(window as u64),
    );
    if h.vdd != Some(vdd) {
        h.power_off();
        h.pause(SETTLE_MS);
        power_and_clock(h, vdd)?;
    }
    Ok(window)
}
