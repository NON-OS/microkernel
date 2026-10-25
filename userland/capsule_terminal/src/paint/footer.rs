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

use nonos_toolkit::font::ttf::line_height;

#[path = "footer_draw.rs"]
mod footer_draw;
pub use footer_draw::draw_footer;

/// The hint bar: what to press, and what it does.
///
/// Two colours rather than one. A single shade for both makes the bar read as
/// one long sentence, which is why a reader stops seeing it at all; the key
/// carries the accent so the eye can find it and skip the rest.
const HINTS: &[(&str, &str)] = &[
    ("Tab", "complete"),
    ("|", "pipe"),
    (">", "redirect"),
    ("^W", "word"),
    ("^K", "kill"),
    ("^L", "clear"),
];

/// Space between a key and its label, and between one pair and the next. The
/// pair gap is wider so the pairs group before they read left to right.
const KEY_GAP: i32 = 5;
const PAIR_GAP: i32 = 18;

/// Nominal size of the hint text. The face clamps it up to the readable floor,
/// so the band has to be sized from the measured line box rather than from this.
const HINT_PX: f32 = 12.0;

/// Space above the hint line, and below its descenders.
const PAD_TOP: u32 = 3;
const PAD_BOT: u32 = 3;

/// Height the hint bar needs for the line it actually draws.
pub fn footer_h() -> u32 {
    PAD_TOP + line_height(HINT_PX).max(1) as u32 + PAD_BOT
}
