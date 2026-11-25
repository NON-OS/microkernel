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

//! The Linux personality's launch, as far as the family seal sees it: its own
//! prove, mounted unchanged, with the store and the
//! kernel's verify standing in, so a store run's proofs can be followed into
//! the seal's measurement list (launch_seal_tests.rs).

pub mod app;
pub mod attest;
pub mod origin;
pub mod start_guest;

#[cfg(test)]
mod launch_seal_tests;
