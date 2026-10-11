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

//! Expressions (Type 2 opcodes) that compute or convert, section 20.2.5.4.

use super::Op;

impl Op {
    pub const STORE: Op = Op(0x70);
    pub const REF_OF: Op = Op(0x71);
    pub const ADD: Op = Op(0x72);
    pub const CONCAT: Op = Op(0x73);
    pub const SUBTRACT: Op = Op(0x74);
    pub const INCREMENT: Op = Op(0x75);
    pub const DECREMENT: Op = Op(0x76);
    pub const MULTIPLY: Op = Op(0x77);
    pub const DIVIDE: Op = Op(0x78);
    pub const SHIFT_LEFT: Op = Op(0x79);
    pub const SHIFT_RIGHT: Op = Op(0x7A);
    pub const AND: Op = Op(0x7B);
    pub const NAND: Op = Op(0x7C);
    pub const OR: Op = Op(0x7D);
    pub const NOR: Op = Op(0x7E);
    pub const XOR: Op = Op(0x7F);
    pub const NOT: Op = Op(0x80);
    pub const FIND_SET_LEFT_BIT: Op = Op(0x81);
    pub const FIND_SET_RIGHT_BIT: Op = Op(0x82);
    pub const DEREF_OF: Op = Op(0x83);
    pub const CONCAT_RES: Op = Op(0x84);
    pub const MOD: Op = Op(0x85);
    pub const SIZE_OF: Op = Op(0x87);
    pub const INDEX: Op = Op(0x88);
    pub const MATCH: Op = Op(0x89);
    pub const OBJECT_TYPE: Op = Op(0x8E);
    pub const TO_BUFFER: Op = Op(0x96);
    pub const TO_DECIMAL_STRING: Op = Op(0x97);
    pub const TO_HEX_STRING: Op = Op(0x98);
    pub const TO_INTEGER: Op = Op(0x99);
    pub const TO_STRING: Op = Op(0x9C);
    pub const COPY_OBJECT: Op = Op(0x9D);
    pub const MID: Op = Op(0x9E);
    pub const COND_REF_OF: Op = Op::ext(0x12);
    pub const LOAD_TABLE: Op = Op::ext(0x1F);
    pub const LOAD: Op = Op::ext(0x20);
    pub const ACQUIRE: Op = Op::ext(0x23);
    pub const WAIT: Op = Op::ext(0x25);
    pub const FROM_BCD: Op = Op::ext(0x28);
    pub const TO_BCD: Op = Op::ext(0x29);
    pub const TIMER: Op = Op::ext(0x33);
}
