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

#[path = "../../../../src/hardware/broker/confine/acs_alias.rs"]
pub mod acs_alias;
#[path = "../../../../src/hardware/broker/confine/acs_layout.rs"]
pub mod acs_layout;
#[path = "../../../../src/hardware/broker/confine/acs_path.rs"]
pub mod acs_path;
#[path = "../../../../src/hardware/broker/confine/acs_reach.rs"]
pub mod acs_reach;
#[path = "../../../../src/hardware/broker/confine/acs_refusal.rs"]
pub mod acs_refusal;
#[path = "../../../../src/hardware/broker/confine/acs_rule.rs"]
pub mod acs_rule;
#[path = "../../../../src/hardware/broker/confine/acs_tree.rs"]
pub mod acs_tree;
#[path = "../../../../src/hardware/broker/confine/acs_words.rs"]
pub mod acs_words;
#[path = "../../../../src/hardware/broker/confine/member.rs"]
mod member;
// The port-only rule, mounted once, here beside the group code it
// reads; confine_posture's proofs reach `port_only` through this path.
#[path = "../../../../src/hardware/broker/confine/port_only.rs"]
pub(crate) mod port_only;
mod alias_tests;
mod hardwired_tests;
mod laptop_tests;
mod laptops;
mod layout_tests;
mod machine;
mod port_only_tests;
mod q35_tests;
mod read_layer;
mod shared_tests;
mod symmetric_cnvi_tests;
mod symmetric_groups;
mod symmetric_ports;
mod symmetric_rng;
mod symmetric_tests;
mod symmetric_topologies;
mod symmetric_unreadable_tests;
mod tests;
mod tree_tests;
mod words_tests;
