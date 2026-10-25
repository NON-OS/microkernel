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

//! Settings as an app: its state, readied from the policy store on first
//! use, and the skeleton's calls answered (`app_ready`, `app_tick`).

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::app_ready::Settings;
use super::event::{on_accessory, on_event};
use super::manifest::manifest;
use super::paint::paint;
use super::ui::search_field;

impl App for Settings {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        self.ensure_ready();
        on_event(&mut self.state, event)
    }

    fn titlebar_accessory_w(&self) -> u32 {
        search_field::WIDTH
    }

    fn paint_accessory(&mut self, fb: &mut PaintBuffer) {
        search_field::paint(fb, &self.state);
    }

    fn on_accessory_event(&mut self, event: InputEvent) -> EventOutcome {
        on_accessory(&mut self.state, event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.ensure_ready();
        // The window resizes, so record what is actually being painted into.
        // Hit tests and row counts read this back.
        self.state.win_w = fb.width;
        self.state.win_h = fb.height;
        paint(&self.state, fb);
    }

    fn tick_interval_ms(&self) -> i64 {
        self.tick_interval()
    }

    fn busy(&self) -> bool {
        self.wifi_busy()
    }

    fn on_tick(&mut self) -> bool {
        self.tick()
    }
}
