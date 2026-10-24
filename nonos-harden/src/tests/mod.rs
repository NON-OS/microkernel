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

//! The proofs, run with `cargo test`. Each states one property of the
//! verdict in its name; the mutants that each one catches are listed in
//! docs/release/nonos-harden.md.

mod cli_tests;
mod fixture;
mod flip_tests;
mod plan_tests;
mod record_shape_tests;
mod record_tests;
mod sealed_link_tests;
mod sealed_tests;
mod verdict_tests;
