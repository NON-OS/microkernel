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

//! `OP_HEALTHCHECK`: alive, and neither queue stopped by the device. A
//! queue the device stops on a bad descriptor stays stopped until reset;
//! that is reported here rather than discovered as silence on the wire.

use crate::constants::cmd::GET_QUEUE_STATUS;
use crate::protocol::{Request, E_IO};
use crate::server::error::reply_with_status;
use crate::setup::Driver;
use crate::shared::queues_failed;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    driver.regs.command(GET_QUEUE_STATUS);
    let status = if queues_failed(driver.control_va) { E_IO } else { 0 };
    reply_with_status(sender, tx, req, status)
}
