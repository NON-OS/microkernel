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

//! Display timings, proved on the host.
//!
//! The `#[path]` includes pull in the shipping crate at the module names
//! its `crate::` paths use. The DMT rows that came from CVT are held to
//! this crate's CVT arithmetic number for number, which checks the table
//! and the formulas against each other.

#[path = "../../nonos_modes/src/check.rs"]
pub mod check;
#[path = "../../nonos_modes/src/cvt/mod.rs"]
pub mod cvt;
#[path = "../../nonos_modes/src/dmt.rs"]
pub mod dmt;
#[path = "../../nonos_modes/src/dmt_parse.rs"]
pub mod dmt_parse;
#[path = "../../nonos_modes/src/fit.rs"]
pub mod fit;
#[path = "../../nonos_modes/src/lex.rs"]
pub mod lex;
#[path = "../../nonos_modes/src/link.rs"]
pub mod link;
#[path = "../../nonos_modes/src/make.rs"]
pub mod make;
#[path = "../../nonos_modes/src/resolve.rs"]
pub mod resolve;

#[cfg(test)]
mod tests;
