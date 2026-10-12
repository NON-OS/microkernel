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

//! Why an evaluation stopped.

use crate::error::ErrorKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultKind {
    /// More steps than the evaluation's budget: a loop that does not end.
    Steps,
    /// Calls nested deeper than the budget, or deeper into the stack than
    /// its share: recursion that does not end.
    CallDepth,
    /// More bytes made than the budget allows.
    Alloc,
    /// More time asked for by Sleep, Stall and waits than the budget allows,
    /// or a wait with no timeout that nothing could ever end.
    Time,
    /// A name that resolves to nothing.
    Undefined,
    /// An operand of a type the operator cannot take or convert.
    Type,
    /// An Index or Mid past the end of its object.
    Index,
    DivideByZero,
    /// A local or argument read before anything was stored to it.
    Uninit,
    /// The table executed Fatal (section 19.6.48): type and code are logged.
    Fatal,
    /// A mutex acquired below the current sync level, or released by an
    /// evaluation that does not hold it (section 19.6.2 and 19.6.118).
    Mutex,
    /// Break or Continue outside a While, or Return outside a method.
    Control,
    /// A reference to an object a method made and has since removed.
    Stale,
    /// A method body that does not parse.
    Parse(ErrorKind),
    /// A region access before its space's handler was connected.
    NoHandler,
    /// A region access the host refused or that failed.
    Region,
}

/// A fault and where in which table it happened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fault {
    pub kind: FaultKind,
    pub table: u16,
    pub at: u32,
}

impl Fault {
    pub const fn new(kind: FaultKind, table: u16, at: u32) -> Self {
        Self { kind, table, at }
    }
}
