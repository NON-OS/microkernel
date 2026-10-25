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

//! A fixed buffer holds whole characters only.

use nonos_app_skeleton::input::text::{last_char_len, push_char};

#[test]
fn a_fixed_buffer_takes_and_gives_back_whole_characters() {
    let mut buf = [0u8; 8];
    let mut len = 0;
    for ch in ['a', 'é', '€'] {
        len = push_char(&mut buf, len, 8, ch).expect("room");
    }
    assert_eq!(&buf[..len], "aé€".as_bytes());
    assert_eq!(push_char(&mut buf, len, 8, '日'), None, "three bytes into two");
    assert_eq!(push_char(&mut buf, len, 6, 'x'), None, "the cap is the field's, not the array's");
    assert_eq!(last_char_len(&buf[..len]), 3);
    len -= 3;
    assert_eq!(last_char_len(&buf[..len]), 2);
    len -= 2;
    assert_eq!(last_char_len(&buf[..len]), 1);
    assert_eq!(last_char_len(&[]), 0);
}
