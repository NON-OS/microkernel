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

//! A select's list in the window: opened by a click on the select (or its
//! label), worked with the pointer and the keyboard, and closed by a
//! choice, Esc, Tab, a click elsewhere or a scroll.

use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC, KEY_HOME, KEY_PAGE_DOWN,
    KEY_PAGE_UP, KEY_TAB, KEY_UP,
};
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use crate::browser::omnibox::Change;
use crate::browser::state::State;
use crate::browser::ui::select_menu::{open_at, place, row_at};

use super::field_value::choose_option;
use super::select_list::{SelectList, MAX_ROWS};

/* Open the list of select `id`, the option it shows highlighted. False
 * when it has nothing to choose or is disabled, and nothing opens. */
pub(super) fn open_select(state: &mut State, id: usize) -> bool {
    let Some(mut list) = state.page_dom.as_ref().and_then(|d| SelectList::open(d, id)) else {
        return false;
    };
    if let Some(select) = select_rect(state, id) {
        open_at(&mut list, select, page_area(state));
    }
    state.ui.select = Some(list);
    state.mark(Change::Page);
    true
}

/// Close the list, if one is open, and take it off the page.
pub(super) fn close_select(state: &mut State) {
    if state.ui.select.take().is_some() {
        state.mark(Change::Page);
    }
}

/// Where the open list is drawn, in page-area coordinates; None when no
/// list is open or its select is no longer laid out.
pub fn placed(state: &State) -> Option<Rect> {
    let list = state.ui.select.as_ref()?;
    Some(place(list, select_rect(state, list.select)?, page_area(state)))
}

/* Where select `id` is on screen, in page-area coordinates, its part
 * scrolled above the page cut off. */
fn select_rect(state: &State, id: usize) -> Option<Rect> {
    let r = *state.page_dom.as_ref()?.rects.get(id).filter(|r| **r != [0; 4])?;
    let top = r[1] - state.scroll as i32;
    let y = top.max(0);
    let (x, w, h) = (r[0].max(0), r[2].max(0), (top + r[3] - y).max(0));
    Some(Rect { x: x as u32, y: y as u32, w: w as u32, h: h as u32 })
}

fn page_area(state: &State) -> Rect {
    Rect { x: 0, y: 0, w: state.viewport_w, h: state.viewport_h }
}

/* A click while the list is open, on the page area: a row chooses its
 * option, one on a heading or a disabled option does nothing, and a click
 * off the list only closes it, as a click outside a dropdown does in other
 * browsers. */
pub(super) fn select_click(state: &mut State, event: InputEvent) -> EventOutcome {
    let (x, y) = (event.x, event.y);
    let row = match (placed(state), state.ui.select.as_mut()) {
        (Some(at), Some(list)) => {
            list.pointer.pressed(x, y);
            contains(at, x, y).then(|| row_at(list, at, x, y))
        }
        _ => None,
    };
    match row {
        Some(None) => state.mark(Change::Page),
        Some(Some(i)) => {
            if let Some(list) = state.ui.select.as_mut() {
                list.hi = i;
            }
            if let Some(option) = state.ui.select.as_ref().and_then(|l| l.highlighted()) {
                choose(state, option);
            }
        }
        None => close_select(state),
    }
    EventOutcome::Repaint
}

/* A key while the list is open. Up and Down move the highlight over the
 * options that can be chosen, Page Up and Page Down a list's height, Home
 * and End to either end; Enter or Space chooses; Esc and Tab close. Every
 * other key is the list's while it is open, as in other browsers. */
pub(super) fn select_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Some(list) = state.ui.select.as_mut() else { return EventOutcome::Idle };
    let page = MAX_ROWS as i32 - 1;
    match event.code {
        KEY_UP => list.step(-1),
        KEY_DOWN => list.step(1),
        KEY_PAGE_UP => list.step(-page),
        KEY_PAGE_DOWN => list.step(page),
        KEY_HOME => list.to_end(false),
        KEY_END => list.to_end(true),
        KEY_ENTER | 0x20 => {
            if let Some(option) = list.highlighted() {
                choose(state, option);
            }
            return EventOutcome::Repaint;
        }
        KEY_ESC | KEY_TAB => {
            close_select(state);
            return EventOutcome::Repaint;
        }
        _ => return EventOutcome::Idle,
    }
    state.mark(Change::Page);
    EventOutcome::Repaint
}

/* The pointer over the open list highlights the row under it. Answers
 * whether the list must be drawn again: the highlight moved, or a press is
 * held, whose row lights only while the pointer stays on it. */
pub(super) fn select_hover(state: &mut State, x: i32, y: i32) -> bool {
    let at = placed(state);
    let Some(list) = state.ui.select.as_mut() else { return false };
    list.pointer.moved(x, y);
    let held = list.pointer.state(|_, _| true).pressed;
    let row = at.and_then(|at| row_at(list, at, x, y)).filter(|&i| i != list.hi);
    if let Some(i) = row {
        list.hi = i;
    }
    if row.is_some() || held {
        state.mark(Change::Page);
    }
    row.is_some() || held
}

/* The button came up, or the pointer left the page: no row is held. */
pub(super) fn select_let_go(state: &mut State, left: bool) {
    let Some(list) = state.ui.select.as_mut() else { return };
    let held = list.pointer.state(|_, _| true).pressed;
    match left {
        true => list.pointer.left(),
        false => list.pointer.released(),
    }
    if held {
        state.mark(Change::Page);
    }
}

/* The reader chose `option`. A select taking one choice shows it and the
 * list closes; in a `multiple` one it turns on or off and the list stays.
 * When what the select sends changed, the page hears input and change, as
 * for a checkbox (toggle_field::heard_change). */
fn choose(state: &mut State, option: usize) {
    let Some((select, multiple)) = state.ui.select.as_ref().map(|l| (l.select, l.multiple)) else {
        return;
    };
    let changed = state.page_dom.as_mut().is_some_and(|d| choose_option(d, select, option));
    match (multiple, state.ui.select.as_mut(), state.page_dom.as_ref()) {
        (true, Some(list), Some(dom)) => list.refresh(dom),
        _ => state.ui.select = None,
    }
    state.mark(Change::Page);
    if changed {
        super::toggle_field::heard_change(state, select);
    }
}
