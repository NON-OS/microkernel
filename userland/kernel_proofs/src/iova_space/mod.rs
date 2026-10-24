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
 * Where the broker places a DMA grant in a confined capsule's device address
 * space, and how freed space is reused. The real placement, the step over
 * reserved regions and the hole-list sources are included by path.
 */

#[path = "../../../../src/hardware/broker/confine/avoid.rs"]
pub mod avoid;
#[path = "../../../../src/hardware/broker/confine/iova_holes.rs"]
pub mod holes;
mod holes_model_tests;
mod holes_tests;
#[path = "../../../../src/hardware/broker/confine/reserved_rule.rs"]
pub mod reserved_rule;
mod reserved_tests;
#[path = "../../../../src/hardware/broker/confine/iova_space.rs"]
pub mod space;
mod tests;

/// The name the hole list reaches the placement rules by in the kernel.
use space as iova_space;
