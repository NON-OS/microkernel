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

//! What the capsule sends to connect by host is what net.sockets reads.
//! The capsule once sent the host length as one byte, which the server read
//! as two, so every connect was refused as a bad length; the two ends are
//! now tested together.

use crate::host_body::host_body;
use crate::server::handlers::parse_host::parse;

#[test]
fn the_server_reads_back_what_the_capsule_sent() {
    for (handle, port, host) in [
        (7u32, 8080u16, &b"10.0.2.2"[..]),
        (0xDEAD_BEEF, 443, b"kali.download"),
        (1, 80, b"a"),
        (2, 1, &[b'x'; 253][..]),
    ] {
        let body = host_body(handle, port, host).expect("encodes");
        assert_eq!(parse(&body), Some((handle, port, host)));
    }
}

#[test]
fn a_host_the_server_would_refuse_is_never_sent() {
    assert_eq!(host_body(1, 80, b""), None);
    assert_eq!(host_body(1, 80, &[b'x'; 254]), None);
}

#[test]
fn the_old_one_byte_length_is_what_the_server_refused() {
    let mut old = vec![7, 0, 0, 0, 0x90, 0x1F, 8];
    old.extend_from_slice(b"10.0.2.2");
    assert_eq!(parse(&old), None);
}
