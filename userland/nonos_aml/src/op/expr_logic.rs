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

//! The logical operators of section 20.2.5.4. LNotEqual, LLessEqual and
//! LGreaterEqual have no opcode of their own: they are LNot wrapped around
//! LEqual, LGreater and LLess, and parse as exactly that.

use super::Op;

impl Op {
    pub const LAND: Op = Op(0x90);
    pub const LOR: Op = Op(0x91);
    pub const LNOT: Op = Op(0x92);
    pub const LEQUAL: Op = Op(0x93);
    pub const LGREATER: Op = Op(0x94);
    pub const LLESS: Op = Op(0x95);
}
