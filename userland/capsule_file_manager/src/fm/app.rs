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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, InputKind, PaintBuffer};

use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::persist_meta::persist_meta;
use super::refresh::refresh;
use super::state::State;
use super::target_at::rehover;

pub struct FileManager {
    state: State,
}

impl FileManager {
    pub fn new() -> Self {
        let mut state = State::new();
        // refresh reads the tags, favourites and preferences first, and stops
        // at the first call the store does not answer.
        refresh(&mut state);
        FileManager { state }
    }
}

impl App for FileManager {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    /// The pointer is fed before the event acts and named again after, since a
    /// click or the wheel can change what lies under it. A change of hover or
    /// press repaints an event that would otherwise not.
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        let lit = self.state.hover.feed(&event);
        let lit = rehover(&mut self.state) || lit;
        // A listing that failed is tried again on the user's next key or click,
        // one call per press: never from paint, and never once per pointer move,
        // since a store that has stopped answering costs a reply timeout a try.
        let press = matches!(event.kind, InputKind::KeyDown | InputKind::ButtonDown);
        if self.state.owner_pid == 0 || (press && self.state.load_error.is_some()) {
            refresh(&mut self.state);
        }
        let outcome = on_event(&mut self.state, event);
        let moved = rehover(&mut self.state);
        if outcome == EventOutcome::Close {
            persist_meta(&self.state);
        }
        if outcome == EventOutcome::Idle && (lit || moved) {
            return EventOutcome::Repaint;
        }
        outcome
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        if self.state.owner_pid == 0 {
            refresh(&mut self.state);
        }
        self.state.win_w = fb.width;
        self.state.win_h = fb.height;
        super::layout_measure::measure(&mut self.state, fb.height);
        super::info_cache::sync_info(&mut self.state);
        super::home_count::sync_places(&mut self.state);
        paint(&self.state, fb);
    }

    fn on_tick(&mut self) -> bool {
        super::open_arg::poll_open_arg(&mut self.state)
    }
}
