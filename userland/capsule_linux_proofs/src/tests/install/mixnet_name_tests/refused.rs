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

//! A name on a mixnet socket is refused; an address, and other sockets, go as before.

use crate::host_body::host_body;
use crate::protocol::{E_BAD_ADDR, E_NAME_REFUSED};
use crate::server::handlers::host_target::host_target;
use crate::server::handlers::parse_host::parse;

/// Run a connect by host the way a guest's goes, counting what net.dns is
/// asked.
pub(super) fn connect_by_host(
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
