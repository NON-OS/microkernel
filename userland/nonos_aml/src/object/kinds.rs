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

//! ObjectType's numbering (section 19.6.96).

use super::Object;
use crate::namespace::Kind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectType {
    Uninit = 0,
    Integer = 1,
    String = 2,
    Buffer = 3,
    Package = 4,
    FieldUnit = 5,
    Device = 6,
    Event = 7,
    Method = 8,
    Mutex = 9,
    OpRegion = 10,
    PowerResource = 11,
    Processor = 12,
    ThermalZone = 13,
    BufferField = 14,
    DdbHandle = 15,
    Debug = 16,
}

/// The type of a named object that holds no data of its own. A Name's type
/// is its value's, so the caller asks the value instead.
pub fn type_of(kind: Kind) -> ObjectType {
    match kind {
        Kind::Scope | Kind::Device => ObjectType::Device,
        Kind::Method(_) => ObjectType::Method,
        Kind::Mutex => ObjectType::Mutex,
        Kind::Event => ObjectType::Event,
        Kind::OpRegion | Kind::DataRegion => ObjectType::OpRegion,
        Kind::FieldUnit(_) => ObjectType::FieldUnit,
        Kind::BufferField => ObjectType::BufferField,
        Kind::PowerResource => ObjectType::PowerResource,
        Kind::ThermalZone => ObjectType::ThermalZone,
        Kind::Processor => ObjectType::Processor,
        Kind::Name | Kind::Alias(_) => ObjectType::Uninit,
    }
}

impl Object {
    /// The type of a value; a reference has no number of its own, so the
    /// caller asks the object it refers to (section 19.6.96).
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Uninit | Object::Ref(_) | Object::Named(..) => ObjectType::Uninit,
            Object::Table(_) => ObjectType::DdbHandle,
            Object::Integer(_) => ObjectType::Integer,
            Object::String(_) => ObjectType::String,
            Object::Buffer(_) => ObjectType::Buffer,
            Object::Package(_) => ObjectType::Package,
        }
    }
}
