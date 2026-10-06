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

//! Says on the log which DNS servers the lease named, one line each, so a
//! photo shows what net.dns asks.

use nonos_libc::mk_debug;

use crate::iface::dhcp::write_octet_quad::write_octet_quad;
use crate::state::DNS_SERVERS;

pub fn emit_dns_marker(servers: &[[u8; 4]; DNS_SERVERS]) {
    let mut named = 0;
    for &server in servers.iter().filter(|s| **s != [0; 4]) {
        say(b"[NET-CORE] lease dns ", Some(server));
        named += 1;
    }
    if named == 0 {
        say(b"[NET-CORE] lease dns none; names will not resolve", None);
    }
}

fn say(tag: &[u8], server: Option<[u8; 4]>) {
    let mut buf = [0u8; 64];
    buf[..tag.len()].copy_from_slice(tag);
    let mut pos = tag.len();
    if let Some(quad) = server {
        pos = write_octet_quad(&mut buf, pos, quad);
    }
    buf[pos] = b'\n';
    mk_debug(buf.as_ptr(), pos + 1);
}
