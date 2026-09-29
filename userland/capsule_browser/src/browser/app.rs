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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::state::State;

pub struct Browser {
    state: State,
}

impl Browser {
    pub fn new() -> Self {
        Browser { state: State::new() }
    }
}

impl App for Browser {
    fn manifest(&self) -> AppManifest {
        manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.state, event)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        /* The window can be resized after the page laid out; when the surface
         * width or page height changes, reflow the document to the new size so
         * it fills the window and vh units and fixed boxes track its height. */
        let page_h = fb.height.saturating_sub(crate::browser::state::CHROME_H);
        let size = (fb.width, page_h);
        if fb.width != 0 && size != (self.state.viewport_w, self.state.viewport_h) {
            (self.state.viewport_w, self.state.viewport_h) = size;
            crate::browser::event::relayout(&mut self.state);
        }
        paint(&self.state, fb);
    }
    fn on_tick(&mut self) -> bool {
        self.tick_body()
    }

    fn tick_interval_ms(&self) -> i64 {
        50
    }

    /* A request on the wire, a queued navigation, or stylesheets, images or
     * fonts still to fetch all mean the load must keep stepping. Reporting busy
     * makes the runner yield rather than sleep, so the page advances through its
     * fetch stages on its own instead of stalling until the pointer moves. */
    fn busy(&self) -> bool {
        let s = &self.state;
        s.fetch.is_some()
            || s.pending_nav.is_some()
            || !s.css_queue.is_empty()
            || !s.script_queue.is_empty()
            || !s.image_queue.is_empty()
            || !s.font_queue.is_empty()
            || s.pool.busy()
    }
}

impl Browser {
    fn tick_body(&mut self) -> bool {
        /* A pending navigation preempts any in-flight fetch. Page loads pull
         * in stylesheets and images that keep the socket busy well after the
         * document appears; without this the user could never navigate away
         * from the address bar or a link while those sub-fetches ran. */
        let mut shown = false;
        if self.state.pending_nav.is_some() {
            if let Some(job) = self.state.fetch.take() {
                let _ = crate::browser::net::socket_close(self.state.sockets_port, job.handle);
            }
        }
        if let Some(target) = self.state.pending_nav.take() {
            if let Err(msg) = crate::browser::fetch::load(&mut self.state, &target) {
                self.state.status = alloc::string::String::from(msg);
                self.state.document = Some(crate::browser::fetch::render_error(msg));
                self.state.box_doc = None;
                self.state.engine = None;
                self.state.page_dom = None;
                self.state.world = None;
                self.state.view = crate::browser::state::View::Page;
            }
            shown = true;
        }
        /* Every fetch steps, lands and starts here; the page's timers after. */
        let fetched = crate::browser::fetch::tick(&mut self.state);
        let timers = crate::browser::event::js_tick(&mut self.state);
        shown || fetched || timers
    }
}
