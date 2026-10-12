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

//! A listing reply never carries a uri its u16 length cannot say (rule 1):
//! the bytes past a wrapped length would read as rows the log never had.

use nonos_settle_proto::{listing_reply, parse_listing_reply, Listed};

fn row(i: u8, uri: &[u8]) -> Listed {
    Listed {
        listing_id: [1; 32],
        release_id: [i; 32],
        release_hash: [i ^ 0x80; 32],
        bond_id: [7; 32],
        block: 1_000 + i as u64,
        uri: uri.to_vec(),
    }
}

#[test]
fn a_uri_that_would_wrap_its_length_never_forges_rows() {
    /* 65 536 + 117 bytes: as u16 the length would read 117, and the rest
     * would parse as a forged row. */
    let mut forged = listing_reply(&[row(9, b"https://evil/x")]).unwrap()[1..].to_vec();
    let mut uri = vec![b'u'; 65_536 + 117];
    uri.append(&mut forged);
    let b = listing_reply(&[row(1, &uri), row(2, b"https://h/r"), row(3, b"")]).unwrap();
    assert_eq!(parse_listing_reply(&b, &[1; 32]), Some(vec![row(2, b"https://h/r")]));
}
