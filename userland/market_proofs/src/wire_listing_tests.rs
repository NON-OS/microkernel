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

//! A listing in full and a release read back as the market wrote them.

use super::*;

#[test]
fn a_listing_in_full_reads_back_its_publisher_and_description() {
    let mut body = Vec::new();
    lp(&mut body, b"linux.htop");
    body.extend_from_slice(&[7; 32]);
    lp(&mut body, b"htop");
    lp(&mut body, b"Alpine");
    body.extend_from_slice(&[9; 32]);
    lp(&mut body, b"An interactive process viewer");
    body.extend_from_slice(&2u32.to_le_bytes());
    let app = parse_app(&body).unwrap();
    assert_eq!(app.name, b"htop");
    assert_eq!(app.publisher, b"Alpine");
    assert_eq!(app.description, b"An interactive process viewer");
    assert_eq!(app.releases, 2);
    for n in 0..body.len() {
        assert!(parse_app(&body[..n]).is_none(), "cut at {n}");
    }
}

#[test]
fn a_release_reads_back_what_the_market_encodes() {
    let mut rel = release("x86_64-linux", 0);
    rel.release_id = "linux.htop@3.3.0".into();
    rel.validation.note = "hashed 123 bytes".into();
    rel.required_capabilities = alloc::vec!["net".into(), "fs".into()];
    let body = encode(&rel);
    let got = parse_release(&body).unwrap();
    assert_eq!(got.version, b"linux.htop@3.3.0");
    assert_eq!(got.short_version(), b"3.3.0");
    assert_eq!(got.note, b"hashed 123 bytes");
    // The validator id ("v", five bytes framed), the eight-byte time and the
    // escrow fields (40) follow the note and are not read here: a cut
    // anywhere before them fails.
    let unread = 5 + 8 + 40;
    for n in 0..body.len() - unread {
        assert!(parse_release(&body[..n]).is_none(), "cut at {n}");
    }
}
