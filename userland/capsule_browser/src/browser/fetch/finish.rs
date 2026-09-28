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

use crate::browser::fetch::{record_history, redirect, render_error, render_response};
use crate::browser::http;
use crate::browser::state::{State, View};

pub(super) fn finish(state: &mut State, raw: &[u8], suppress: bool) {
    state.retries = 0;
    match http::response::parse(raw) {
        Some(resp) => {
            if matches!(resp.status, 301 | 302 | 303 | 307 | 308) {
                if let Some(loc) = resp.location {
                    return redirect::redirect(state, loc);
                }
            }
            state.redirect_count = 0;
            let (rendered, count) = render_response::render_response(&resp);
            state.scroll = 0;
            state.status = alloc::format!(
                "{} raw={} body={} fl={}",
                resp.status,
                raw.len(),
                resp.body.len(),
                count
            );
            // A fresh document drops any stylesheets gathered for the last one.
            state.css_queue.clear();
            state.script_queue.clear();
            state.page_css.clear();
            match rendered {
                render_response::Rendered::Html(dom) => {
                    // Drop the prior page's engine before its DOM disappears,
                    // then home the new DOM and run its scripts against that
                    // stable address.
                    state.engine = None;
                    state.page_dom = Some(dom);
                    state.document = None;
                    super::commit_html::commit_html(state);
                    super::enqueue_css::enqueue_css(state);
                    super::enqueue_scripts::enqueue_scripts(state);
                    // Stylesheets are render-blocking: if the page pulls in
                    // external CSS, hold the first paint until it arrives so
                    // the user never sees the unstyled document flash. With no
                    // external CSS the inline render is already complete.
                    if !state.css_queue.is_empty() {
                        state.box_doc = None;
                        state.status = alloc::string::String::from("loading styles");
                    }
                }
                render_response::Rendered::Lines(d) => {
                    state.document = Some(d);
                    state.box_doc = None;
                    state.page_dom = None;
                    state.world = None;
                    state.engine = None;
                }
                render_response::Rendered::Nothing => {
                    state.document = None;
                    state.box_doc = None;
                    state.page_dom = None;
                    state.world = None;
                    state.engine = None;
                }
            }
            crate::browser::image::enqueue_from_doc(state);
            record_history::record_history(state, suppress);
        }
        None => {
            state.redirect_count = 0;
            state.status = alloc::format!("bad resp raw={}", raw.len());
            state.document = Some(render_error::render_error(&incomplete(raw)));
            state.box_doc = None;
            state.page_dom = None;
            state.world = None;
            state.engine = None;
        }
    }
    state.view = View::Page;
}

/// Why a response could not be read, with the numbers that say so.
///
/// "bad response" alone told a reader nothing, and it covered two different
/// things: a body that stopped short of what its headers promised, and bytes
/// that were never an HTTP response at all.
fn incomplete(raw: &[u8]) -> alloc::string::String {
    let Some(sep) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
        return alloc::format!("bad response: {} bytes and no complete header", raw.len());
    };
    let body = raw.len() - sep - 4;
    let head = core::str::from_utf8(&raw[..sep]).unwrap_or("");
    let declared = head.lines().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        k.trim().eq_ignore_ascii_case("content-length").then(|| v.trim())
    });
    match declared {
        Some(n) => {
            alloc::format!("bad response: body {} of {} bytes the headers declared", body, n)
        }
        None => alloc::format!("bad response: {} header bytes, {} body bytes", sep, body),
    }
}
