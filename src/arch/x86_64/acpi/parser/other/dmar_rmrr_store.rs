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

//! The reserved memory regions DMAR reports, kept for the IOMMU.

use spin::Mutex;

use super::dmar_rmrr::{parse_rmrr, Rmrr};

const MAX_RMRRS: usize = 16;

static RMRRS: Mutex<heapless::Vec<Rmrr, MAX_RMRRS>> = Mutex::new(heapless::Vec::new());
/// RMRRs left out: malformed, or past the room kept.
static DROPPED: Mutex<usize> = Mutex::new(0);

/// Every RMRR that parsed, in table order.
pub fn rmrr_regions() -> heapless::Vec<Rmrr, MAX_RMRRS> {
    RMRRS.lock().clone()
}

/// RMRRs the kernel could not keep. Each is memory some device is still
/// using that no mapping will cover.
pub fn rmrr_dropped() -> usize {
    *DROPPED.lock()
}

pub(super) fn record_rmrr(bytes: &[u8]) {
    let kept = parse_rmrr(bytes).is_some_and(|r| RMRRS.lock().push(r).is_ok());
    if !kept {
        *DROPPED.lock() += 1;
    }
}
