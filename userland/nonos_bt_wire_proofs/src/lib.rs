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

//! Proofs for the Bluetooth wire formats. Every module is test-only.

#[cfg(test)]
mod acl_tests;
#[cfg(test)]
mod addr_tests;
#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod cursor_tests;
#[cfg(test)]
mod event_tests;
#[cfg(test)]
mod l2cap_tests;
#[cfg(test)]
mod le_meta_tests;
#[cfg(test)]
mod reassembly_tests;
