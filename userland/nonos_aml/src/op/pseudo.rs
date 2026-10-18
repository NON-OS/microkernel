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

//! The arena's own node kinds. Chapter 20 has grammar productions with no
//! opcode byte (a term list, a NameString, a field element); the parser still
//! needs a node for each, so they take values in a page section 20.3 never
//! uses.

use super::Op;

impl Op {
    /// A TermList: the body of a scope, an If, an Else or a While.
    pub const TERMS: Op = Op(0xF001);
    /// A NameString. As a term its kids are a method call's arguments.
    pub const NAME_STRING: Op = Op(0xF002);
    /// A fixed-width operand (ByteData, WordData, DWordData) inside an object.
    pub const IMM: Op = Op(0xF003);
    /// A Buffer's ByteList.
    pub const BYTES: Op = Op(0xF004);
    /// A method body, kept as bytes until the namespace can say how many
    /// arguments each name it calls takes.
    pub const DEFERRED: Op = Op(0xF005);
    pub const NAMED_FIELD: Op = Op(0xF006);
    pub const RESERVED_FIELD: Op = Op(0xF007);
    pub const ACCESS_FIELD: Op = Op(0xF008);
    pub const EXTENDED_ACCESS_FIELD: Op = Op(0xF009);
    pub const CONNECT_FIELD: Op = Op(0xF00A);
    /// A FieldList: its kids are the field elements.
    pub const FIELD_LIST: Op = Op(0xF00B);
    /// A PackageElementList: its kids are the initializers.
    pub const ELEMENTS: Op = Op(0xF00C);
}
