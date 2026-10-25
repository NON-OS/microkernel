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

//! OP_GET_APP read for a purchase: the id echoed, the name and the
//! publisher's key, and nothing past the release count.

use alloc::vec::Vec;

use super::parse_app_ids;

fn lp(o: &mut Vec<u8>, b: &[u8]) {
    o.extend_from_slice(&u32::try_from(b.len()).unwrap_or(0).to_le_bytes());
    o.extend_from_slice(b);
}

fn reply() -> Vec<u8> {
    let mut o = Vec::new();
    lp(&mut o, b"linux.notes");
    o.extend_from_slice(&[7; 32]);
    lp(&mut o, b"Notes");
    lp(&mut o, b"A Publisher");
    o.extend_from_slice(&[0x2B; 32]);
    lp(&mut o, b"What it does.");
    o.extend_from_slice(&2u32.to_le_bytes());
    o
}

#[test]
fn the_id_name_and_publisher_key_are_read_from_the_whole_reply() {
    let a = parse_app_ids(&reply()).unwrap_or_else(|| panic!("reads"));
    assert_eq!(
        (&a.id[..], &a.name[..], a.publisher_key),
        (&b"linux.notes"[..], &b"Notes"[..], [0x2B; 32])
    );
    let long = [reply().as_slice(), &[0]].concat();
    assert_eq!(parse_app_ids(&long), None, "a byte past the count");
    let r = reply();
    for n in 0..r.len() {
        assert_eq!(parse_app_ids(r.get(..n).unwrap_or_default()), None, "cut at {n}");
    }
}
