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

//! The polls the slot proofs feed a slot: a card up, its link down, no
//! card, and the button pressed with the card up.

use crate::native::regs::{BUTTON_PRESSED, PRESENT};
use crate::native::slot::Read;

pub const fn rd(status: u16, link: Option<bool>, powered: bool) -> Read {
    Read { status, link, powered, latch_open: false }
}

pub const UP: Read = rd(PRESENT, Some(true), true);
pub const DOWN: Read = rd(PRESENT, Some(false), true);
pub const OUT: Read = rd(0, Some(false), true);
pub const PRESS: Read = rd(PRESENT | BUTTON_PRESSED, Some(true), true);
