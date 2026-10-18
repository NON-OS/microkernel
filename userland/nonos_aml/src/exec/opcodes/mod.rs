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

//! Every opcode the interpreter accepts, as one table: whether it is run,
//! passed over, or refused, and why, with the bound that holds it where it
//! has one. The table is in parts only to keep each file short. A byte in
//! neither the table nor a name is refused by the parser; `nonos_aml_proofs`
//! holds the table to the grammar's rows and to dispatch, so a new opcode
//! cannot be added without a row.

mod data;
mod expr;
mod logic;
mod object;
mod statement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fate {
    Run,
    /// Accepted and passed over: nothing happens.
    NoOp,
    /// Accepted by the grammar, refused when evaluated.
    Refused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpRow {
    /// The opcodes the row covers, `first` to `last`: one, or a run of
    /// locals or arguments. Extended opcodes are 0x5Bxx.
    pub first: u16,
    pub last: u16,
    pub name: &'static str,
    pub fate: Fate,
    pub why: &'static str,
}

pub(super) const fn run(op: u16, name: &'static str, why: &'static str) -> OpRow {
    OpRow { first: op, last: op, name, fate: Fate::Run, why }
}

pub(super) const fn noop(op: u16, name: &'static str, why: &'static str) -> OpRow {
    OpRow { first: op, last: op, name, fate: Fate::NoOp, why }
}

pub const TABLE: [&[OpRow]; 5] =
    [data::ROWS, object::ROWS, statement::ROWS, expr::ROWS, logic::ROWS];

pub fn rows() -> impl Iterator<Item = &'static OpRow> {
    TABLE.iter().flat_map(|part| part.iter())
}

/// The row for an opcode, None for one the interpreter does not accept.
pub fn row(op: u16) -> Option<&'static OpRow> {
    rows().find(|r| (r.first..=r.last).contains(&op))
}
