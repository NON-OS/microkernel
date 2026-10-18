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

//! Waiting for the card to finish what it was sent (the MMC core's
//! card_busy_detect, with CMD13): asked every millisecond until it is ready
//! for data and back in the transfer state, for at most PROGRAM_MS. An SD
//! card reports a write's errors in the status that follows it, so an
//! error bit here fails the write. A sync is this wait on its own.

use super::command::send_command;
use super::info::Card;
use super::limits::PROGRAM_MS;
use crate::clock::{now, sleep_ms, Budget};
use crate::error::{Result, RtsxError};
use crate::sd::{r1_error, r1_ready, Command};
use crate::setup::Driver;

pub fn wait_ready(drv: &Driver, card: &Card) -> Result<()> {
    let budget = Budget::begin(now(), PROGRAM_MS);
    loop {
        let status = send_command(drv, Command::send_status(card.rca))?.words[0];
        if r1_error(status) {
            return Err(RtsxError::CardError);
        }
        if r1_ready(status) {
            return Ok(());
        }
        if budget.spent(now()) {
            return Err(RtsxError::CardBusy);
        }
        sleep_ms(1);
    }
}
