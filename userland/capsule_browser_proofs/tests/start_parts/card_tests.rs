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

use capsule_browser_proofs::browser::start::card::{paint as card, Line};
use capsule_browser_proofs::browser::ui::start_geom::compute;
use nonos_toolkit::widgets::{Card, Ink};

use super::kit::{at_scale, ink_box, inside, paint, W};

const FIVE: [Line<'static>; 5] = [
    ("Open tabs: 12", Ink::Text2, Some("12")),
    ("Pages visited: 340", Ink::Text2, None),
    ("Downloads: 3 saved, 1 failed", Ink::Text2, None),
    ("Saved a-rather-long-file-name.tar.gz (12.4 MB)", Ink::Muted, None),
    ("History clears at shutdown", Ink::Muted, None),
];

#[test]
fn every_card_holds_its_lines_under_its_head_and_inside_its_body() {
    for q in [4, 8] {
        let _g = at_scale(q);
        for w in [700, W] {
            for r in compute(w, 900, 4, 0).cards {
                let bare = paint(|fb| card(fb, r, "This session", &[]));
                let full = paint(|fb| card(fb, r, "This session", &FIVE));
                let lines = ink_box(&bare, &full).expect("no lines");
                let head = Card::new().title("This session").head_h() as i32;
                assert!(inside(lines, r), "{lines:?} past {r:?} at {w}, {q}");
                assert!(lines.y >= r.y + head, "{lines:?} in the head of {r:?} at {q}");
                let body = ink_box(&paint(|_| {}), &bare).expect("no card");
                assert!(inside(body, r), "{body:?} past {r:?} at {q}");
            }
        }
    }
}
