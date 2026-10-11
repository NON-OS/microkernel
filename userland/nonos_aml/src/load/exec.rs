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

//! What loading needs from the interpreter.
//!
//! A definition block's term list is not only definitions: an SSDT commonly
//! wraps them in `If (CondRefOf (...))`, and may Store or Notify while it
//! loads. Those run as the table loads, in the scope they stand in, so the
//! loader hands each one to the interpreter and loads the TermList an If
//! chooses.

use super::Aml;
use crate::error::Error;
use crate::namespace::Id;

pub trait LoadExec {
    /// Evaluates an If's predicate (the node's first operand) in `scope`.
    fn condition(&mut self, aml: &mut Aml, table: u16, scope: Id, node: u32)
        -> Result<bool, Error>;

    /// Runs any other statement or expression found while loading.
    fn run(&mut self, aml: &mut Aml, table: u16, scope: Id, node: u32) -> Result<(), Error>;
}
