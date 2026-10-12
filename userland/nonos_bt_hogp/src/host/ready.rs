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

//! Reports, once notifications are on, and Battery Levels. A value on any
//! other handle is not one this host asked for and is left alone. A lock key that
//! goes down also changes the keyboard's LEDs.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use super::types::Kind;
use crate::report::Input;
use alloc::vec::Vec;

impl Host {
    pub(super) fn report(&mut self, handle: u16, value: &[u8]) -> Step {
        if self.phase != Phase::Ready {
            return Step::default();
        }
        if let Some(cell) = self.cells.iter().position(|c| c.value == handle) {
            return Step { battery: self.level(cell, value), ..Step::default() };
        }
        let input = match self.sources.iter().find(|s| s.value == handle).map(|s| s.kind) {
            Some(Kind::Keyboard) => self.keyboard.report(value),
            Some(Kind::Mouse) => self.mouse.report(value),
            None => Vec::new(),
        };
        let mut step = Step::input(input);
        if self.locks.on(&step.input) {
            step.send = self.leds();
        }
        step
    }

    /// The link has gone: release every key and button still held, so
    /// nothing stays pressed in the session.
    pub fn disconnected(&mut self) -> Vec<Input> {
        self.phase = Phase::Failed;
        self.release_all()
    }

    pub(super) fn release_all(&mut self) -> Vec<Input> {
        let mut out = self.keyboard.release_all();
        out.extend(self.mouse.release_all());
        out
    }
}
