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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ESC};

use super::super::state::State;
use super::event_apply::apply;
use super::event_scroll::scroll_key;
use crate::pm::ui::keys::Act;
use crate::pm::ui::keys_table::act_for;

pub fn key(state: &mut State, code: u32) -> EventOutcome {
    // The overlay answers two keys and swallows the rest. A panel that let the
    // table scroll underneath it would be a panel the reader has to fight.
    if state.help_open {
        if code == KEY_ESC || code == 0x3F {
            state.help_open = false;
            return EventOutcome::Repaint;
        }
        return EventOutcome::Idle;
    }
    if code == 0x3F && !state.query_focused() {
        state.help_open = true;
        return EventOutcome::Repaint;
    }
    if state.query_focused() {
        if let Some(outcome) = typing(state, code) {
            return outcome;
        }
    }
    if let Some(act) = ordinal(code) {
        apply(state, act);
        return EventOutcome::Repaint;
    }
    if scroll_key(state, code) {
        return EventOutcome::Repaint;
    }
    let Some(act) = act_for(code) else {
        return if code == KEY_ESC { EventOutcome::Close } else { EventOutcome::Idle };
    };
    apply(state, act);
    EventOutcome::Repaint
}

// While the field owns the keyboard, printable ASCII edits the query instead of
// firing a letter shortcut, and Escape empties the field rather than the window.
// Every navigation code is >= 0x1201 and so falls through untouched, which is
// what lets the arrows keep driving the selection while the user types.
fn typing(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        KEY_ESC => {
            state.clear_query();
            state.focus_search(false);
        }
        KEY_BACKSPACE => state.pop_char(),
        0x20..=0x7E => state.push_char(code as u8),
        _ => return None,
    }
    Some(EventOutcome::Repaint)
}

// The digit row selects a screen by the position its nav row is drawn at. It
// goes through the same `apply` as every other key rather than reaching for the
// state directly, so `Act::Screen` is a case something actually dispatches
// instead of a variant the table mentions and nothing produces.
fn ordinal(code: u32) -> Option<Act> {
    (0x31..=0x36).contains(&code).then(|| Act::Screen((code - 0x31) as usize))
}
