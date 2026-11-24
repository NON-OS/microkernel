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

//! Why a grant was refused. Said by number on the wire (wire.rs) and never
//! with the program, the policy or a destination.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The bytes are not a Linux app policy.
    NotAPolicy,
    /// The policy does not grant NET_ANON.
    NoNetAnon,
    /// The kernel verified no such policy for this family.
    PolicyNotVouched,
    /// The kernel proved no such program in this family.
    ProgramNotProved,
    /// The run's argv names no box with a destination.
    NoBox,
    /// The front holds as many grants as it keeps.
    Full,
    /// The sender supervises no live family, or its family ended while the
    /// grant was being decided.
    NoFamily,
}

impl Refusal {
    /// Its number on the wire, after 0 for granted.
    pub fn code(self) -> u8 {
        match self {
            Refusal::NotAPolicy => 1,
            Refusal::NoNetAnon => 2,
            Refusal::PolicyNotVouched => 3,
            Refusal::ProgramNotProved => 4,
            Refusal::NoBox => 5,
            Refusal::Full => 6,
            Refusal::NoFamily => 7,
        }
    }
}
