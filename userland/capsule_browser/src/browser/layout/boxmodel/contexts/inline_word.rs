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

use crate::browser::css::{Computed, WhiteSpace};

use super::super::text_transform::transform;
use super::super::tree::BoxNode;
use super::inline_flow::Flow;
use super::inline_parts::Lead;
use super::inline_sink::Sink;
use super::inline_split::soft_pieces;

/* Text measurements made through `measure` on this thread, counted in the
 * host harness so a proof can show intrinsic sizing stays linear in
 * nesting depth. The capsule build has no harness and no counter. */
#[cfg(feature = "harness")]
std::thread_local!(pub(crate) static MEASURES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) });

/* Advance of `w` in the text style `s`. */
pub(in super::super) fn measure(s: &Computed, w: &str) -> i32 {
    #[cfg(feature = "harness")]
    MEASURES.with(|m| m.set(m.get() + 1));
    let px = s.font_size_px as f32;
    crate::browser::fonts::measure_text(s.font_key, s.mono, s.bold, w, px, s.letter_spacing)
}

/* One source word, transformed, split where a line may wrap inside it. */
pub(in super::super) fn word(
    c: &BoxNode,
    w: &str,
    wrap: bool,
    flow: &mut Flow,
    sink: &mut dyn Sink,
) {
    let s = &c.style;
    let text = transform(w, s.text_transform, flow.word_start(), s.icon_font);
    for (i, piece) in soft_pieces(&text, wrap).enumerate() {
        if i > 0 {
            flow.allow_break();
        }
        let lead = flow.lead(false, wrap || s.white_space == WhiteSpace::PreWrap);
        let lead = Lead { join: i > 0, ..lead };
        sink.word(c, piece.into(), measure(s, piece).max(0), lead);
    }
}
