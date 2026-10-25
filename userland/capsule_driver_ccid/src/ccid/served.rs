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

//! Which slots a command may go to: those the reader has and the driver
//! remembers (slots.rs). A slot not remembered could be powered and never
//! powered off at CLOSE, leaving a card live after its holder.

use super::slots::SLOTS;

/// Whether `slot` of a reader with slots 0 to `max_slot` is served.
pub fn served(slot: u8, max_slot: u8) -> bool {
    slot <= max_slot && slot < SLOTS
}
