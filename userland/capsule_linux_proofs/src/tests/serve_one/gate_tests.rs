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

//! P.4 and P.5: a box with SERVE_ONE listens at one address only, a stream
//! on exactly 127.0.0.1 at its field's port, and holds no NET_LISTEN_LO by
//! it, so every other bind stays EACCES as it was.

use crate::app::rules::inet_allowed;
use crate::app::serve_rule::{may_bind, serve_at, SERVE_IP};
use nonos_linux_policy::mask::{NET_ANON, NET_LISTEN_LO, SERVE_ONE};
use nonos_linux_policy::ServeOne;

const PORT: u16 = 5225;

fn field() -> Option<ServeOne> {
    ServeOne::new(PORT, b"app.simplex")
}

#[test]
fn only_a_stream_on_exactly_127_0_0_1_at_the_fields_port_is_served() {
    assert!(serve_at(SERVE_ONE, field(), true, SERVE_IP, PORT));
    for ip in [[127, 0, 0, 2], [127, 1, 0, 1], [127, 255, 255, 254], [0, 0, 0, 0], [10, 0, 0, 1]] {
        assert!(!serve_at(SERVE_ONE, field(), true, ip, PORT), "{ip:?}");
    }
    assert!(!serve_at(SERVE_ONE, field(), true, SERVE_IP, PORT + 1), "another port");
    assert!(!serve_at(SERVE_ONE, field(), true, SERVE_IP, 0), "port 0");
    assert!(!serve_at(SERVE_ONE, field(), false, SERVE_IP, PORT), "a datagram");
    assert!(!serve_at(SERVE_ONE, None, true, SERVE_IP, PORT), "no field");
    assert!(!serve_at(NET_LISTEN_LO | NET_ANON, field(), true, SERVE_IP, PORT), "no bit");
}

#[test]
fn serve_one_alone_admits_an_internet_socket_and_no_other_bind() {
    assert!(inet_allowed(SERVE_ONE));
    assert!(may_bind(SERVE_ONE, field(), true, SERVE_IP, PORT));
    let others = [
        (true, [127, 0, 0, 2], PORT),
        (true, SERVE_IP, 8080),
        (true, [0; 4], PORT),
        (false, SERVE_IP, PORT),
    ];
    for (stream, ip, port) in others {
        for mask in [SERVE_ONE, SERVE_ONE | NET_ANON] {
            assert!(!may_bind(mask, field(), stream, ip, port), "{mask:#x} {stream} {ip:?}:{port}");
        }
        // NET_LISTEN_LO is what takes them (net/bind.rs then holds it to 127.0.0.0/8).
        assert!(may_bind(SERVE_ONE | NET_LISTEN_LO, field(), stream, ip, port));
    }
    assert!(!may_bind(NET_ANON, field(), true, SERVE_IP, PORT), "the field without the bit");
}
