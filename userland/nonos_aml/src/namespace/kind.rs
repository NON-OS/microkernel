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

//! What kind of object an entry is. Values are not kept here: an object's
//! definition stays in its table's arena, and the interpreter evaluates it.

pub use super::field_unit::FieldUnit;
pub use super::method_info::MethodInfo;
use super::Id;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A scope with no object of its own: the root and \_GPE, \_PR, \_SI, \_TZ.
    Scope,
    Device,
    Method(MethodInfo),
    Name,
    Alias(Id),
    OpRegion,
    DataRegion,
    FieldUnit(FieldUnit),
    BufferField,
    Mutex,
    Event,
    PowerResource,
    ThermalZone,
    Processor,
}

impl Kind {
    /// Whether the object opens a scope its definition's TermList fills.
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            Kind::Scope | Kind::Device | Kind::PowerResource | Kind::ThermalZone | Kind::Processor
        )
    }
}
