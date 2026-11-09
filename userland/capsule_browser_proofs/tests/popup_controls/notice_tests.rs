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

use capsule_browser_proofs::browser::omnibox::geometry::{bubble_at, bubble_h};
use capsule_browser_proofs::browser::short_name::{notice_panel, SHORT_NOTICE};
use capsule_browser_proofs::browser::ui::limits::MIN_CONTENT_W;
use nonos_toolkit::design::size::TOOLTIP_MAX_W;

use super::kit::{at_scale, changed, inside, paint, px, H, W};

const LINK: &str = "https://example.com/a/very/long/path/to/a/page/that/goes/on/and/on/and/on/and/on";

#[test]
fn the_bubble_sits_in_its_band_at_the_bottom_left_of_the_page() {
    let alert = LINK.repeat(4);
    for q in [4, 8] {
        let _g = at_scale(q);
        for (text, room) in [(LINK, W / 2), (alert.as_str(), W)] {
            let (tip, at) = bubble_at(text, room, W, H);
            assert!(at.x + at.w <= room && at.x < px(8), "{at:?} at {q}");
            assert!(at.y >= H - bubble_h() && at.y + at.h <= H, "in the band: {at:?} at {q}");
            let drawn = changed(&paint(|_| {}), &paint(|fb| tip.draw(fb, at)));
            assert!(!drawn.is_empty() && drawn.iter().all(|&p| inside(at, p)), "at {q}");
        }
        assert!(bubble_at(&alert, W, W, H).1.w > px(TOOLTIP_MAX_W), "an alert takes the width");
    }
}

#[test]
fn the_short_name_notice_is_drawn_whole_inside_the_page() {
    let narrowest = MIN_CONTENT_W as u32;
    for q in [4, 8] {
        let _g = at_scale(q);
        for w in [px(narrowest), narrowest, W] {
            let (toast, at) = notice_panel(SHORT_NOTICE, w);
            assert_eq!(toast.size(), toast.text_lines(64).size(), "no line cut at {w} at {q}");
            assert!(at.x + at.w <= w && at.y + at.h <= H, "{at:?} inside {w} at {q}");
            let drawn = changed(&paint(|_| {}), &paint(|fb| toast.draw(fb, at)));
            assert!(!drawn.is_empty() && drawn.iter().all(|&p| inside(at, p)), "at {q}");
        }
    }
}
