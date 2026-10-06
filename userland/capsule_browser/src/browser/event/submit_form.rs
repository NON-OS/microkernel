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

use alloc::string::ToString;

use crate::browser::omnibox::Change;
use crate::browser::state::State;
use crate::browser::url;

use super::enclosing_form::enclosing_form;
use super::form_fields::form_fields;
use super::relayout::relayout;

/* How long the notice that a form was not sent stays up. */
const NOTICE_MS: i64 = 8_000;

/* Whether `id` is a submit button, whose own name and value go with the
 * form when it is the one clicked. */
fn is_submit(dom: &crate::browser::dom::Dom, id: usize) -> bool {
    matches!(super::field_at::field_at(dom, id), super::field_at::Field::Submit(s) if s == id)
}

/* Submit the form enclosing `from`: run its submit listeners, gather the
 * fields, then navigate (POST body or GET query per the form's method). A
 * listener that calls preventDefault keeps the page where it is, which is
 * how script-driven forms submit through fetch instead. An empty action
 * submits to the document's own address, not to whatever is typed in the
 * address bar. `at` is where an image submit button was clicked. */
pub(super) fn submit_form(state: &mut State, from: usize, at: Option<(i32, i32)>) {
    let Some(form) = enclosing_form(state, from) else {
        return;
    };
    state.focus_page(None);
    if let Some(engine) = state.engine.as_ref() {
        let fired = engine.dispatch_event(form as i32, "submit") > 0;
        let prevented = fired && engine.default_prevented();
        if fired {
            relayout(state);
            state.track.laid_print = None;
            state.mark(Change::Page);
            /* A check that alerts "fill in your name" or brings the field
             * into view is said and followed; a navigation it asked for
             * gives way to the form's own, as in other browsers. */
            super::script_nav::take_script_nav(state);
        }
        if prevented {
            return;
        }
    }
    let Some(dom) = state.page_dom.as_ref() else {
        return;
    };
    let Some(node) = dom.nodes.get(form) else {
        return;
    };
    let submitter = Some(from).filter(|&f| f != form && is_submit(dom, f));
    let Ok(body) = form_fields(dom, form, submitter, at) else {
        let line = alloc::format!(
            "This form has more than {} fields, so it was not sent.",
            super::form_fields::MAX_FIELDS
        );
        state.ui.notice = Some((line, nonos_libc::mk_uptime_ms() + NOTICE_MS));
        state.mark(Change::Bubble);
        return;
    };
    let action = node.attr("action").unwrap_or("").to_string();
    let post = node.attr("method").is_some_and(|m| m.eq_ignore_ascii_case("post"));
    let target = match (state.base.as_ref(), action.is_empty()) {
        (Some(base), false) => url::join(base, &action),
        (Some(_), true) => state.ui.current_url.clone(),
        (None, _) => action,
    };
    super::form_send::form_send(state, target, body, post);
}
