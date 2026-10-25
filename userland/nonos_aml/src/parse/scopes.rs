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

//! What the parser has to ask the namespace.
//!
//! A method call is a NameString followed by its arguments, and nothing in
//! the bytes says how many there are (section 20.2.5.4, MethodInvocation):
//! only the method's own definition, or an External, does. So the parser
//! tells the namespace where it is and what is declared, and asks it how many
//! arguments a name takes. That is also why a method body is read only after
//! every table is loaded: by then each name it calls is known.

use crate::grammar::NamePath;

pub trait Scopes {
    /// The parser is about to read the TermList of the object `name` names,
    /// relative to the scope it is in now.
    fn enter(&mut self, bytes: &[u8], name: NamePath);

    /// That TermList has ended.
    fn leave(&mut self);

    /// A Method, or an External for one, says a call to `name` takes `args`.
    fn declare(&mut self, bytes: &[u8], name: NamePath, args: u8);

    /// How many arguments a call to `name` takes from the current scope; 0 for
    /// a name that is not a method or is not known yet.
    fn arity(&self, bytes: &[u8], name: NamePath) -> u8;
}
