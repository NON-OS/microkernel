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

//! Selecting revision 1 and UPT 1. Every vmxnet3 host offers them, and a
//! host that offers neither is not one whose layout this driver knows.

use crate::constants::regs::{REV_1, UPT_1};
use crate::regs::Regs;

pub fn select(regs: &Regs) -> Result<(), &'static str> {
    const _: () = assert!(REV_1 == UPT_1);
    if regs.select(REV_1) {
        Ok(())
    } else {
        Err("revision 1 not offered")
    }
}
