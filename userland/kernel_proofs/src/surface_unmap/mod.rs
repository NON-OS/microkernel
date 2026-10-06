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
 * What an owner's munmap does to its surface, included by path.
 *
 * A surface slot was freed only when its owner exited, so a long-lived
 * owner that registered and unmapped surfaces (image_codec, one per decoded
 * image) used up the machine's 256 slots. The test below holds every
 * combination of the rule.
 */

#[path = "../../../../src/kernel_core/surface_registry/pin/unmap_rule.rs"]
pub mod unmap_rule;
mod tests;
