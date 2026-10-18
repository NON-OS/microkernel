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

//! A hung transmitter put right without giving anything back: the part
//! quiesced and reset (the I219 flush runs on this driver's own rings), both
//! rings started over from slot 0, and the part put on the air again under
//! the station address it already has. As at bring-up, `reset_part` turns
//! Bus Master off before the reset and `start` turns it on again once both
//! rings name this driver's frames.

use crate::setup::Driver;

use super::{reset_part, start};

pub fn recover(d: &mut Driver) -> Result<(), &'static str> {
    reset_part::run(d)?;
    d.rx.head = 0;
    d.tx.tail = 0;
    d.tx.clean = 0;
    start::run(d)
}
