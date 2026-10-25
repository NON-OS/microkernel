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

//! A pasted line into a stored value's field, whose rule is the policy
//! store's.

use nonos_app_skeleton::input::text::first_line;

use super::paste_rig::*;
use crate::text::edit_buffer::EditBuffer;
use crate::text::edit_paste::{value_char, Pasted};

#[test]
fn a_hostname_pastes_whole_and_the_clipboards_newline_stays_behind() {
    let clip = first_line(b"nonos-box.lan\n").expect("text");
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste(clip.text, 63, usize::MAX, value_char), Pasted::Whole);
    assert_eq!(held(&b), "nonos-box.lan");
}

#[test]
fn a_value_with_a_character_the_store_refuses_is_refused_whole() {
    let mut b = EditBuffer::empty();
    b.push(b'x');
    for bad in ["my host", "naïve", "a/b", "rm;ls"] {
        assert_eq!(b.paste(bad, 63, usize::MAX, value_char), Pasted::Refused, "{bad}");
        assert_eq!(held(&b), "x", "nothing of {bad} went in");
    }
}

#[test]
fn a_value_longer_than_its_field_is_cut_to_it() {
    let mut b = EditBuffer::empty();
    assert_eq!(b.paste(&"a".repeat(40), 32, usize::MAX, value_char), Pasted::Cut);
    assert_eq!(b.len, 32);
}
