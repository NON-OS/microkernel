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

//! net.nym's session table, the files `state/table/mod.rs` names, with the
//! table type itself in reach so each proof can hold one of its own.

#[path = "../../capsule_net_nym/src/state/table/ended.rs"]
mod ended;
#[path = "../../capsule_net_nym/src/state/table/ops.rs"]
mod ops;
#[path = "../../capsule_net_nym/src/state/table/owner.rs"]
mod owner;
#[path = "../../capsule_net_nym/src/state/table/reset.rs"]
mod reset;
#[path = "../../capsule_net_nym/src/state/table/sphinx.rs"]
mod sphinx;
#[path = "../../capsule_net_nym/src/state/table/stream.rs"]
mod stream;
#[path = "../../capsule_net_nym/src/state/table/topology_gate.rs"]
mod topology_gate;
#[path = "../../capsule_net_nym/src/state/table/types.rs"]
mod types;

pub use types::{Table, TableError, PER_OWNER, TABLE, TABLE_CAP};
