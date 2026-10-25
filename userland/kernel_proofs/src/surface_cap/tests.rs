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

//! The cap at each panel the boot framebuffer can be: under 64 MiB it stays
//! 64 MiB; past it, the framebuffer's own size a page up, to 128 MiB.

use super::cap::{surface_cap, CEILING, FLOOR};

const MIB: u64 = 1024 * 1024;

/// A framebuffer `w` pixels wide with a `pitch`-byte row, `h` rows.
const fn fb(pitch: u64, h: u64) -> Option<u64> {
    Some(pitch * h)
}

#[test]
fn headless_and_panels_to_4k_keep_64_mib() {
    assert_eq!(FLOOR, 64 * MIB);
    assert_eq!(surface_cap(None), FLOOR);
    assert_eq!(surface_cap(fb(1920 * 4, 1080)), FLOOR);
    assert_eq!(surface_cap(fb(3840 * 4, 2160)), FLOOR);
}

#[test]
fn panels_of_5k_6k_and_8k_are_admitted_at_their_own_size() {
    // 5K (58,982,400 bytes) fits under the floor; 6K and 8K raise the cap.
    assert_eq!(surface_cap(fb(5120 * 4, 2880)), FLOOR);
    for (w, h) in [(6016u64, 3384u64), (7680, 4320)] {
        let len = w * 4 * h;
        let cap = surface_cap(fb(w * 4, h));
        assert!(cap >= len && cap < len + 4096, "{w}x{h}: cap {cap} for {len}");
        assert!(cap > FLOOR, "{w}x{h}");
    }
    assert_eq!(surface_cap(fb(7680 * 4, 4320)), 132_710_400);
}

#[test]
fn a_padded_8k_past_the_ceiling_is_held_to_it() {
    assert_eq!(CEILING, 128 * MIB);
    let padded = 8192 * 4 * 4320; // a pitch padded to 8192 pixels: 141,557,760 bytes
    assert!(padded > CEILING);
    assert_eq!(surface_cap(Some(padded)), CEILING);
    assert!(surface_cap(Some(padded)) < padded, "the padded 8K backing is refused");
    assert_eq!(surface_cap(Some(u64::MAX)), CEILING);
}

#[test]
fn the_framebuffer_is_rounded_up_to_a_page_at_the_boundary() {
    assert_eq!(surface_cap(Some(FLOOR)), FLOOR);
    assert_eq!(surface_cap(Some(FLOOR + 1)), FLOOR + 4096);
    assert_eq!(surface_cap(Some(FLOOR + 4096)), FLOOR + 4096);
    assert_eq!(surface_cap(Some(FLOOR + 4097)), FLOOR + 8192);
    assert_eq!(surface_cap(Some(CEILING - 1)), CEILING);
    assert_eq!(surface_cap(Some(CEILING + 1)), CEILING);
}
