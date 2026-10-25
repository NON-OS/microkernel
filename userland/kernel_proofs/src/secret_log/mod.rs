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

/*
 * U3: no capsule that takes keys or input, or holds a secret, writes to the
 * serial console off the dev profile. The kernel's rule is included by path,
 * and every manifest in the tree is held to it.
 */

mod manifest_tests;
#[path = "../../../../src/kernel_core/process_spawn/capsule_spawn/runner/secret_log.rs"]
pub mod rule;
mod tests;
