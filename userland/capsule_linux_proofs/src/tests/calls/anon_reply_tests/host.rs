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

//! A host goes as written, and an address as its dotted quad.

use crate::linux::net::anon_answer::{dotted, open_body};

#[test]
fn a_host_goes_as_written_and_an_address_as_its_dotted_quad() {
    assert_eq!(open_body(b"example.com", 443), Some(b"\xbb\x01example.com".to_vec()));
    assert_eq!(dotted([93, 184, 216, 34]), b"93.184.216.34".to_vec());
    assert_eq!(dotted([0, 0, 0, 0]), b"0.0.0.0".to_vec());
    assert_eq!(
        open_body(&dotted([255, 255, 255, 255]), 1),
        Some(b"\x01\x00255.255.255.255".to_vec())
    );
    /* What a BEGIN's "host:port" cannot carry as written. */
    assert_eq!(open_body(b"", 80), None);
    assert_eq!(open_body(&[b'a'; 254], 80), None);
    assert!(open_body(&[b'a'; 253], 80).is_some());
    assert_eq!(open_body(b"evil.com:25\0", 80), None);
    assert_eq!(open_body(b"a\0b", 80), None);
    assert_eq!(open_body(b"a:b", 80), None);
}
