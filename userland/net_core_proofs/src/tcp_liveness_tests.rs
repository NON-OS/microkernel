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

//! A connection is kept while its peer answers, rides out a router reboot,
//! and is given up when nothing answers for TIMEOUT.

use smoltcp::socket::tcp::{Socket, State};
use smoltcp::time::Duration;

use crate::tcp_liveness::{KEEP_ALIVE, TIMEOUT};
use crate::tcp_pair::Pair;

// A probe must land inside every TIMEOUT window, twice over.
const _: () = assert!(KEEP_ALIVE.total_millis() * 2 < TIMEOUT.total_millis());

#[test]
fn an_idle_connection_is_kept_by_its_probes() {
    let mut p = Pair::established();
    p.run(TIMEOUT * 6);
    assert_eq!(p.state(p.client), State::Established);
    assert_eq!(p.state(p.server), State::Established);
}

#[test]
fn a_silent_peer_is_given_up_at_the_timeout() {
    let mut p = Pair::established();
    p.link.cut = true;
    p.run(TIMEOUT - Duration::from_secs(2));
    assert_eq!(p.state(p.client), State::Established);
    p.run(Duration::from_secs(4));
    assert_eq!(p.state(p.client), State::Closed);
}

// Two minutes with every frame lost, data waiting to go, then the link is
// back: the data arrives on the same connection.
#[test]
fn data_sent_during_a_router_reboot_arrives_after_it() {
    let mut p = Pair::established();
    p.link.cut = true;
    let sent = p.sockets.get_mut::<Socket>(p.client).send_slice(b"GET / HTTP/1.1\r\n\r\n");
    assert_eq!(sent, Ok(18));
    p.run(Duration::from_secs(120));
    p.link.cut = false;
    p.run(Duration::from_secs(15));
    assert_eq!(p.state(p.client), State::Established);
    assert_eq!(p.sockets.get::<Socket>(p.server).recv_queue(), 18);
}
