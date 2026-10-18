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

//! SVGA_REG_ID: write the version wanted, read back the version agreed.
//! Every SVGA II host takes 2; one that answers otherwise is not one.

use crate::constants::regs::{ID, SVGA_ID_2};
use crate::regs::Regs;

pub fn negotiate(regs: &Regs) -> Result<(), &'static str> {
    regs.write(ID, SVGA_ID_2);
    if regs.read(ID) != SVGA_ID_2 {
        return Err("SVGA version 2 not offered");
    }
    Ok(())
}
