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

//! Proofs for U3, the exposure map: the terminal's `audit` join and labels,
//! and the About screen's and the attest service's two holder checks. The
//! capsules' own sources are included by path, at the `crate::` paths they
//! name, so the tests run the shipping code.

extern crate alloc;

pub mod attest_live;
pub mod command;
#[path = "../../capsule_about/src/about/data/verify/held_by.rs"]
pub mod held_by;
pub mod term;

#[cfg(test)]
mod tests;
