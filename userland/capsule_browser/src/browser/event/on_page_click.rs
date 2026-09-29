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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::browser::omnibox::geometry::CONTENT_TOP;
use crate::browser::omnibox::{focus_after, Region};
use crate::browser::state::State;

use super::field_at::{field_at, Field};

/* A click on the page. Script listeners see it first; only a listener that
 * calls preventDefault stops the default action, so a page that merely
 * listens for clicks (most of the web does) keeps working links and fields.
 * Then a form field takes focus, a submit control submits, and a link is
 * followed. The link is read before the listeners run, as the default
 * action belongs to the element that was clicked. */
pub fn on_page_click(state: &mut State, event: InputEvent) -> EventOutcome {
    let y = event.y - CONTENT_TOP as i32;
    let link = super::link_under::link_under(state, event.x, y);
    let scroll = state.scroll as i32;
    let node = state.box_doc.as_ref().and_then(|b| b.hit_node(event.x, y, scroll));
    let mut field = None;
    let mut prevented = false;
    if let Some(node) = node {
        let click = super::js_click::js_click(state, node);
        prevented = click.prevented;
        if click.fired && state.pending_nav.is_some() {
            state.focus_page(None);
            return EventOutcome::Repaint;
        }
        let hit = state.page_dom.as_ref().map(|dom| field_at(dom, node));
        match hit {
            Some(Field::Edit(id)) if !prevented => field = Some(id),
            Some(Field::Submit(id)) if !prevented => {
                state.focus_page(None);
                super::submit_form::submit_form(state, id);
                return EventOutcome::Repaint;
            }
            _ => {}
        }
    }
    let (_, field) = focus_after(Region::Page, field, state.ui.kbd);
    state.focus_page(field);
    match link {
        Some(href) if !prevented && field.is_none() => {
            super::follow_link::follow_link(state, &href)
        }
        _ => EventOutcome::Idle,
    }
}
