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

//! A NONOS key code is sent as the evdev key it came from on a US board,
//! the same key whichever character shift made of it.

use crate::evdev::evdev_of;

#[test]
fn letters_are_their_keys_in_either_case() {
    let qwerty = [("qwertyuiop", 16), ("asdfghjkl", 30), ("zxcvbnm", 44)];
    for (row, first) in qwerty {
        for (i, c) in row.bytes().enumerate() {
            let want = Some(first + i as u32);
            assert_eq!(evdev_of(c.into()), want, "{}", c as char);
            assert_eq!(evdev_of(c.to_ascii_uppercase().into()), want, "{}", c as char);
        }
    }
}

#[test]
fn the_digit_row_and_its_shifted_symbols_share_keys() {
    for (plain, shifted, key) in
        [(b'1', b'!', 2), (b'5', b'%', 6), (b'9', b'(', 10), (b'0', b')', 11)]
    {
        assert_eq!((evdev_of(plain.into()), evdev_of(shifted.into())), (Some(key), Some(key)));
    }
    for (plain, shifted, key) in
        [(b'-', b'_', 12), (b'\'', b'"', 40), (b'/', b'?', 53), (b'`', b'~', 41)]
    {
        assert_eq!((evdev_of(plain.into()), evdev_of(shifted.into())), (Some(key), Some(key)));
    }
    let controls = [(0x1B, 1), (0x08, 14), (b'\t' as u32, 15), (0x0D, 28), (b' ' as u32, 57)];
    controls.iter().for_each(|(code, key)| assert_eq!(evdev_of(*code), Some(*key), "{code:#x}"));
}
