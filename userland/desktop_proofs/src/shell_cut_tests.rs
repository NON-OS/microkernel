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

//! The shell's labels cut to a width by the toolkit's `clip_by` and
//! `clip_tail_by` over the shell's own measure, as its private cuts did. The
//! scale test asks at each scale it sets, so the scale holds still.

use nonos_toolkit::widgets::text;

use crate::render::measure_aa::measure_aa;
use crate::shell_apps::LAUNCHER_APPS;
use crate::ui_font::{valid_str, LABEL_PX, UI_PX};

const NAMES: [&str; 5] =
    ["Search applications...", "notes.txt", "Quarterly report final.pdf", "café ünïcode ✓", ""];

fn old_head(text: &str, px: f32, max_w: u32) -> &str {
    if measure_aa(text, px) <= max_w {
        return text;
    }
    let mut fit = 0;
    for (i, _) in text.char_indices() {
        if measure_aa(&text[..i], px) > max_w {
            break;
        }
        fit = i;
    }
    &text[..fit]
}

fn old_tail(text: &str, px: f32, max_w: u32) -> &str {
    let mut start = text.len();
    for (i, _) in text.char_indices().rev() {
        if measure_aa(&text[i..], px) > max_w {
            break;
        }
        start = i;
    }
    &text[start..]
}

/// Every app label and sample name, at every width up to its own and past it,
/// cuts to the same start and the same end at the scale in force.
pub(crate) fn cuts_agree_with_the_old_cut(tag: &str) {
    let labels = LAUNCHER_APPS.iter().map(|a| valid_str(a.label));
    for name in labels.chain(NAMES) {
        for px in [LABEL_PX, UI_PX] {
            let full = measure_aa(name, px);
            for max_w in (0..=full + 4).step_by(3) {
                let m = |t: &str| measure_aa(t, px) as i32;
                let head = text::clip_by(name, max_w as i32, m);
                assert_eq!(head, old_head(name, px, max_w), "{tag}: {name:?} at {max_w}");
                let tail = text::clip_tail_by(name, max_w as i32, m);
                assert_eq!(tail, old_tail(name, px, max_w), "{tag}: {name:?} end at {max_w}");
            }
        }
    }
}
