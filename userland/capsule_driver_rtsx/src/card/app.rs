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

//! An application command: CMD55 with the card's address, which the card
//! must acknowledge with APP_CMD in its status, then the command itself.
//! Only APP_CMD is checked, as mmc_app_cmd does: a version 1.x card reports
//! the CMD8 it did not know as ILLEGAL_COMMAND in exactly this status.

use super::command::send_command;
use crate::error::{Result, RtsxError};
use crate::sd::{r1_app_cmd, Command};
use crate::setup::Driver;
use crate::wire::Response;

pub fn app_command(drv: &Driver, rca: u16, cmd: Command) -> Result<Response> {
    if !r1_app_cmd(send_command(drv, Command::app_cmd(rca))?.words[0]) {
        return Err(RtsxError::CardStatus);
    }
    send_command(drv, cmd)
}
