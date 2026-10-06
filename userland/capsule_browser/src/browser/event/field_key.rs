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

use alloc::format;

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER};

use crate::browser::omnibox::{text_char, Change};
use crate::browser::state::State;

use super::field_room::{room, Room};
use super::field_value::field_text;
use super::relayout::relayout;
use super::submit_form::submit_form;

/* How long the notice that a field is full stays up. */
const NOTICE_MS: i64 = 4_000;

/* Typing into the focused field edits its value attribute, redraws, and
 * fires the input event. Enter submits (a textarea takes a newline). A
 * character that does not fit (field_room) is refused; when it is the
 * browser's own bound that refused it, the reader is told. */
pub(super) fn field_key(state: &mut State, id: usize, event: InputEvent) -> EventOutcome {
    let Some(node) = state.page_dom.as_ref().and_then(|d| d.nodes.get(id)) else {
        return EventOutcome::Idle;
    };
    let is_textarea = node.tag == "textarea";
    let mut value = state.page_dom.as_ref().map(|d| field_text(d, id)).unwrap_or_default();
    let typed = match event.code {
        KEY_ENTER if !is_textarea => {
            submit_form(state, id, None);
            return EventOutcome::Repaint;
        }
        KEY_ENTER => Some('\n'),
        KEY_BACKSPACE => None,
        code => match text_char(code, event.flags) {
            Some(c) => Some(c),
            None => return EventOutcome::Idle,
        },
    };
    let node = state.page_dom.as_ref().and_then(|d| d.nodes.get(id));
    let verdict = match (typed, node) {
        (Some(c), Some(node)) => Some((c, room(node, &value, c))),
        (Some(_), None) => return EventOutcome::Idle,
        (None, _) => None,
    };
    match verdict {
        None => {
            value.pop();
        }
        Some((c, Room::Fits)) => value.push(c),
        Some((_, Room::PageLimit)) => return EventOutcome::Idle,
        Some((_, Room::BrowserLimit(max))) => {
            let line = format!("This field holds at most {} KB of text here.", max / 1024);
            state.ui.notice = Some((line, nonos_libc::mk_uptime_ms() + NOTICE_MS));
            state.mark(Change::Bubble);
            return EventOutcome::Repaint;
        }
    }
    if let Some(dom) = state.page_dom.as_mut() {
        dom.set_attr(id, "value", value);
    }
    if let Some(engine) = state.engine.as_ref() {
        engine.dispatch_event(id as i32, "input");
        /* A handler that sends the page elsewhere is followed now. */
        super::script_nav::take_script_nav(state);
    }
    relayout(state);
    state.track.laid_print = None;
    state.mark(Change::Page);
    EventOutcome::Repaint
}
