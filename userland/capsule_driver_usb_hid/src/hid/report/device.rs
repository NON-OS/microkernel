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

//! An interface read through its report descriptor. Each part of its
//! layout keeps its own state: its keyboard keys, its consumer and system
//! keys, and its pointer, so a media key report never releases a letter.

use crate::descriptors::report::Layout;

use super::super::{Keyboard, Mouse, Tablet};
use super::keys::{keys_report, media_report};
use super::pointer::{pointer_report, Motion};
use super::read::split;

pub struct ReportDevice {
    layout: Layout,
    keys: Keyboard,
    media: [Keyboard; 2],
    mouse: Mouse,
    tablet: Tablet,
}

impl ReportDevice {
    pub fn new(layout: Layout) -> Self {
        let media = [Keyboard::new(), Keyboard::new()];
        Self { layout, keys: Keyboard::new(), media, mouse: Mouse::new(), tablet: Tablet::new() }
    }

    /// Take one report: the parts of its report ID read it, the others are
    /// not touched by it.
    pub fn feed(&mut self, raw: &[u8]) {
        let Some((id, body)) = split(self.layout.ids, raw) else { return };
        if let Some(r) = self.layout.keys.filter(|k| k.id == id).and_then(|k| keys_report(&k, body))
        {
            self.keys.apply(r);
        }
        for (part, state) in self.layout.media.iter().zip(self.media.iter_mut()) {
            if let Some(r) = part.filter(|m| m.id == id).and_then(|m| media_report(&m, body)) {
                state.apply(r);
            }
        }
        match self.layout.pointer.filter(|p| p.id == id).and_then(|p| pointer_report(&p, body)) {
            Some(Motion::Relative(event)) => self.mouse.apply(event),
            Some(Motion::Absolute(report)) => self.tablet.apply(report),
            None => {}
        }
    }

    pub fn tick(&mut self, now_ms: u64) {
        self.keys.tick(now_ms);
        self.media.iter_mut().for_each(|m| m.tick(now_ms));
    }

    pub fn release_all(&mut self) {
        self.keys.release_all();
        self.media.iter_mut().for_each(Keyboard::release_all);
        self.mouse.release_all();
        self.tablet.release_all();
    }
}
