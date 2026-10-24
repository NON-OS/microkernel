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

//! The tag of each UAS command, 1 and 2 by turns: an IU a device sends
//! late, for the command before, then never carries the tag of the one in
//! flight and is let go (nonos_uas). Two tags fit every device: without
//! streams any tag does, and with them there are at least two.

use super::types::State;

impl State {
    pub fn uas_tag(&mut self) -> u16 {
        2 - (self.next_tag() & 1) as u16
    }
}
