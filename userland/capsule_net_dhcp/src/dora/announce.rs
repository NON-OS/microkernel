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

//! The bound lease said once on the serial console, whole, through MkDebug.
//! Debug is optional (Capsule.mk): without it the call is refused and the
//! lease stands all the same.

use crate::dhcp::Message;

use super::lease_line::lease_line;

pub fn lease(ack: &Message, prefix: u8) {
    let (line, len) = lease_line(&ack.yiaddr, prefix, &ack.router, &ack.server_id);
    let _ = nonos_libc::mk_debug(line.as_ptr(), len);
}
