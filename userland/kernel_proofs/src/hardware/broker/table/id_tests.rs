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

//! A device id is never handed to a second device: once a record leaves,
//! every id allocated after is above it, whichever record was newest, and
//! even when the table empties. A capsule holding a stale id can never
//! reach whatever arrives next.

use super::remove::remove;
use super::state::{next_id, TABLE};
use crate::hardware::broker::DeviceRecord;

fn record(id: u64) -> DeviceRecord {
    DeviceRecord { device_id: id, ..DeviceRecord::empty() }
}

/// What both allocators do: the next id past the highest in the table.
fn allocate(empty: u64) -> u64 {
    let mut t = TABLE.write();
    let id = next_id(t.iter().map(|r| r.device_id).max(), empty);
    t.push(record(id));
    id
}

#[test]
fn a_removed_id_is_never_allocated_again() {
    *TABLE.write() = (0..3).map(record).collect();
    let mut retired = Vec::new();
    for gone in [2u64, 1] {
        assert!(remove(gone).is_some());
        retired.push(gone);
        let fresh = allocate(0);
        assert!(!retired.contains(&fresh), "id {fresh} was retired");
        assert!(fresh > gone);
    }
    let ids: Vec<u64> = TABLE.read().iter().map(|r| r.device_id).collect();
    for id in ids {
        assert!(remove(id).is_some());
    }
    assert!(TABLE.read().is_empty());
    let after_empty = allocate(0);
    assert!(after_empty > 4, "an emptied table still starts above every retired id");
    assert!(allocate(0x1_0000_0000) > after_empty, "each later id is above the last");
    assert!(remove(9999).is_none(), "an id never in the table retires nothing");
}
