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

use nonos_libc::mk_debug;
use smoltcp::iface::{Interface, SocketSet};

use crate::state::{self, DnsSockets};

pub fn handle_deconfigured(
    iface: &mut Interface,
    sockets: &mut SocketSet<'static>,
    dns_slot: &mut DnsSockets,
) {
    iface.update_ip_addrs(|addrs| addrs.clear());
    let _ = iface.routes_mut().remove_default_ipv4_route();
    for old in dns_slot.iter_mut().filter_map(Option::take) {
        sockets.remove(old);
    }
    state::set_lease(None);
    // smoltcp drops a lease that ran out unrenewed, a NAK from the server, or
    // one reset when the link came back; each was silent, and the address
    // just vanished from Settings.
    let line = b"[NET-CORE] lease dropped; DHCP asks for a new one\n";
    mk_debug(line.as_ptr(), line.len());
}
