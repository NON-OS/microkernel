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

//! A guest's Nym stream goes by name through net.socks5, on a stream of its
//! own, in route link's frames; never through net.sockets' mixnet socket.

use crate::frame::numbered;
use crate::frame_on::{on, NUMBERED_ON};
use crate::linux::net::guest_route::Path;
use crate::socks::connect_request;

const NET: &str = include_str!("../../../../../capsule_linux/src/linux/net/mod.rs");
const OUT: &str = include_str!("../../../../../capsule_linux/src/linux/net/connect/out.rs");
const NYM: &str = include_str!("../../../../../capsule_linux/src/linux/net/routed/nym.rs");
const ROUTE: &str = include_str!("../../../../../capsule_linux/src/linux/net/routed/route.rs");

#[test]
fn the_connect_names_the_host_unresolved_on_the_guest_stream() {
    for (host, port) in [("node.example.org", 18089u16), ("198.51.100.7", 18081)] {
        let ask = connect_request(host, port).expect("a host SOCKS carries");
        let frame = on(3, &numbered(9, &ask).expect("one frame")).expect("named");
        assert_eq!(frame[..9], [NUMBERED_ON, 3, 0, 0, 0, 9, 0, 0, 0], "stream 3, number 9");
        let socks = &frame[9..];
        /* SOCKS5, CONNECT, ATYP 3: a DOMAINNAME, never an address looked up here. */
        assert_eq!(socks[..5], [5, 1, 0, 3, host.len() as u8]);
        assert_eq!(&socks[5..5 + host.len()], host.as_bytes());
        assert_eq!(socks[5 + host.len()..], port.to_be_bytes());
    }
}

#[test]
fn the_nym_path_opens_through_route_link_at_net_socks5() {
    assert!(NYM.contains("RouteOpening::begin_on(Route::Nym(port), host, dest.port"));
    assert!(ROUTE.contains("port_of(b\"net.socks5\")"));
    assert!(ROUTE.contains("Path::Nym(port) =>"));
    assert_eq!(service(Path::Nym(1)), "net.socks5");
}

/// Every path there is, by its service: net.sockets is not one of them.
fn service(p: Path) -> &'static str {
    match p {
        Path::Nym(_) => "net.socks5",
        Path::Anyone(_) => "net.anon",
        Path::Wait(..) => "none yet",
    }
}

#[test]
fn no_guest_stream_reaches_net_sockets() {
    for (name, text) in
        [("connect/out.rs", OUT), ("routed/nym.rs", NYM), ("routed/route.rs", ROUTE)]
    {
        for word in ["KIND_MIXNET", "OP_CONNECT_HOST", "OP_SOCKET", "sockets::", "host_body("] {
            assert!(!text.contains(word), "{name} names {word}");
        }
    }
    /* net.sockets is compiled only into a development build's local mirror. */
    assert!(NET.contains("#[cfg(feature = \"dev-mirror\")]\nmod sockets;"));
    for gone in ["mod call;", "mod ops;", "mod host_body;", "mod stream;"] {
        assert!(!NET.contains(&format!("\n{gone}")), "{gone} is compiled into every build");
    }
}
