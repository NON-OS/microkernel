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

//! What a battery's _BST got wrong, said once per battery so a fault that
//! firmware repeats at every reading is not a line at every reading: state
//! bits past those defined, ignored; and charging and discharging at once,
//! a firmware fault that leaves the direction unknown.

use super::state::Battery;
use crate::sys::log::Line;

const RESERVED: u8 = 1;
const BOTH: u8 = 2;

pub fn tell(b: &mut Battery) {
    let Some(s) = b.status else { return };
    if s.reserved != 0 && b.told & RESERVED == 0 {
        let line = Line::new(b"battery _BST: state bits past those defined, ignored: ");
        line.hex(u64::from(s.reserved)).end();
        b.told |= RESERVED;
    }
    if s.contradicts && b.told & BOTH == 0 {
        let fault = b"battery _BST: charging and discharging at once, a firmware fault; ";
        Line::new(fault).str(b"direction unknown, critical still heeded").end();
        b.told |= BOTH;
    }
}
