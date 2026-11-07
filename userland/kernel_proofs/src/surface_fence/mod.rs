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

//! A surface's release fence: who waits on which slot, and who may move or
//! wait on a fence at all.

#[path = "../../../../src/kernel_core/surface_registry/fence/wait_rule.rs"]
mod wait_rule;
#[path = "../../../../src/kernel_core/surface_registry/fence/waiters_list.rs"]
mod waiters;

mod tests;
mod wait_tests;
