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

//! The word that confirms erasing a disk, from the serial the device hands
//! the installer (`nonos_blk_client/src/disks/word.rs`). The installer took
//! the serial's last four bytes as a string slice, which panics when they
//! begin inside a multi-byte character, as in "1€€". Any serial now gives
//! its last four characters in lower case, or no word when they are not
//! four printable ASCII characters, and never stops the installer.

#[path = "../../nonos_blk_client/src/disks/word.rs"]
mod word;

use word::serial_word;

#[test]
fn an_ascii_serial_gives_its_last_four_characters_in_lower_case() {
    assert_eq!(serial_word("S3EVNX0K123456A").as_deref(), Some("456a"));
    assert_eq!(serial_word("AB12").as_deref(), Some("ab12"));
    assert_eq!(serial_word("ABC"), None);
    assert_eq!(serial_word(""), None);
}

#[test]
fn a_serial_ending_in_other_characters_gives_no_word_and_no_panic() {
    for serial in ["1\u{20ac}\u{20ac}", "\u{20ac}\u{20ac}", "abc\u{e9}", "12\u{0}4", "ab\ncd"] {
        assert_eq!(serial_word(serial), None, "{serial:?}");
    }
    assert_eq!(serial_word("\u{20ac}abcd").as_deref(), Some("abcd"));
}

#[test]
fn any_serial_the_device_could_send_gives_a_typable_word_or_none() {
    let mut s = 0x1357_9BDF_2468_ACE0u64;
    for _ in 0..100_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let bytes: Vec<u8> =
            (0..(s % 24) as usize).map(|i| (s >> (i % 8 * 8)) as u8 ^ i as u8).collect();
        let serial = String::from_utf8_lossy(&bytes);
        if let Some(w) = serial_word(&serial) {
            let tail: String =
                serial.chars().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect();
            assert_eq!(w, tail.to_ascii_lowercase());
            assert!(w.len() == 4 && w.bytes().all(|b| (b' '..=b'~').contains(&b)), "{w:?}");
        }
    }
}
