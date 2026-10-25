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

//! What sits to the left of the line being typed.

use nonos_app_skeleton::PaintBuffer;

use super::line_text::text;
use super::metrics::Metrics;
use super::search_head::{head, width};
use super::user_host::draw_id;
use crate::term::state::State;
use crate::term::theme::types::Theme;

/// Draw the prompt, and say how many cells it took.
///
/// Two heads, because the line means different things in the two modes. Under
/// a search the location is replaced by what is being searched for: the
/// location is not what the reader is looking at, and the line below is a
/// match rather than something they typed.
pub fn draw_prompt(
    state: &State,
    fb: &mut PaintBuffer,
    ox: u32,
    y: u32,
    m: Metrics,
    room: usize,
    t: &Theme,
) -> usize {
    let (adv, px) = (m.adv, m.px);
    let mut home_buf = [0u8; 160];
    let cwd = crate::term::cwd::shorten(
        state.cwd.as_bytes(),
        crate::term::cwd::home_var(state),
        &mut home_buf,
    );
    let take = cwd.len().min(room.max(1));

    if let Some(search) = &state.search {
        let (label, shown) = head(&search.needle, search.found, room.max(1));
        let tone = if search.found { t.dim } else { t.err };
        text(fb, ox, y, label, tone, adv, px);
        text(fb, ox + label.len() as u32 * adv, y, shown, t.accent, adv, px);
        return width(label, shown);
    }

    // The mark takes the colour of what the last command did, so a reader who
    // looked away while it ran learns the outcome where they are about to
    // type rather than by finding the block it came from.
    // Who, where, then the mark and a space: `ek@nonos ~/src % `. Read the
    // other way round, `>~`, it looked like a redirect into a file named `~`.
    let mark = if state.last_status == 0 { t.accent } else { t.err };
    let id = draw_id(fb, ox, y, adv, px, room.max(1), t);
    let take = take.min(room.max(1) - id);
    let x = ox + id as u32 * adv;
    text(fb, x, y, &cwd[cwd.len() - take..], t.path, adv, px);
    text(fb, x + (take as u32 + 1) * adv, y, b"%", mark, adv, px);
    id + take + 3
}
