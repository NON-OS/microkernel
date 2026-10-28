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

//! The class of an exit's refusal (tunnel/refusal.rs), read from the
//! messages a Nym network requester and the sockets under it give, and
//! nothing read from a reply that is not a connection error.

use crate::tunnel::{
    refusal, Refusal, INTERFACE_VERSION, PROTOCOL_VERSION, RESP_CONNECTION_ERROR,
    RESP_NETWORK_DATA, TAG_PROVIDER_DATA,
};

fn reply(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![INTERFACE_VERSION, TAG_PROVIDER_DATA, PROTOCOL_VERSION, kind];
    out.extend_from_slice(&7u64.to_be_bytes());
    out.extend_from_slice(body);
    out
}

#[test]
fn each_refusal_reads_as_its_class() {
    let cases: [(&[u8], Refusal); 7] = [
        (b"could not resolve socket address for 'node.invalid:18089'", Refusal::Dns),
        (b"failed to lookup address information: Name or service not known", Refusal::Dns),
        (
            b"the provided socket address, '1.2.3.4:25' is not covered by the exit policy!",
            Refusal::Policy,
        ),
        (b"connection timed out", Refusal::Timeout),
        (b"Connection refused (os error 111)", Refusal::Refused),
        (b"Network is unreachable (os error 101)", Refusal::Unreachable),
        (b"something no class names", Refusal::Other),
    ];
    for (message, class) in cases {
        assert_eq!(refusal(&reply(RESP_CONNECTION_ERROR, message)), Some(class));
    }
}

#[test]
fn only_a_connection_error_has_a_class() {
    assert_eq!(refusal(&reply(RESP_NETWORK_DATA, b"connection refused")), None);
    let mut outside = reply(RESP_CONNECTION_ERROR, b"refused");
    outside[1] = TAG_PROVIDER_DATA + 1;
    assert_eq!(refusal(&outside), None);
    assert_eq!(refusal(&reply(RESP_CONNECTION_ERROR, b"")[..11]), None, "a cut id");
    assert_eq!(refusal(&reply(RESP_CONNECTION_ERROR, b"")), Some(Refusal::Other));
}
