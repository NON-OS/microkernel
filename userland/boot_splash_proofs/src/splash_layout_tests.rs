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

//! The status line repaints its band whole and nothing outside it, on every
//! frame and every mode, so the damage the splash sends for that band alone
//! shows each frame whole.

use crate::layout::status;
use crate::status::status as paint_status;

const MODES: [(u32, u32); 6] =
    [(640, 480), (800, 600), (1024, 768), (1280, 720), (1280, 800), (1920, 1080)];
const SENTINEL: u32 = 0x1234_5678;

fn frame(w: u32, h: u32, n: u32, from: &[u32]) -> Vec<u32> {
    let mut buf = from.to_vec();
    paint_status(&mut buf, w as usize, w, h, status(w, h), n);
    buf
}

#[test]
fn the_status_line_touches_nothing_outside_its_band() {
    for (w, h) in MODES {
        let (_, y0, _, rows) = status(w, h);
        let fresh = vec![SENTINEL; (w * h) as usize];
        for n in 0..8 {
            let buf = frame(w, h, n, &fresh);
            for (i, &px) in buf.iter().enumerate() {
                let y = i as u32 / w;
                let inside = (y0..y0 + rows).contains(&y);
                assert!(
                    inside || px == SENTINEL,
                    "{w}x{h} frame {n}: row {y} painted outside the band"
                );
                assert!(
                    !inside || px != SENTINEL,
                    "{w}x{h} frame {n}: row {y} of the band left as it was"
                );
            }
        }
    }
}

#[test]
fn each_frame_is_whole_whatever_came_before() {
    for (w, h) in MODES {
        let fresh = vec![SENTINEL; (w * h) as usize];
        for n in 1..6 {
            assert_eq!(
                frame(w, h, n, &frame(w, h, n - 1, &fresh)),
                frame(w, h, n, &fresh),
                "{w}x{h} frame {n}"
            );
        }
    }
}
