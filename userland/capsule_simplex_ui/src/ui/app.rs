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

use alloc::boxed::Box;

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;
use super::view::View;
use crate::link::Transport;
use crate::model::state::Model;
use crate::serve::open;

/// How often the window asks the transport for frames.
const TICK_MS: i64 = 250;

pub struct SimplexUi {
    pub model: Model,
    pub link: Box<dyn Transport>,
    /// The last `corrId` sent.
    pub corr: u32,
}

impl SimplexUi {
    pub fn new() -> Self {
        SimplexUi { model: Model::new(), link: open(), corr: 0 }
    }
}

impl App for SimplexUi {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(self, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(fb, &View::new(&self.model, self.link.label()));
    }

    fn on_tick(&mut self) -> bool {
        self.link.tick();
        self.follow_start() | self.pump()
    }

    fn tick_interval_ms(&self) -> i64 {
        TICK_MS
    }
}
