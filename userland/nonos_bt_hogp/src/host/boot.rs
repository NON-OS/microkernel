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
//! The boot reports' state, one for each: two boot keyboards, or two boot
//! mice, in separate HID services each keep their own keys and buttons,
//! so a report from one never releases what the other holds.

use super::state::Host;
use super::types::Kind;
use crate::report::{Input, Keyboard, Mouse};
use alloc::vec::Vec;

#[derive(Default)]
pub(super) struct Boot {
    pub value: u16,
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl Host {
    /// A state for each boot report kept, made as discovery ends.
    pub(super) fn boot_states(&mut self) {
        let boot = self.sources.iter().filter(|s| s.kind != Kind::Report);
        self.boots = boot.map(|s| Boot { value: s.value, ..Boot::default() }).collect();
    }

    pub(super) fn boot_in(&mut self, kind: Kind, handle: u16, value: &[u8]) -> Vec<Input> {
        let Some(b) = self.boots.iter_mut().find(|b| b.value == handle) else { return Vec::new() };
        match kind {
            Kind::Keyboard => b.keyboard.report(value),
            Kind::Mouse => b.mouse.report(value),
            Kind::Report => Vec::new(),
        }
    }
}
