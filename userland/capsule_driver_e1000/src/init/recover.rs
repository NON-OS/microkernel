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
//! stopped and reset, both rings started over from slot 0, and the part put
//! on the air again under the station address it already has. Bus Master
//! stays on: every frame the part can reach is still this driver's.

use crate::setup::Driver;

use super::{reset, start};

pub fn recover(d: &mut Driver) -> Result<(), &'static str> {
    reset::run(&d.regs)?;
    d.rx.head = 0;
    d.tx.tail = 0;
    d.tx.clean = 0;
    start::run(d)
}
