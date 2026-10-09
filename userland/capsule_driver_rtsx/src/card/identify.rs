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

//! Identification up to a powered card (SD Physical Layer 4.2.3, as
//! mmc_attach_sd runs it): CMD0, CMD8 to learn whether the card is version
//! 2.00 or later and may be high capacity, then ACMD41 every 10 ms until
//! the card reports power-up done, for at most one second.

use super::app::app_command;
use super::command::send_command;
use crate::clock::{now, sleep_ms, Budget};
use crate::error::{Result, RtsxError};
use crate::sd::{if_cond_echoed, ocr_ready, Command};
use crate::setup::Driver;

const POWER_UP_MS: u64 = 1000;

/// The OCR of a card that finished powering up.
pub fn identify(drv: &Driver) -> Result<u32> {
    send_command(drv, Command::GO_IDLE)?;
    sleep_ms(1);
    // A version 1.x card does not answer CMD8; that is no failure.
    let v2 =
        matches!(send_command(drv, Command::SEND_IF_COND), Ok(r) if if_cond_echoed(r.words[0]));
    let budget = Budget::begin(now(), POWER_UP_MS);
    loop {
        let ocr = match app_command(drv, 0, Command::sd_send_op_cond(v2)) {
            Ok(r) => r.words[0],
            Err(RtsxError::Gone) => return Err(RtsxError::Gone),
            Err(_) => return Err(RtsxError::NotSd),
        };
        if ocr_ready(ocr) {
            return Ok(ocr);
        }
        if budget.spent(now()) {
            return Err(RtsxError::PowerUpTimeout);
        }
        sleep_ms(10);
    }
}
