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

//! A keyboard that went away, unplugged or behind a hub that was: every key
//! it held comes up, with the code its press went down with, as a report
//! with nothing down would make it, and each release ends that key's repeat.
//! Left alone, the last key held repeated into the focused window for half
//! a minute and was never released, in the input router or in any client
//! holding it.

use super::boot_report::BootReport;
use super::types::Keyboard;

impl Keyboard {
    pub fn release_all(&mut self) {
        self.apply(BootReport { modifiers: 0, keys: [0; 6] });
    }
}
