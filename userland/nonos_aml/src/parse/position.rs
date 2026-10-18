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

//! Where in the grammar a term stands, and what each place admits.

use crate::op::Op;
use crate::shape::Class;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pos {
    /// A TermObj in a TermList.
    Statement,
    /// A TermArg.
    Term,
    /// Name's value: a DataObject.
    Data,
    /// A PackageElement: a DataObject, or a NameString that refers.
    Element,
    /// A SuperName or Target.
    Super,
    /// A SimpleName.
    Simple,
}

impl Pos {
    /// Whether an opcode of `class` may stand here. A TermList also takes a
    /// bare data object: the grammar does not list one there, but its
    /// encoding is unambiguous and evaluating it changes nothing, so refusing
    /// a whole table for it would gain nothing.
    pub fn admits(self, class: Class, op: Op) -> bool {
        match (self, class) {
            (Pos::Statement, Class::Object | Class::Statement) => true,
            (Pos::Statement | Pos::Term, Class::Expr | Class::Data) => true,
            (Pos::Data | Pos::Element, Class::Data) => true,
            // ReferenceTypeOpcode, section 20.2.5.4.
            (Pos::Super, Class::Expr) => matches!(op, Op::REF_OF | Op::DEREF_OF | Op::INDEX),
            _ => false,
        }
    }

    /// Integers, strings and Revision.
    pub fn admits_constant(self) -> bool {
        matches!(self, Pos::Statement | Pos::Term | Pos::Data | Pos::Element)
    }

    /// LocalObj and ArgObj.
    pub fn admits_variable(self) -> bool {
        matches!(self, Pos::Statement | Pos::Term | Pos::Super | Pos::Simple)
    }

    /// Whether a NameString here calls a method rather than naming it.
    pub fn calls(self) -> bool {
        matches!(self, Pos::Statement | Pos::Term)
    }
}
