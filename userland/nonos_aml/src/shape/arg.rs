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

//! The operand kinds of chapter 20 and the classes that say where an opcode
//! may stand.

/// One operand, in the order the grammar lists it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg {
    /// TermArg: an expression, a data object, a local or an argument.
    Term,
    /// SuperName or Target. NullName is a NameString, so Target needs no kind
    /// of its own.
    Super,
    /// SimpleName: a NameString, a local or an argument.
    Simple,
    /// A NameString that names, never calls.
    Name,
    /// DataRefObject, as Name's value.
    Data,
    U8,
    U16,
    U32,
    /// A TermList to the end of the package, opening the scope named by the
    /// object's first operand.
    Scope,
    /// A TermList to the end of the package, in the enclosing scope.
    Terms,
    /// FieldList to the end of the package.
    Fields,
    /// ByteList to the end of the package.
    Bytes,
    /// PackageElementList to the end of the package.
    Elements,
    /// A method's TermList, kept unparsed.
    Body,
}

/// Where an opcode may stand (section 20.2.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// NamedObj or NameSpaceModifierObj: a term list only.
    Object,
    /// Type1Opcode: a term list only.
    Statement,
    /// Type2Opcode: a term list or any TermArg.
    Expr,
    /// DataObject: also allowed as Name's value and as a package element.
    Data,
    /// Else, which only follows an If.
    Else,
}

/// Opcodes that share a class and an operand list.
#[derive(Clone, Copy, Debug)]
pub struct Group {
    pub class: Class,
    /// Whether a PkgLength follows the opcode and bounds the operands.
    pub pkg: bool,
    pub args: &'static [Arg],
    pub ops: &'static [u16],
}
