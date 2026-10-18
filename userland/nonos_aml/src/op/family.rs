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

//! Which part of the interpreter evaluates an expression opcode.

use super::Op;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// Operand, Operand, Target: the integer operators.
    Binary,
    Divide,
    /// Operand, Target: Not, FindSetLeftBit, FindSetRightBit, ToBCD, FromBCD.
    Unary,
    /// Increment and Decrement.
    Step,
    /// LAnd, LOr, LNot, LEqual, LGreater, LLess.
    Logic,
    /// Concatenate, ConcatenateResTemplate, Mid and the To conversions.
    Text,
    /// Store and CopyObject.
    Store,
    /// RefOf, DerefOf, CondRefOf, Index, SizeOf, ObjectType.
    Refs,
    Match,
    /// Acquire and Wait.
    Sync,
    Timer,
    /// Load and LoadTable.
    Load,
    /// Not an expression.
    None,
}

impl Op {
    pub const fn family(self) -> Family {
        match self.0 {
            0x72 | 0x74 | 0x77 | 0x79 | 0x7A | 0x7B | 0x7C | 0x7D | 0x7E | 0x7F | 0x85 => {
                Family::Binary
            }
            0x78 => Family::Divide,
            0x80 | 0x81 | 0x82 | 0x5B28 | 0x5B29 => Family::Unary,
            0x75 | 0x76 => Family::Step,
            0x90..=0x95 => Family::Logic,
            0x73 | 0x84 | 0x9E | 0x9C | 0x96 | 0x97 | 0x98 | 0x99 => Family::Text,
            0x70 | 0x9D => Family::Store,
            0x71 | 0x83 | 0x5B12 | 0x88 | 0x87 | 0x8E => Family::Refs,
            0x89 => Family::Match,
            0x5B23 | 0x5B25 => Family::Sync,
            0x5B33 => Family::Timer,
            0x5B20 | 0x5B1F => Family::Load,
            _ => Family::None,
        }
    }

    /// The integer constants, whose value the arena node carries.
    pub const fn is_integer_constant(self) -> bool {
        matches!(self.0, 0x00 | 0x01 | 0x0A | 0x0B | 0x0C | 0x0E | 0xFF)
    }
}
