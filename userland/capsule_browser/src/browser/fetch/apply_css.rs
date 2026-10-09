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

mod css_cut;
mod css_fold;

use crate::browser::http::response::parse;
use crate::browser::state::State;
use crate::browser::url::Url;

use super::enqueue_imports::enqueue_imports;

/* Folds a finished stylesheet fetch into the page. The body (when a 200)
cascades after inline <style>; `raw` is None when the fetch failed, and
`sheet` is the fetched URL so its @import targets resolve against it.
The page is laid out again once, after the last queued sheet, fetched
or failed, which is also when the render-blocking hold in finish ends. */
pub(super) fn apply_css(state: &mut State, raw: Option<&[u8]>, sheet: Option<&Url>) {
    let body = raw.and_then(parse).filter(|r| r.status == 200).map(|r| r.body).unwrap_or_default();
    let text = alloc::string::String::from_utf8_lossy(&body);
    if !text.is_empty() {
        if let Some(sheet) = sheet {
            enqueue_imports(state, &text, sheet);
        }
    }
    let text = (!text.is_empty()).then_some(&*text);
    /* Past the wait (`style_hold`) the page is already shown: a sheet
     * that lands marks it for a restyle, which the tick makes at most once
     * a second; the last sheet restyles at once. */
    let last = css_fold::sheet_done(&mut state.page_css, &state.css_queue, text);
    let waiting = state.pool.live.iter().any(|f| f.css);
    if state.style_hold.over && text.is_some() && (waiting || !last) {
        state.style_hold.stale = true;
    } else if last {
        state.style_hold.stale = false;
        crate::browser::event::relayout(state);
    }
}
