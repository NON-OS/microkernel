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
 * Which pages a process may register as a surface, included by path.
 *
 * Registering took any writable page, and a surface's frames are the
 * registry's to free when its owner exits while another process maps them.
 * The tests hold the rule, and a model of exits and releases shows that with
 * it each frame goes back to the allocator once.
 */

#[path = "../../../../src/kernel_core/surface_registry/pin/register_rule.rs"]
pub mod register_rule;
mod model;
mod model_tests;
mod tests;
