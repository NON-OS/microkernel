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

//! Any frame a guest can write, held to the report's rule: taken only when
//! it opens with the tag and the whole key, and never a panic; and whole
//! only once its closing line came.

use super::box_hold::{BoxHold, HELD_MAX, KEY_LEN, TAG};
use super::linux_frames::frames;

#[test]
fn no_guest_frame_without_the_key_is_taken() {
    let key: [u8; KEY_LEN] = *b"k3y-for-the-box!";
    let mut hold = BoxHold::new(key);
    let mut s: u64 = 0x5eed_b0c5;
    let mut taken = 0;
    for _ in 0..300_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let len = (s % 40) as usize;
        let mut f: Vec<u8> = (0..len).map(|i| (s >> (i % 8 * 8)) as u8).collect();
        // Most frames start like a report's, some with part or all of the key.
        if s.is_multiple_of(3) && !f.is_empty() {
            f[0] = TAG;
            let n = ((s >> 20) as usize % (KEY_LEN + 1)).min(f.len() - 1);
            f[1..1 + n].copy_from_slice(&key[..n]);
        }
        let real = f.len() > KEY_LEN && f[0] == TAG && f[1..=KEY_LEN] == key;
        assert_eq!(hold.take(&f), real, "{f:?}");
        taken += usize::from(real);
    }
    assert!(taken > 1_000, "only {taken} real frames: the sweep tests refusals alone");
}

#[test]
fn a_report_past_the_bound_is_held_to_it() {
    let mut hold = BoxHold::new([9; KEY_LEN]);
    for f in frames(&[9; KEY_LEN], &vec![b'a'; HELD_MAX * 2]) {
        assert!(hold.take(&f));
    }
    assert_eq!(hold.lines().0.concat().len(), HELD_MAX);
}

#[test]
fn a_report_without_its_closing_line_is_not_whole() {
    const KEY: [u8; KEY_LEN] = [7; KEY_LEN];
    let mut hold = BoxHold::new(KEY);
    assert!(!hold.complete());
    let frame = |body: &[u8]| [&[TAG][..], &KEY, body].concat();
    assert!(hold.take(&frame(b"BOX REPORT v1\n  refused  0\n")));
    assert!(!hold.complete());
    assert!(hold.take(&frame(b"[BOX] v=1 refused=0\n")));
    assert!(hold.complete());
    assert_eq!(hold.ended, None);
}
