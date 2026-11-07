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

//! OUTPUT_RECTS: every output's place on the canvas, which together cover
//! the whole of each output, for the lock screen's cover.

use crate::display::layout_rule::plan;
use crate::display::output_rects::{encode, rects, REPLY_LEN};

/// Every pixel of every output lies in exactly one rectangle.
fn covered_once(r: &[(u32, u32, u32, u32)], outputs: &[(u32, u32, u32)]) {
    let mut x0 = 0;
    for &(w, h, _) in outputs {
        for (px, py) in
            [(x0, 0), (x0 + w - 1, 0), (x0, h - 1), (x0 + w - 1, h - 1), (x0 + w / 2, h / 2)]
        {
            let n = r
                .iter()
                .filter(|&&(x, y, rw, rh)| px >= x && py >= y && px < x + rw && py < y + rh)
                .count();
            assert_eq!(n, 1, "({px}, {py}) in {n} rectangles");
        }
        x0 += w;
    }
}

#[test]
fn one_output_or_mirrored_is_the_whole_canvas() {
    let (r, n) = rects(None, (1920, 1080));
    assert_eq!(&r[..n], &[(0, 0, 1920, 1080)]);
}

#[test]
fn extended_outputs_each_get_their_slice() {
    let p = plan((1920, 1080), 4, &[(2560, 1440, 4), (1280, 1024, 4)]).unwrap();
    let (r, n) = rects(Some(&p), p.size);
    assert_eq!(n, 3);
    assert_eq!(&r[..n], &[(0, 0, 1920, 1080), (1920, 0, 2560, 1440), (4480, 0, 1280, 1024)]);
    covered_once(&r[..n], &[(1920, 1080, 4), (2560, 1440, 4), (1280, 1024, 4)]);
}

#[test]
fn four_outputs_are_all_covered() {
    let heads = [(1920, 1080, 4), (1920, 1200, 4), (1366, 768, 4)];
    let p = plan((3840, 2160), 4, &heads).unwrap();
    let (r, n) = rects(Some(&p), p.size);
    assert_eq!(n, 4);
    covered_once(&r[..n], &[(3840, 2160, 4), (1920, 1080, 4), (1920, 1200, 4), (1366, 768, 4)]);
}

#[test]
fn the_reply_is_a_count_then_the_rectangles() {
    let body = encode(&[(0, 0, 800, 600), (800, 0, 1024, 768)]);
    assert_eq!(body.len(), REPLY_LEN);
    assert_eq!(REPLY_LEN, 68);
    assert!(body[36..].iter().all(|&b| b == 0), "the unused slots are zero");
    assert_eq!(&body[0..4], &2u32.to_le_bytes());
    assert_eq!(&body[4..8], &0u32.to_le_bytes());
    assert_eq!(&body[12..16], &800u32.to_le_bytes());
    assert_eq!(&body[20..24], &800u32.to_le_bytes());
    assert_eq!(&body[28..32], &1024u32.to_le_bytes());
    assert_eq!(&body[32..36], &768u32.to_le_bytes());
}

#[test]
fn only_the_shell_is_answered() {
    let src = include_str!("../../../../compositor/src/server/handlers/output_rects.rs");
    assert!(src.contains("if !super::shell_only::from_shell(sender) {"));
    let ops = include_str!("../../../../compositor/src/protocol/ops.rs");
    assert!(ops.contains("pub const OP_OUTPUT_RECTS: u16 = 0x0021;"));
}
