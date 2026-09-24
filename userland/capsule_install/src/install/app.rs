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

//! The app the runner drives: events to the router, ticks to the job,
//! frames to the painter.

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::event::on_event;
use super::job::tick;
use super::manifest::manifest;
use super::state::{Screen, State};
use super::ui::paint;

pub struct Install {
    state: State,
}

impl Install {
    pub fn new() -> Self {
        Install { state: State::new() }
    }
}

impl App for Install {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(&mut self.state, fb);
    }

    fn on_tick(&mut self) -> bool {
        tick(&mut self.state)
    }

    /// While a disk is being written or read back the loop runs as fast as
    /// the runner allows and paints every step; otherwise once a second.
    fn tick_interval_ms(&self) -> i64 {
        if self.busy() {
            0
        } else {
            1000
        }
    }

    fn busy(&self) -> bool {
        matches!(self.state.screen, Screen::Writing | Screen::Verifying)
    }
}
