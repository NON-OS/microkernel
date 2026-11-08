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
use nonos_toolkit::widgets::Controls;

use super::action::Action;
use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;

pub struct GuiDemo {
    pub clicks: u32,
    pub on: bool,
    pub remember: bool,
    pub last: Option<Action>,
    pub controls: Controls<Action>,
}

impl GuiDemo {
    pub fn new() -> Self {
        GuiDemo { clicks: 0, on: false, remember: false, last: None, controls: Controls::new() }
    }

    pub fn act(&mut self, a: Action) {
        match a {
            Action::Count => self.clicks = self.clicks.saturating_add(1),
            Action::Reset => self.clicks = 0,
            Action::Toggle => self.on = !self.on,
            Action::Remember => self.remember = !self.remember,
        }
        self.last = Some(a);
    }
}

impl App for GuiDemo {
    fn manifest(&self) -> AppManifest {
        manifest()
    }
    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(event, self)
    }
    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(fb, self);
    }
}
