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
use crate::driver::Driver;
use crate::protocol::{Request, HDR_LEN, HEALTH_LOST, HEALTH_RESP_LEN, STATUS_LEN};
use crate::server::respond;
/// The compositor's poll: its heartbeat, and LOST once after a blank.
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let flags = if driver.owner.take_lost(sender_pid) { HEALTH_LOST } else { 0 };
    let off = HDR_LEN + STATUS_LEN;
    tx[off..off + 4].copy_from_slice(&flags.to_le_bytes());
    respond::payload(sender_pid, req, HEALTH_RESP_LEN, tx);
}
