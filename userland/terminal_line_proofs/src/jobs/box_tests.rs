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

//! The report's way from capsule_linux to the terminal, both ends as they
//! ship: the key the terminal names is the key the capsule reads, the
//! frames the capsule writes are all the terminal keeps, and nothing the
//! guest writes on the same channel is taken for the report.

use super::box_hold::{BoxHold, KEY_LEN, TAG};
use super::linux_frames::{frames, key_of, FRAME_MAX, TAG as LINUX_TAG};
use super::tool_argv::key_arg;

const KEY: [u8; KEY_LEN] = [
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
];

fn report() -> Vec<u8> {
    let mut text = b"BOX REPORT v1\n  refused  0\n".repeat(30);
    text.extend_from_slice(b"[BOX] v=1 preset=strict refused=0\n");
    text
}

#[test]
fn both_ends_agree_on_the_tag_and_the_key() {
    assert_eq!(TAG, LINUX_TAG);
    assert_eq!(key_of(&key_arg(&KEY)), Some(KEY));
}

#[test]
fn the_report_crosses_whole_and_only_the_report_is_held() {
    let mut hold = BoxHold::new(KEY);
    let mut guest = Vec::new();
    for f in frames(&KEY, &report()) {
        assert!(f.len() <= FRAME_MAX);
        // The guest's own frames, between the report's.
        for g in [
            &b"hello\n"[..],
            &[TAG],
            &[TAG, 0x00, 0x11],
            &[[TAG].as_slice(), &[0; KEY_LEN], b"x"].concat(),
        ] {
            assert!(!hold.take(g), "a guest frame was taken for the report");
            guest.push(g.to_vec());
        }
        assert!(hold.take(&f));
    }
    let (shown, line) = hold.lines();
    assert_eq!(line, Some(&b"[BOX] v=1 preset=strict refused=0"[..]));
    assert_eq!(shown.len(), 60);
    assert!(shown.iter().all(|l| !l.starts_with(b"[BOX]")));
}

#[test]
fn a_report_under_another_key_is_the_guests_and_shown_as_such() {
    let mut other = KEY;
    other[15] ^= 1;
    let mut hold = BoxHold::new(KEY);
    assert!(frames(&other, &report()).iter().all(|f| !hold.take(f)));
    assert_eq!(hold.lines(), (Vec::new(), None));
}
