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
//! The no-IOMMU warning's row and its drawing: always between the panel
//! and the status line, 14 px from the panel and at least 14 from the
//! status line, never under a kept logo, and drawn in red inside its row.

use crate::layout::{logo_clear, warning, STATUS_ROWS};
use crate::layout_fit::layout;
use crate::logo::place::Spot;

const MODES: [(u32, u32); 7] =
    [(640, 480), (800, 600), (1024, 768), (1280, 720), (1280, 800), (1920, 1080), (3840, 2160)];

fn spots(w: u32, h: u32) -> [Option<Spot>; 4] {
    let c = |side: u32| Spot {
        x: ((w - side) / 2) as i64,
        y: ((h - side) / 2) as i64,
        w: side,
        h: side,
    };
    [None, Some(c(120)), Some(Spot { x: 8, y: 8, w: 64, h: 64 }), Some(c(h * 3 / 4))]
}

#[test]
fn the_row_sits_between_panel_and_status_on_every_mode() {
    for (w, h) in MODES {
        for s in spots(w, h) {
            let l = layout(s.as_ref(), w, h);
            let (p, r, st) = (l.panel, l.warn, l.status);
            assert_eq!(r, (0, p.1 + p.3 + 14, w, STATUS_ROWS), "{w}x{h}");
            assert!(r.1 + r.3 + 14 <= st.1, "{w}x{h}: clear of the status line");
            assert!(r.1 + r.3 <= h, "{w}x{h}: on screen");
            if let (Some(s), true) = (s, l.logo) {
                assert!(s.y + s.h as i64 <= r.1 as i64 || s.y >= (r.1 + r.3) as i64, "{w}x{h}");
            }
        }
    }
}

#[test]
fn a_logo_on_the_row_alone_is_not_clear() {
    let (w, h) = (1280, 800);
    let r = warning(crate::layout::panel(w, h), w);
    // Beside the panel, in the row: clear of panel and status, not of the row.
    let s = Spot { x: 8, y: r.1 as i64 + 2, w: 32, h: 8 };
    assert!(!logo_clear(&s, w, h));
    assert!(logo_clear(&Spot { y: r.1 as i64 - 9, ..s }, w, h));
}
