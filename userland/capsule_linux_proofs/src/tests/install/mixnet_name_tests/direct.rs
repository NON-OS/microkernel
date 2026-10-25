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

//! A name on any socket is refused while the chosen network is not Direct,
//! and what counts as Direct: only the Direct route, never a store that does
//! not answer.

use nonos_policy_proto::route;

use super::helpers::connect_under;
use crate::protocol::E_NAME_REFUSED;
use crate::server::handlers::host_target::is_direct;

const HANDLE_HOST: &str =
    include_str!("../../../../../capsule_net_sockets/src/server/handlers/connect/handle_host.rs");

#[test]
fn only_the_direct_route_is_direct() {
    assert!(is_direct(Some(route::DIRECT)));
    assert!(!is_direct(None), "a store that does not answer");
    for v in (0..=u8::MAX).filter(|v| *v != route::DIRECT) {
        assert!(!is_direct(Some(v)), "route {v}");
    }
    assert!(HANDLE_HOST.contains("get_u8(port, Field::NetworkRoute)"));
    assert!(HANDLE_HOST
        .contains("match host_target(kind == Kind::Mixnet, host, direct_chosen, resolve) {"));
}

#[test]
fn a_name_on_a_plain_socket_is_refused_unless_direct_is_chosen() {
    for mixnet in [false, true] {
        for host in [&b"example.com"[..], b"dl-cdn.alpinelinux.org"] {
            let (got, asked) = connect_under(false, mixnet, host, Ok([93, 184, 216, 34]));
            assert_eq!(got, Err(E_NAME_REFUSED), "mixnet={mixnet} {host:?}");
            assert_eq!(asked, 0, "net.dns was asked for {host:?} off Direct");
        }
    }
}

#[test]
fn an_address_goes_whatever_the_chosen_network() {
    for mixnet in [false, true] {
        let (got, asked) = connect_under(false, mixnet, b"10.0.2.2", Err(7));
        assert_eq!((got, asked), (Ok([10, 0, 2, 2]), 0));
    }
}
