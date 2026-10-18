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

//! The end of a transfer: CMD12 after a multi-block one, as sd_request
//! sends mrq->stop, and after a write the card asked until it has
//! programmed what it took (ready.rs). That wait is kept when the data
//! phase failed, one block or many, as the MMC core waits for busy after
//! every write: the card may still be programming what it took, and the
//! next request power-cycles it. The wait is bounded by PROGRAM_MS, and
//! the transfer's own error is the one answered.

use super::command::send_command;
use super::info::Card;
use super::ready::wait_ready;
use crate::error::{Result, RtsxError};
use crate::sd::{r1_error, Command};
use crate::setup::Driver;

/// CMD12, whether or not the data moved, and an error bit in its status
/// is the transfer's. Not to a card that left, or a reader gone, where it
/// could only wait out its time.
pub fn stop_transmission(drv: &Driver, done: Result<()>) -> Result<()> {
    if matches!(done, Err(RtsxError::Removed | RtsxError::Gone)) {
        return done;
    }
    let stop =
        send_command(drv, Command::STOP_TRANSMISSION).and_then(|r| match r1_error(r.words[0]) {
            true => Err(RtsxError::CardError),
            false => Ok(()),
        });
    done.and(stop)
}

/// A write's data phase ended `done`: CMD12 after a multi-block one, then
/// the programming wait, failed or not, unless the card or the reader is
/// gone.
pub fn finish_write(drv: &Driver, card: &Card, done: Result<()>, multi: bool) -> Result<()> {
    let ended = if multi { stop_transmission(drv, done) } else { done };
    if matches!(ended, Err(RtsxError::Removed | RtsxError::Gone)) {
        return ended;
    }
    let ready = wait_ready(drv, card);
    ended.and(ready)
}
