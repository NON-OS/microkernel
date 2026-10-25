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

//! Where a field unit sits.

/// A field unit's place in its region or buffer, laid out from its FieldList
/// (section 19.6.48): each NamedField starts where the last element ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldUnit {
    /// The Field, IndexField or BankField node that declares it.
    pub list: u32,
    pub bit_offset: u32,
    pub bit_len: u32,
    /// The access in force: the FieldFlags' AccessType, or the last
    /// AccessField's or ExtendedAccessField's bytes (type, attrib, length).
    pub access: [u8; 3],
    /// The last ConnectField before it, if any.
    pub connection: Option<u32>,
}
