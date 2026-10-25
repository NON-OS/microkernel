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

//! Every record's `numa_node` set from what is known now: SRAT's Generic
//! Initiators at boot, then the root bridges the capsule publishes, a
//! platform device it adds, and a function found after boot. Domains are
//! worked out before the table is locked, so no other lock is taken under
//! it.

use super::super::{list, table};
use super::numa_node::node_byte;
use super::numa_store::device_domain;
use alloc::vec::Vec;

pub fn refresh() {
    let ids = list().into_iter().map(|r| r.device_id);
    let nodes: Vec<(u64, u8)> = ids.map(|id| (id, node_byte(device_domain(id)))).collect();
    table::set_numa_nodes(&nodes);
}
