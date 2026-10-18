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

//! The terminal's wrapped description, and the request bodies.

use super::*;

#[test]
fn a_description_is_cut_at_spaces_to_the_width() {
    use crate::terminal::wrap;
    let text = b"an interactive process viewer for the terminal";
    let lines = wrap(text, 20);
    assert!(lines.iter().all(|l| l.len() <= 20));
    assert_eq!(lines.join(&b' '), text.to_vec());
    // One word longer than the width is cut where it must be.
    let long = [b'x'; 45];
    assert_eq!(wrap(&long, 20).iter().map(|l| l.len()).collect::<Vec<_>>(), [20, 20, 5]);
    assert!(wrap(b"", 20).is_empty());
}

#[test]
fn request_bodies_are_length_prefixed_ids() {
    assert_eq!(listing_body(b"ab").unwrap(), [2, 0, 0, 0, b'a', b'b']);
    assert_eq!(pair_body(b"ab", b"").unwrap(), [2, 0, 0, 0, b'a', b'b', 0, 0, 0, 0]);
}
