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

//! The trace shapes that carry an address or bytes from a peer.

use nonos_libc::mk_debug;

use super::super::write::Line;

/// Report an IPv4 address and port.
///
pub fn say_addr(stage: &[u8], address: [u8; 4], port: u16) {
    let mut line = Line::new(stage);
    line.text(b" ");
    for (index, octet) in address.iter().enumerate() {
        if index > 0 {
            line.text(b".");
        }
        line.num(*octet as u64);
    }
    line.text(b":");
    line.num(port as u64);
    let (bytes, len) = line.finish();
    mk_debug(bytes.as_ptr(), len);
}
