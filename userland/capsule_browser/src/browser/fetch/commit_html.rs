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

//! Commit a freshly homed HTML DOM: give it an engine, plan its scripts in
//! document order, queue its stylesheets, and lay it out unless a
//! stylesheet is still to come.

use alloc::vec::Vec;

use crate::browser::event::relayout;
use crate::browser::js::{plan, PageScript, World};
use crate::browser::qjs_run::page_engine;
use crate::browser::state::State;
use crate::browser::url::join;

/// Install the page DOM in a new engine and set its scripts going. The DOM
/// must already sit in `state.page_dom`, its address for the page's life, so
/// the engine's pointer into it stays valid; the caller drops any prior
/// engine before replacing the DOM. The inert tree-walk world is set so the
/// timer and script-fetch pumps have something to read while the engine owns
/// page state.
pub fn commit_html(state: &mut State) {
    /* The document has to know where it came from before a script runs.
     * `location` is read during setup on most pages, and a relative href
     * resolved against the wrong address points somewhere else entirely. */
    let base = state.base.as_ref().map(|u| join(u, "")).unwrap_or_default();
    /* history.length counts this page's entry, already taken (committed). */
    state.note_history();
    let (engine, scripts) = match state.page_dom.as_mut() {
        Some(dom) => {
            dom.base = base;
            dom.viewport = (state.viewport_w, state.viewport_h);
            (page_engine(dom), plan(dom))
        }
        None => (None, Vec::new()),
    };
    let scripted = engine.is_some();
    /* A page with scripts and no engine to run them in (QuickJS could not
     * get its heap) used to be shown as if it had none: a page that half
     * works with no word of why. */
    if !scripted && !scripts.is_empty() {
        state.tell(alloc::string::String::from(SCRIPTS_NOT_RUN));
    }
    state.engine = engine;
    state.world = Some(World::empty());
    if scripted {
        queue_scripts(state, scripts);
    }
    /* Stylesheets are render-blocking: while one is queued, the layout
     * waits for it rather than being made and thrown away. Queueing is
     * idempotent, so a caller that queues again adds nothing. */
    super::enqueue_css::enqueue_css(state);
    /* Scripts wait for the stylesheets before them, as they do in every
     * browser: code that measures the page must find it styled. With no
     * stylesheet to wait for, the inline ones run here. */
    if !super::land::run_held(state) && state.css_queue.is_empty() {
        relayout(state);
    }
}

/// What the reader is told when a page's scripts cannot run at all.
pub const SCRIPTS_NOT_RUN: &str = "This page's scripts did not run: the script engine could not \
     start, most likely for want of memory. The page is shown without them; close other \
     programs and reload to try again.";

/*
 * Each script takes its place by its index in the plan. An inline one is
 * held with its text at once; an external one is fetched and held when it
 * lands. run_held runs whatever is held in index order and stops at the
 * first gap, so an inline script after an external one waits for it, and
 * the page's events fire once the last one has run.
 */
fn queue_scripts(state: &mut State, scripts: Vec<PageScript>) {
    state.pool.held.clear();
    state.pool.run_order = 0;
    state.pool.scripts = Some(scripts.len() as u32);
    state.script_queue.clear();
    for (order, step) in scripts.into_iter().enumerate() {
        let order = order as u32;
        match step {
            PageScript::Inline(text) => state.pool.held.push((order, text.into_bytes())),
            PageScript::External(src) => {
                let abs = match state.base.as_ref() {
                    Some(b) => join(b, &src),
                    None => src,
                };
                state.script_queue.push((order, abs));
            }
        }
    }
}
