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

//! The Generic Register descriptor (ACPI 6.5, 6.4.3.7) that _PCT, _CST and
//! _CPC hand over inside a resource template: where a control lives, in
//! which address space, and how wide it is.

use crate::object::Object;

const GENERIC_REGISTER: u8 = 0x82;
const BODY: usize = 12;

/// Address spaces (5.2.3.2) a processor register names.
pub const SYSTEM_MEMORY: u8 = 0x00;
pub const SYSTEM_IO: u8 = 0x01;
pub const PCC: u8 = 0x0A;
pub const FIXED_HARDWARE: u8 = 0x7F;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Register {
    pub space: u8,
    pub width: u8,
    pub offset: u8,
    pub access: u8,
    pub address: u64,
}

impl Register {
    /// _CPC's way of saying the platform gives no such register (8.4.6.1):
    /// a descriptor whose every field is zero.
    pub fn is_null(&self) -> bool {
        *self == Register::default()
    }
}

/// The descriptor at the start of a template; None for anything else, or
/// one shorter than its fields, or claiming more than its buffer holds.
pub fn register_in(t: &[u8]) -> Option<Register> {
    let (&tag, l) = (t.first()?, t.get(1..3)?);
    let len = u16::from_le_bytes([l[0], l[1]]) as usize;
    let b = t.get(3..3 + len)?;
    if tag != GENERIC_REGISTER || len < BODY {
        return None;
    }
    let mut a = [0u8; 8];
    a.copy_from_slice(&b[4..12]);
    Some(Register {
        space: b[0],
        width: b[1],
        offset: b[2],
        access: b[3],
        address: u64::from_le_bytes(a),
    })
}

/// The register a package element holds, if it is a buffer holding one.
pub fn register(o: &Object) -> Option<Register> {
    match o {
        Object::Buffer(b) => register_in(&b.borrow()),
        _ => None,
    }
}
