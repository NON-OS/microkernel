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

//! A listener takes connections from outside any mixnet, so net.sockets
//! opens one only while the chosen network is Direct: under any other
//! route, or a policy store that does not answer, OP_LISTEN is refused as
//! having no transport and net.tcp is never asked.

use nonos_policy_proto::route;

use crate::protocol::E_NO_TRANSPORT;
use crate::server::handlers::host_target::is_direct;
use crate::server::handlers::listen_route::listen_route;

const LISTEN: &str =
    include_str!("../../../../../capsule_net_sockets/src/server/handlers/listen.rs");

#[test]
fn a_listener_opens_only_on_the_direct_route() {
    assert_eq!(listen_route(|| is_direct(Some(route::DIRECT))), Ok(()));
    assert_eq!(
        listen_route(|| is_direct(None)),
        Err(E_NO_TRANSPORT),
        "a store that does not answer"
    );
    for v in (0..=u8::MAX).filter(|v| *v != route::DIRECT) {
        assert_eq!(listen_route(|| is_direct(Some(v))), Err(E_NO_TRANSPORT), "route {v}");
    }
}

#[test]
fn the_route_is_asked_before_net_tcp() {
    let gate = LISTEN.find("listen_route(direct_chosen)").expect("listen asks the route");
    let tcp = LISTEN.find("tcp::listen(").expect("listen asks net.tcp");
    assert!(gate < tcp, "net.tcp is asked before the route is");
    assert!(LISTEN.contains("if let Err(errno) = listen_route(direct_chosen) {\n        return status(pid, req, errno, tx);"));
}
