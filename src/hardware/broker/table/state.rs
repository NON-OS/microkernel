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

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::RwLock;

use crate::hardware::broker::DeviceRecord;

pub(super) static TABLE: RwLock<Vec<DeviceRecord>> = RwLock::new(Vec::new());

/// One past the highest id a removed record had: no new record takes an id
/// below it, so an id is never handed to a second device.
static RETIRED: AtomicU64 = AtomicU64::new(0);

pub(super) fn retire(device_id: u64) {
    RETIRED.fetch_max(device_id.saturating_add(1), Ordering::AcqRel);
}

/// The next id after `max` (the highest in the table), above every retired id.
pub(super) fn next_id(max: Option<u64>, empty: u64) -> u64 {
    max.map_or(empty, |m| m + 1).max(RETIRED.load(Ordering::Acquire))
}
