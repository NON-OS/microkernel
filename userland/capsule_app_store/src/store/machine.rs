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

//! What this machine is, for which tier cards fit it: its memory, as the
//! model fetcher reads it, and where a tier's model is kept on this boot.
//! The policy store's Persistent is false on a live boot, whose data volume
//! the kernel holds in memory, true on an installed NONOS. A store that does
//! not answer is taken as live: the smaller offer, never a tier that a live
//! boot could not hold.

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use crate::need::Room;

/* The memory the kernel counts, read as `qwen tiers` reads it. */
#[path = "../../../capsule_model_fetch/src/tiers/memory.rs"]
mod memory;

pub fn read() -> (Option<u64>, Room) {
    let room = match lookup().and_then(|port| get_bool(port, Field::Persistent)) {
        Some(true) => Room::Disk,
        _ => Room::Memory,
    };
    (memory::memory(), room)
}
