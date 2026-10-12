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

//! Reports, once notifications are on. A value on any other handle is not
//! a boot report this host asked for and is left alone.

use super::phase::Phase;
use super::state::Host;
use super::types::Kind;
use crate::report::Input;
use alloc::vec::Vec;

impl Host {
    pub(super) fn report(&mut self, handle: u16, value: &[u8]) -> Vec<Input> {
        if self.phase != Phase::Ready {
            return Vec::new();
        }
        match self.sources.iter().find(|s| s.value == handle).map(|s| s.kind) {
            Some(Kind::Keyboard) => self.keyboard.report(value),
            Some(Kind::Mouse) => self.mouse.report(value),
            None => Vec::new(),
        }
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
