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
 * Which of a guest's pages its supervisor may pin as a surface the
 * compositor reads in place, included by path, and what becomes of the
 * frames as the guest unmaps, reprotects and exits.
 *
 * The tests hold the rule. The model runs pins, unpins, attaches, unmaps
 * and exits in every order through the rules the kernel uses (pin_rule,
 * register_rule, unmap_rule, rights_rule and rights_attach) and shows that
 * no view ever maps a frame the allocator has free or has given to another
 * owner, that a revoked surface is never attached again, that every view is
 * read-only, and that no frame of the compositor's ever reaches a guest.
 */

mod exit_race_tests;
#[path = "../../../../src/kernel_core/surface_registry/share/answerable.rs"]
pub mod answerable;
mod model;
mod model_tests;
#[path = "../../../../src/kernel_core/surface_registry/pin/pin_rule.rs"]
pub mod pin_rule;
#[path = "../../../../src/kernel_core/surface_registry/pin/register_rule.rs"]
pub mod register_rule;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_attach.rs"]
pub mod rights_attach;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_rule.rs"]
pub mod rights_rule;
mod tests;
#[path = "../../../../src/kernel_core/surface_registry/pin/unmap_rule.rs"]
pub mod unmap_rule;
#[path = "../../../../src/kernel_core/surface_registry/pin/view_rule.rs"]
pub mod view_rule;
mod view_rule_tests;
