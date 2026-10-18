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

//! The statuses net.sockets sends, read back by the capsule.

use super::refused::connect_by_host;
use crate::net_ops::{NET_E_NAME_REFUSED, NET_E_NO_TRANSPORT};
use crate::protocol::{E_BAD_ADDR, E_NAME_REFUSED, E_NO_DNS, E_NO_TRANSPORT};
use crate::server::handlers::host_target::lookup_status;

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
