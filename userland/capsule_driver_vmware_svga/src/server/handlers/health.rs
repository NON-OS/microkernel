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

//! OP_HEALTHCHECK: still in SVGA mode with the FIFO configured. A host that
//! dropped either (a VGA mode set by someone else) is reported, not drawn
//! into blind.

use crate::constants::regs::{CONFIG_DONE, ENABLE};
use crate::protocol::E_DEVICE;
use crate::setup::Driver;

pub fn health(d: &Driver) -> i32 {
    if d.regs.read(ENABLE) != 0 && d.regs.read(CONFIG_DONE) != 0 {
        0
    } else {
        E_DEVICE
    }
}
