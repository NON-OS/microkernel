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
 * Where a tier's model is kept on this boot, for need.rs's rule: on the
 * disk of an installed NONOS, or in memory on a live boot, whose volume the
 * kernel holds in memory. The policy store's Persistent says which, as the
 * desktop and the store read it; a store that does not answer is taken as
 * live, the smaller offer, as the store takes it.
 */

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use crate::need::Room;

pub fn room() -> Room {
    match lookup().and_then(|port| get_bool(port, Field::Persistent)) {
        Some(true) => Room::Disk,
        _ => Room::Memory,
    }
}
