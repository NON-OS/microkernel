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

// NONOS Operating System (AGPL-3.0-or-later)
//! F-104: the opcode table against the grammar and against dispatch. Every
//! opcode the grammar has a row for has exactly one table row and nothing
//! else does but the data, locals, arguments and Debug; an expression row
//! is run exactly when dispatch has a family for it; only Noop, BreakPoint
//! and External pass over; and every row has its reason.

use crate::exec::{opcode_row, opcode_rows, Fate};
use crate::op::{Family, Op};
use crate::shape::{lookup, Class};

/// Every opcode value: the primary page and the extended one.
fn all() -> impl Iterator<Item = u16> {
    (0..=0xFFu16).chain(0x5B00..=0x5BFF)
}

/// Accepted with no grammar row: their value is the opcode itself.
fn plain(op: u16) -> bool {
    let o = Op(op);
    o.is_integer_constant()
        || o.local().is_some()
        || o.arg().is_some()
        || matches!(op, 0x0D | 0x5B30 | 0x5B31)
}

#[test]
fn every_grammar_opcode_has_one_row_and_nothing_else_does() {
    for op in all() {
        let rows = opcode_rows().filter(|r| (r.first..=r.last).contains(&op)).count();
        let known = lookup(Op(op)).is_some() || plain(op);
        assert_eq!(rows, usize::from(known), "{op:#06x}");
    }
    assert!(opcode_rows().all(|r| !r.why.is_empty() && !r.name.is_empty() && r.first <= r.last));
}

#[test]
fn the_table_agrees_with_dispatch() {
    for op in all().filter(|op| opcode_row(*op).is_some()) {
        let (row, class) = (opcode_row(op).unwrap(), lookup(Op(op)).map(|g| g.class));
        if class == Some(Class::Expr) {
            let dispatched = Op(op).family() != Family::None;
            assert_eq!(row.fate == Fate::Run, dispatched, "{op:#06x} {}", row.name);
        }
        let passed_over = matches!(op, 0xA3 | 0xCC | 0x15);
        assert_eq!(row.fate == Fate::NoOp, passed_over, "{op:#06x} {}", row.name);
    }
}

#[path = "opcode_run_tests.rs"]
mod run;
