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

//! A guest's connect by name on a mixnet socket never reaches net.dns.
//!
//! The guest's stream goes through the mixnet, but net.sockets used to
//! resolve the name it was handed through net.dns first, in the clear, for
//! every socket kind, so every host a guest program reached was named to
//! the resolver and the path. The name now travels nowhere: the mixnet's
//! frames carry an address, not a name, so the connect is refused and the
//! guest is told the network is unreachable. Held end to end here: the
//! capsule's own encoder, net.sockets' own reader, and its decision.

use crate::host_body::host_body;
use crate::net_ops::{NET_E_NAME_REFUSED, NET_E_NO_TRANSPORT};
use crate::protocol::{E_BAD_ADDR, E_NAME_REFUSED, E_NO_DNS, E_NO_TRANSPORT};
use crate::server::handlers::host_target::{host_target, lookup_status};
use crate::server::handlers::parse_host::parse;

/// Run a connect by host the way a guest's goes, counting what net.dns is
/// asked.
fn connect_by_host(
    mixnet: bool,
    host: &[u8],
    answer: Result<[u8; 4], u16>,
) -> (Result<[u8; 4], u16>, usize) {
    let body = host_body(9, 443, host).unwrap_or_default();
    let Some((_, _, name)) = parse(&body) else { panic!("the server reads what the capsule sent") };
    let mut asked = 0usize;
    let got = host_target(mixnet, name, |_| {
        asked += 1;
        answer
    });
    (got, asked)
}

#[test]
fn a_name_on_a_mixnet_socket_is_refused_and_net_dns_is_never_asked() {
    for host in [&b"dl-cdn.alpinelinux.org"[..], b"example.com", b"a", b"10.0.0.256", b"::1"] {
        let (got, asked) = connect_by_host(true, host, Ok([93, 184, 216, 34]));
        assert_eq!(got, Err(E_NAME_REFUSED), "{host:?}");
        assert_eq!(asked, 0, "net.dns was asked for {host:?}");
    }
}

#[test]
fn an_address_on_a_mixnet_socket_goes_as_before_without_a_lookup() {
    /* An address needs no lookup: it goes as it was given. */
    let (got, asked) = connect_by_host(true, b"151.101.2.132", Err(7));
    assert_eq!(got, Ok([151, 101, 2, 132]));
    assert_eq!(asked, 0);
}

#[test]
fn other_sockets_resolve_as_they_did() {
    let (got, asked) = connect_by_host(false, b"example.com", Ok([93, 184, 216, 34]));
    assert_eq!((got, asked), (Ok([93, 184, 216, 34]), 1));
    let (got, asked) = connect_by_host(false, b"nowhere.invalid", Err(7));
    assert_eq!((got, asked), (Err(E_BAD_ADDR), 1));
    let (got, asked) = connect_by_host(false, b"10.0.2.2", Err(7));
    assert_eq!((got, asked), (Ok([10, 0, 2, 2]), 0));
}

#[test]
fn the_capsule_reads_the_statuses_the_server_sends() {
    /* The guest is told ENETUNREACH for both: no route out, nothing sent. */
    assert_eq!(NET_E_NAME_REFUSED, E_NAME_REFUSED);
    assert_eq!(NET_E_NO_TRANSPORT, E_NO_TRANSPORT);
    assert_ne!(E_NAME_REFUSED, E_BAD_ADDR, "a refusal is not a name that failed to resolve");
}

/// A lookup net.dns could not make (unanswered, or no upstream answered it)
/// is no DNS server reachable, said apart from a name that has no address:
/// the browser tells the reader to check the network, not the spelling.
#[test]
fn no_dns_server_is_said_apart_from_a_name_with_no_address() {
    for dns in [6u16, 15] {
        let (got, asked) = connect_by_host(false, b"example.com", Err(dns));
        assert_eq!((got, asked), (Err(E_NO_DNS), 1), "net.dns said {dns}");
        assert_eq!(lookup_status(dns), E_NO_DNS);
    }
    for dns in [4u16, 7, 8, 9, 10] {
        assert_eq!(lookup_status(dns), E_BAD_ADDR, "net.dns said {dns}");
    }
    const { assert!(E_NO_DNS != E_NAME_REFUSED && E_NO_DNS != E_BAD_ADDR) };
}
