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

//! Records at the edges: all padding, no content, zeros inside.

use crate::inner_plain::split;

/*
 * A record that is entirely padding has no content type, so there is nothing to
 * dispatch on and it must be refused. RFC 8446 requires the peer to send an
 * unexpected_message alert for exactly this, and silently treating it as some
 * default type would be the wrong kind of forgiving.
 */
#[test]
fn a_record_of_nothing_but_padding_has_no_type() {
    assert!(split(&[]).is_none(), "an empty record");
    for len in 1..8usize {
        assert!(split(&vec![0u8; len]).is_none(), "{len} zero bytes carry no type");
    }
}
#[test]
fn a_record_may_carry_a_type_and_no_content() {
    let (content, kind) = split(&[21]).expect("alert with no body");
    assert!(content.is_empty());
    assert_eq!(kind, 21);
}
#[test]
fn zeros_inside_the_content_are_kept() {
    let (content, kind) = split(&[0, 0, 5, 0, 0, 22, 0, 0]).expect("split");
    assert_eq!(content, &[0, 0, 5, 0, 0]);
    assert_eq!(kind, 22);
}
