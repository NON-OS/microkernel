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

//! A device's isolation group: what it reaches past the IOMMU through a
//! shared device, bus or port without ACS. The real path, tree and
//! rule are included by path.

#[path = "../../../../src/hardware/broker/confine/acs_path.rs"]
pub mod acs_path;
#[path = "../../../../src/hardware/broker/confine/acs_rule.rs"]
pub mod acs_rule;
#[path = "../../../../src/hardware/broker/confine/acs_tree.rs"]
pub mod acs_tree;
mod machine;
mod tests;
mod tree_tests;
