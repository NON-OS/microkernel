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

//! A device's identity as _HID, _CID, _UID and _ADR give it (ACPI 6.5,
//! 6.1).

use super::eisa::eisa_id;
use crate::object::Object;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HardwareId {
    /// A compressed EISA id, decoded.
    Eisa([u8; 7]),
    /// A string id ("ACPI0003", "INT33D5").
    Text(Vec<u8>),
}

impl HardwareId {
    pub fn bytes(&self) -> &[u8] {
        match self {
            HardwareId::Eisa(b) => b,
            HardwareId::Text(t) => t,
        }
    }
}

/// _HID and each _CID entry are an Integer EISA id or a String.
pub fn hardware_id(o: &Object) -> Option<HardwareId> {
    match o {
        Object::Integer(v) => Some(HardwareId::Eisa(eisa_id(*v as u32))),
        Object::String(s) => Some(HardwareId::Text(s.borrow().clone())),
        _ => None,
    }
}

/// _CID is one id or a package of them (6.1.2).
pub fn compatible_ids(o: &Object) -> Vec<HardwareId> {
    match o {
        Object::Package(p) => p.borrow().iter().filter_map(hardware_id).collect(),
        o => hardware_id(o).into_iter().collect(),
    }
}
