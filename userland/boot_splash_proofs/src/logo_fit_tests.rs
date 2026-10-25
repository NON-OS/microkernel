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

//! The layout around the firmware's logo: kept where it clears the text;
//! centred (as OVMF draws it) the panel and status line move below it, all
//! of it on screen and nothing overlapping; where that would not fit, the
//! logo is left out and the plain layout drawn.

use crate::layout::{panel, status, Area, GAP, PANEL_H, STATUS_GAP};
use crate::layout_fit::{layout, Layout};
use crate::logo::place::Spot;

const MODES: [(u32, u32); 6] =
    [(640, 480), (800, 600), (1024, 768), (1280, 720), (1280, 800), (1920, 1080)];

fn on(s: &Spot, (x, y, w, h): Area) -> bool {
    s.x < i64::from(x + w)
        && s.x + i64::from(s.w) > i64::from(x)
        && s.y < i64::from(y + h)
        && s.y + i64::from(s.h) > i64::from(y)
}

fn centred(w: u32, h: u32, side: u32) -> Spot {
    Spot { x: i64::from((w - side) / 2), y: i64::from((h - side) / 2), w: side, h: side }
}

#[test]
fn no_logo_or_one_clear_of_the_text_keeps_the_plain_layout() {
    for (w, h) in MODES {
        let plain = Layout { panel: panel(w, h), status: status(w, h), logo: false };
        assert_eq!(layout(None, w, h), plain);
        let corner = Spot { x: 8, y: 8, w: 64, h: 64 };
        assert_eq!(layout(Some(&corner), w, h), Layout { logo: true, ..plain });
    }
}

#[test]
fn a_centred_logo_moves_the_text_below_it_on_every_mode() {
    for (w, h) in MODES {
        let s = centred(w, h, 120);
        let l = layout(Some(&s), w, h);
        if h < 600 {
            assert_eq!(l, layout(None, w, h), "{w}x{h}: no room below");
            continue;
        }
        assert!(l.logo, "{w}x{h}: kept");
        assert_eq!(l.panel.1 as i64, s.y + 120 + GAP as i64, "{w}x{h}: panel below the logo");
        assert_eq!(l.status.1, l.panel.1 + PANEL_H + STATUS_GAP, "{w}x{h}");
        assert!(!on(&s, l.panel) && !on(&s, l.status), "{w}x{h}: nothing on the logo");
        assert!(l.status.1 + l.status.3 <= h, "{w}x{h}: on screen");
    }
}

#[test]
fn a_logo_too_tall_to_fit_above_the_text_is_left_out() {
    for (w, h) in MODES {
        let s = centred(w, h, h * 3 / 4);
        assert_eq!(layout(Some(&s), w, h), layout(None, w, h), "{w}x{h}");
    }
}
