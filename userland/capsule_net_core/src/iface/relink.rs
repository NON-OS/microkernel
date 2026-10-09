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

//! Starts DHCP over when the bound link comes back.
//!
//! RFC 2131 3.7: a client that may have moved to another network asks for
//! its address again rather than keep using it. smoltcp's reset drops the
//! lease and runs a fresh exchange; a router that knows the MAC hands back
//! the same address, so connections that outlived the drop carry on.

use nonos_libc::{mk_debug, mk_uptime_ms};
use smoltcp::socket::dhcpv4;
use spin::Mutex;

use crate::iface::link_watch::{Change, LinkWatch};
use crate::{device, setup, state};

static WATCH: Mutex<LinkWatch> = Mutex::new(LinkWatch::new());

pub fn check() {
    let port = setup::bound_port();
    let now = mk_uptime_ms();
    let mut watch = WATCH.lock();
    if port == 0 || !watch.due(now) {
        return;
    }
    match watch.observe(now, port, device::link_up(port)) {
        Some(Change::Dropped) => say(b"[NET-CORE] link down on the bound interface\n"),
        Some(Change::Returned) => {
            say(b"[NET-CORE] link back; DHCP asks for the lease again\n");
            state::with_dhcp_and_dns_slot(|_, sockets, dhcp, _| {
                sockets.get_mut::<dhcpv4::Socket>(dhcp).reset();
            });
        }
        None => {}
    }
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
