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

//! _UPC, USB Port Capabilities (ACPI 6.5, 9.14): a package of whether the
//! port is connectable, its connector type, and two reserved integers.
//! Type 0xFF is a proprietary connector: a port wired to something inside
//! the machine rather than to a socket.

use crate::object::Object;

pub const PROPRIETARY: u8 = 0xFF;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Upc {
    pub connectable: bool,
    /// 0x00 Type-A, 0x03 USB 3 Standard-A, 0x08 to 0x0A Type-C, 0xFF
    /// proprietary, and the rest as 9.14 lists them.
    pub connector: u8,
}

/// None when it is not a package of at least two integers, or the type does
/// not fit a byte.
pub fn upc(o: &Object) -> Option<Upc> {
    let Object::Package(p) = o else { return None };
    let p = p.borrow();
    let (Some(Object::Integer(c)), Some(Object::Integer(t))) = (p.first(), p.get(1)) else {
        return None;
    };
    Some(Upc { connectable: *c != 0, connector: u8::try_from(*t).ok()? })
}
