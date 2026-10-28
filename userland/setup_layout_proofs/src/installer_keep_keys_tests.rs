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

//! The update screens' keys fit side by side as every screen's do, full
//! screen and in the window, at the size the footer picks for them.

use nonos_toolkit::decorations::content_rect;

use crate::displays::canvases;
use crate::installer::full::layout::layout;
use crate::installer::metrics::{Metrics, WIN_H, WIN_W};
use crate::installer::text::keys_px;
use crate::label_w;
use crate::literals::literals;

const PREPARE: &str = "P prepare to update   Enter see the proofs";
const PAIRS: [(&str, &str); 6] = [
    ("Esc back", "\u{2191}\u{2193} choose   type the word, then Enter"),
    ("Esc back", "\u{2191}\u{2193} choose"),
    ("Esc stop, your data stays as it is", "do not power off"),
    ("", "writing the boot sector, do not power off"),
    ("Esc close", PREPARE),
    ("Esc leave, the desktop starts", PREPARE),
];

fn caps(s: &str) -> String {
    s.to_ascii_uppercase()
}

#[test]
fn the_update_keys_never_run_into_each_other() {
    let shipped = literals("../capsule_install/src/install/ui", false);
    let m = Metrics::window();
    let window = content_rect(WIN_W, WIN_H, false).w - 2 * m.pad;
    let mut rooms = vec![("the window".to_string(), window, m.inset, m.small_px, m.label_px)];
    for c in canvases() {
        let l = layout(c.width, c.height);
        let at = format!("{}x{}", c.width, c.height);
        rooms.push((at, l.col_w, 2 * l.unit, l.caption_px, l.label_px));
    }
    for (left, right) in PAIRS {
        for s in [left, right].into_iter().filter(|s| !s.is_empty()) {
            assert!(shipped.iter().any(|l| l.text == s), "\"{s}\" is not in hints.rs");
        }
        for (at, room, gap, px, small) in &rooms {
            let size = keys_px(left, right, *room, *gap, *px, *small);
            let need = label_w(&caps(left), size) + label_w(&caps(right), size) + gap;
            assert!(need <= *room, "{at}: \"{left}\" and \"{right}\" need {need}, have {room}");
        }
    }
}
