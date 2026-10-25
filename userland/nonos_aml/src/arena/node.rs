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

//! One parsed object.

use crate::grammar::NamePath;
use crate::op::Op;

/// A run of table bytes, or of entries in the arena's kid list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub at: u32,
    pub len: u32,
}

impl Span {
    pub fn end(&self) -> u32 {
        self.at + self.len
    }
}

/// What an object carries beside its operands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imm {
    None,
    /// A constant's value, a fixed-width operand, a field element's width in
    /// bits, or an access field's bytes packed low to high.
    Int(u64),
    /// A NameString, or a NamedField's single NameSeg.
    Name(NamePath),
    /// A string's characters without the NUL, a ByteList, or a method body.
    Bytes(Span),
}

/// An opcode, where it starts in the table, its immediate, and its operands
/// as a run of the arena's kid list, in grammar order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    pub op: Op,
    pub at: u32,
    pub imm: Imm,
    pub kids: Span,
}
