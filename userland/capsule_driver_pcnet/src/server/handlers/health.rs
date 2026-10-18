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

//! `OP_HEALTHCHECK`: alive, and the part still has both engines on. A
//! memory error or a babble stops the part by itself; that is reported here
//! rather than discovered as silence on the wire.

use crate::constants::csr::{CSR0, CSR0_RXON, CSR0_TXON};
use crate::protocol::{Request, E_IO};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) -> bool {
    let on = CSR0_RXON | CSR0_TXON;
    let status = if driver.regs.csr(CSR0) & on == on { 0 } else { E_IO };
    reply_with_status(sender, tx, req, status)
}
