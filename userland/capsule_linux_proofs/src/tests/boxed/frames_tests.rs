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

//! The report's frames: each one a message the terminal reads whole, each
//! opening with TAG and the key, and together exactly the report.

use crate::boxed::frames::{frames, key_of, FRAME_MAX, KEY_ARG, KEY_LEN, TAG};

const KEY: [u8; KEY_LEN] = *b"0123456789abcdef";

#[test]
fn the_frames_carry_the_report_whole_under_the_key() {
    for len in [0, 1, FRAME_MAX - 17, FRAME_MAX - 16, 5_000] {
        let text: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
        let fs = frames(&KEY, &text);
        assert!(fs.iter().all(|f| f.len() <= FRAME_MAX && f[0] == TAG && f[1..17] == KEY));
        assert!(fs.iter().all(|f| f.len() > 1 + KEY_LEN), "an empty frame");
        let body: Vec<u8> = fs.iter().flat_map(|f| f[1 + KEY_LEN..].to_vec()).collect();
        assert_eq!(body, text);
    }
}

#[test]
fn only_a_well_formed_key_word_is_a_key() {
    let hex = "000102030405060708090a0b0c0d0eff";
    let word = [KEY_ARG, hex.as_bytes()].concat();
    let mut want = [0u8; KEY_LEN];
    (0..15).for_each(|i| want[i] = i as u8);
    want[15] = 0xff;
    assert_eq!(key_of(&word), Some(want));
    for bad in [&hex[..31], "000102030405060708090A0B0C0D0EFF", "000102030405060708090a0b0c0d0eg0"]
    {
        assert_eq!(key_of(&[KEY_ARG, bad.as_bytes()].concat()), None, "{bad}");
    }
    assert_eq!(key_of(&[b"--box-key".as_slice(), hex.as_bytes()].concat()), None);
    assert_eq!(key_of(&[KEY_ARG, hex.as_bytes(), b"0"].concat()), None);
}
