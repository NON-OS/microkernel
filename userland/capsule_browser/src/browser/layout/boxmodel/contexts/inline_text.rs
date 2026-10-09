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

use crate::browser::css::WhiteSpace;

use super::super::tree::BoxNode;
use super::inline_flow::Flow;
use super::inline_sink::{line_end, Sink};
use super::inline_split::{expand_tabs, runs};
use super::inline_word::{measure, word};

/* Collapsible white space: space, tab and the segment breaks. */
fn ws(ch: char) -> bool {
    matches!(ch, ' ' | '\t' | '\n' | '\r' | '\x0C')
}

/* Feed the text box `c` holding `t` through its white-space mode: collapse
 * runs of spaces into the flow's one pending space (normal, nowrap and
 * pre-line), keep them as text (pre, pre-wrap), and turn a preserved
 * newline into a forced break. */
pub(in super::super) fn text_items(c: &BoxNode, t: &str, flow: &mut Flow, sink: &mut dyn Sink) {
    let s = &c.style;
    let (keep, wrap, lines) = match s.white_space {
        WhiteSpace::Normal => (false, true, false),
        WhiteSpace::Nowrap => (false, false, false),
        WhiteSpace::PreLine => (false, true, true),
        WhiteSpace::Pre => (true, false, true),
        WhiteSpace::PreWrap => (true, true, true),
    };
    let mut space_w: Option<i32> = None;
    for (i, line) in t.split(|ch| lines && ch == '\n').enumerate() {
        if i > 0 {
            line_end(flow, sink);
        }
        if keep {
            /* pre-wrap may break after each run of spaces; pre never. */
            for run in runs(&expand_tabs(line.trim_end_matches('\r')), !wrap) {
                word(c, run, false, flow, sink);
                if wrap && run.starts_with(' ') {
                    flow.allow_break();
                }
            }
            continue;
        }
        for (j, w) in line.split(ws).enumerate() {
            if (j > 0 || w.is_empty()) && !t.is_empty() {
                let sw = *space_w.get_or_insert_with(|| measure(s, " ").max(1));
                flow.gap(sw, wrap, s.underline);
            }
            if !w.is_empty() {
                word(c, w, wrap, flow, sink);
            }
        }
    }
}
