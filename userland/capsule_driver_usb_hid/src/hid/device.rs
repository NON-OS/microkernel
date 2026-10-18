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

//! What one bound interface keeps between its reports, its own and no other
//! interface's: a report from one keyboard used to release every key held
//! on another, and drop its Shift, since all of them shared one state. An
//! interface that goes away lets go of everything it held.

use alloc::boxed::Box;

use super::{Keyboard, Mouse, Pad, ReportDevice};

pub enum Device {
    Keyboard(Keyboard),
    Mouse(Mouse),
    Report(Box<ReportDevice>),
    Pad(Box<Pad>),
}

impl Device {
    pub fn feed(&mut self, raw: &[u8]) {
        match self {
            Device::Keyboard(k) => k.feed(raw),
            Device::Mouse(m) => m.feed(raw),
            Device::Report(r) => r.feed(raw),
            Device::Pad(p) => p.feed(raw),
        }
    }

    /// A held key repeats from here: a USB keyboard reports only changes.
    pub fn tick(&mut self, now_ms: u64) {
        match self {
            Device::Keyboard(k) => k.tick(now_ms),
            Device::Mouse(_) | Device::Pad(_) => {}
            Device::Report(r) => r.tick(now_ms),
        }
    }

    /// The interface is gone: every key and button it held comes up, and
    /// its key repeat ends.
    pub fn release_all(&mut self) {
        match self {
            Device::Keyboard(k) => k.release_all(),
            Device::Mouse(m) => m.release_all(),
            Device::Report(r) => r.release_all(),
            Device::Pad(p) => p.release_all(),
        }
    }
}
