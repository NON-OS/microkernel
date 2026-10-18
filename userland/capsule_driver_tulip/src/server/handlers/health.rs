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

//! `OP_HEALTHCHECK`: alive, and the part still has both processes running
//! with no fatal bus error. A bus error stops the part by itself; that is
//! reported here rather than discovered as silence on the wire.

use crate::constants::csr::CSR5;
use crate::init::running;
use crate::protocol::{Request, E_IO};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    let status = if running(driver.regs.csr(CSR5)) { 0 } else { E_IO };
    reply_with_status(sender, tx, req, status)
}
